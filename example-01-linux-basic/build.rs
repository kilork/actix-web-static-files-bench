use static_files::resource_dir;

fn main() -> std::io::Result<()> {
    change_detection::ChangeDetection::path("linux-5.10.9")
        .path("build.rs")
        .generate();
    resource_dir("linux-5.10.9").build()
}
