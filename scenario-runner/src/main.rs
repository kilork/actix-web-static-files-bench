use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use async_recursion::async_recursion;
use futures::TryStreamExt;
use log::{error, info, trace, LevelFilter};
use structopt::StructOpt;
use tokio::{
    fs::{read_dir, DirEntry},
    spawn,
    sync::{
        mpsc::{self, Sender},
        Mutex, Semaphore,
    },
    time::Instant,
};
use tokio_stream::wrappers::ReceiverStream;

/// The scenario runner.
#[derive(Debug, StructOpt)]
struct Cli {
    /// The path to the directory to be processed.
    src_dir: PathBuf,
    /// The number of threads to use for processing.
    threads: Option<usize>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::builder().filter_level(LevelFilter::Info).init();

    let cli = Cli::from_args();

    let mut join_handles = Vec::new();

    let threads = cli.threads.unwrap_or_else(num_cpus::get);
    info!("Using {} threads", threads);
    let semaphore = Arc::new(Semaphore::new(threads));

    let (tx, rx): (Sender<std::io::Result<DirEntry>>, _) = mpsc::channel(1);
    let mut rx = ReceiverStream::new(rx);

    let cli_src_dir = cli.src_dir.as_path();

    spawn(walk_dir(cli_src_dir.to_path_buf(), tx));

    let count = Arc::new(Mutex::new(0usize));
    let client = reqwest::Client::new();
    let start_time = Instant::now();
    while let Some(dir) = rx.try_next().await? {
        let permit = semaphore.clone().acquire_owned().await?;

        let path = dir.path();
        let cnt = count.clone();
        if let Ok(path) = path.strip_prefix(cli_src_dir).map(Path::to_string_lossy) {
            let url = "http://localhost:8080/static/".to_string() + &path;
            let client_copy = client.clone();
            join_handles.push(spawn(async move {
                *cnt.lock().await += 1;

                match client_copy.get(&url).send().await {
                    Ok(response) => {
                        let status = response.status();
                        match response.text().await {
                            Ok(body) => {
                                trace!("{}: {} : {:?}", url, status, body.get(0..10));
                            }
                            Err(err) => {
                                error!("Error: {:?}", err);
                            }
                        }
                    }
                    Err(err) => {
                        error!("Error: {:?}", err);
                    }
                }
                drop(permit);
            }));
        }
    }

    for handle in join_handles {
        handle.await?;
    }

    let elapsed = Instant::now().duration_since(start_time).as_millis();
    let count = *count.lock().await;
    info!(
        "Done. Requests count: {}. Elapsed (ms): {}. Requests per ms: {}",
        count,
        elapsed,
        count as f64 / elapsed as f64,
    );

    Ok(())
}

#[async_recursion]
async fn walk_dir(
    dir: PathBuf,
    tx: Sender<std::io::Result<DirEntry>>,
) -> Result<(), anyhow::Error> {
    let mut dir_info = read_dir(dir).await?;

    while let Some(dir) = dir_info.next_entry().await? {
        let path = dir.path();
        if path.is_dir() {
            walk_dir(path, tx.clone()).await?;
        } else {
            tx.send(Ok(dir)).await?;
        }
    }
    Ok(())
}
