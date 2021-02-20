use hathor::FileGenerator;

use actix_web_static_files::generate_resources;
use std::path::Path;
use std::{env, ffi::OsStr};

const TARGET: &str = "generated";

fn main() {
    let flag = Path::new(&TARGET).join("flag");
    if !flag.exists() {
        std::fs::remove_dir_all(TARGET).unwrap_or_default();
        std::fs::create_dir(TARGET).unwrap();
        hathor::FileGeneratorBuilder::with_size(4096)
            .repeat(256 * 4)
            .generate_to(TARGET)
            .unwrap();
        std::fs::write(flag, "ok").unwrap();
    }

    let out_dir = env::var("OUT_DIR").unwrap();

    let generated_filename = Path::new(&out_dir).join("generated.rs");
    let generated_rs = "\
mod sets;

pub use sets::generate;";
    std::fs::write(generated_filename, generated_rs).unwrap();

    let generated_dir = Path::new(&out_dir).join("sets");
    std::fs::create_dir_all(&generated_dir).unwrap();
    let mod_filename = generated_dir.join("mod.rs");
    std::fs::write(
        mod_filename,
        "\
mod set_01;
pub use set_01::generate;",
    )
    .unwrap();
    let generated_filename = generated_dir.join("set_01.rs");

    generate_resources(
        TARGET,
        Some(|path: &Path| path.file_name() != Some(&OsStr::new("flag"))),
        generated_filename,
        "generate",
    )
    .unwrap();
}
