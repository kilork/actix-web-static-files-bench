use actix_service::Service;
use actix_web::App;
use mem_helper::MemoryStats;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    actix_web::HttpServer::new(|| {
        let generated_file = generate();
        App::new()
            .wrap_fn(|req, srv| {
                println!("Memory stats: {}. You requested: {}", MemoryStats::current(), req.path());
                let fut = srv.call(req);
                async {
                    let res = fut.await?;
                    Ok(res)
                }
            })
            .service(actix_web_static_files::ResourceFiles::new(
                "/static",
                generated_file,
            ))
    })
    .bind("localhost:8080")?
    .run()
    .await
}
