use clap::{
    App as ClapApp, 
    Arg,
};
use actix_web::{
    App,
    http,
    middleware, 
    server, 
};
use image_uploader::{
    UploadHandler, 
    AppState,
};

fn main() {
    let matches = ClapApp::new("Image Uploader")
        .version("0.1.0")
        .author("Ales Tsurko")
        .about("Image uploader server demo.")
        .arg(Arg::with_name("bind_to")
             .short("b")
             .long("bind_to")
             .value_name("ADDRESS")
             .help("127.0.0.1:8000 for ex.")
             .required(true)
             .takes_value(true))
        .arg(Arg::with_name("storage_path")
             .short("s")
             .long("storage_path")
             .value_name("PATH")
             .help("Specify path where data will be stored. If not specified default path (./storage) will be used.")
             .required(false)
             .takes_value(true))
        .get_matches();

    std::env::set_var("RUST_LOG", "actix_web=DEBUG");
    env_logger::init();

    let bind_to = matches.value_of("bind_to").unwrap();
    let storage_path = matches.value_of("storage_path")
        .unwrap_or("storage")
        .to_string();
    let app_state = AppState::new(storage_path);

    server::new(move || {
        App::with_state(app_state.clone())
            .middleware(middleware::Logger::new("\"%r\" %s %b %Dms"))
            .resource("/upload", |r| r.method(http::Method::PUT).h(UploadHandler))
    }).bind(&bind_to)
    .expect("Unable to start the server")
        .run();
}
