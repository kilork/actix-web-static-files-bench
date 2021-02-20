use actix_web::App;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

fn main() {
    let generated_file = generate();

    assert_eq!(generated_file.len(), 4);

    let app = App::new()
        .service(actix_web_static_files::ResourceFiles::new(
           "/static",
           generated_file,
       ));
}
