// mod generated;

// use generated::generate;
use example_02_set_01::generate;

fn main() {
    let generated_files = generate();

    let size: usize = generated_files.values().map(|x| x.data.len()).sum();

    println!("count: {}, size: {}", generated_files.len(), size);
}
