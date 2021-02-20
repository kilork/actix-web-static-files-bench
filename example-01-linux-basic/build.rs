use actix_web_static_files::generate_resources;
use std::env;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let generated_filename = Path::new(&out_dir).join("generated.rs");
    generate_resources("./linux-5.10.9", None, generated_filename, "generate")
        .expect("can't collect resources");
}
