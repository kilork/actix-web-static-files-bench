use hathor::FileGenerator;

use static_files::sets::{generate_resources_sets, SplitByCount};
use std::path::Path;
use std::{env, ffi::OsStr};

const TARGET: &str = "generated";

fn main() {
    let flag = Path::new(&TARGET).join("flag");
    if !flag.exists() {
        std::fs::remove_dir_all(TARGET).unwrap_or_default();
        std::fs::create_dir(TARGET).unwrap();
        hathor::FileGeneratorBuilder::with_size(4096)
            .repeat(256 * 128 * 8)
            .generate_to(TARGET)
            .unwrap();
        std::fs::write(flag, "ok").unwrap();
    }

    let out_dir = env::var("OUT_DIR").unwrap();

    let generated_filename = Path::new(&out_dir).join("generated.rs");

    generate_resources_sets(
        TARGET,
        Some(|path: &Path| path.file_name() != Some(&OsStr::new("flag"))),
        generated_filename,
        "sets",
        "generate",
        &mut SplitByCount::new(1024),
    )
    .unwrap();

    change_detection::ChangeDetection::path("generated")
        .path("build.rs")
        .generate();
}
