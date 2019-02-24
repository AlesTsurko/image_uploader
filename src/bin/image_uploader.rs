use actix_web::{
    App,
    AsyncResponder,
    Error,
    HttpMessage,
    HttpRequest,
    HttpResponse,
    error,
    http,
    middleware, 
    server, 
};
use futures::{Future, Stream};
use serde_json::json;
use std::fs::File;
use std::io::Write;

fn upload_file(req: &HttpRequest) -> Box<Future<Item = HttpResponse, Error = Error>> {
    let mime_type = req.mime_type();

    req.payload()
        .concat2()
        .from_err()
        .and_then(|bytes| {
            let image_type: ImageType = match mime_type? {
                Some(mime_type) => mime_type.into(),
                None => return Err(error::ErrorBadRequest("mime type should be specified"))
            };

            if image_type == ImageType::Unknown {
                return Err(error::ErrorBadRequest("unsupported image format"));
            }

            let mut file = File::create(&format!("image.{}", image_type.to_string()))
                .map_err(|e| {
                    let message = format!("{:?}", e);
                    error::ErrorInternalServerError(message)
                })?;

            file.write_all(&bytes)
                .map_err(|e| {
                    let message = format!("{:?}", e);
                    error::ErrorInternalServerError(message)
                })?;

            let js = json!({
                "status": "ok"
            });

            Ok(HttpResponse::Ok().json(js))
        })
    .responder()
}

fn main() {
    std::env::set_var("RUST_LOG", "actix_web=DEBUG");
    env_logger::init();

    server::new(move || {
        App::new()
            .middleware(middleware::Logger::new("\"%r\" %s %b %Dms"))
            .resource("/upload", |r| r.method(http::Method::PUT).f(upload_file))
    }).bind("127.0.0.1:8000")
    .expect("Unable to start the server")
        .run();
}

#[derive(PartialEq, Eq)]
enum ImageType {
    Jpeg,
    Bmp,
    Gif,
    Png,
    Unknown,
}

impl From<mime::Mime> for ImageType {
    fn from(t: mime::Mime) -> Self {
        match t.subtype().as_str() {
            "bmp" => ImageType::Bmp,
            "gif" => ImageType::Gif,
            "jpeg" => ImageType::Jpeg,
            "png" => ImageType::Png,
            _ => ImageType::Unknown,
        }
    }
}

impl ToString for ImageType {
    fn to_string(&self) -> String {
        match self {
            ImageType::Bmp => "bmp".to_string(),
            ImageType::Gif => "gif".to_string(),
            ImageType::Jpeg => "jpg".to_string(),
            ImageType::Png => "png".to_string(),
            ImageType::Unknown => String::new(),
        }
    }
}
