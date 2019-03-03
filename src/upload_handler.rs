mod direct_strategy;
use actix_web::{
    Error,
    HttpRequest,
    HttpResponse,
    HttpMessage,
    error,
    dev::Handler,
    error::Result as ActixResult,
    AsyncResponder,
};
use futures::future::{Future, result};
use direct_strategy::DirectStrategy;
use crate::AppState;

type HandlerResult = Box<Future<Item = HttpResponse, Error = Error>>;

pub struct UploadHandler;

impl Handler<AppState> for UploadHandler {
    type Result = HandlerResult;

    fn handle(&self, req: &HttpRequest<AppState>) -> Self::Result {
        let strategy = match self.choose_strategy(req) {
            Ok(s) => s,
            Err(e) => return result(Err(e)).responder(),
        };
        strategy.handle_request(req)
    }
}

impl UploadHandler {
    fn choose_strategy(&self, req: &HttpRequest<AppState>) -> ActixResult<impl Strategy> {
        let mime = match req.mime_type()? {
            Some(t) => t,
            None => return Err(error::ErrorBadRequest("MIME not specified"))
        };

        match (mime.type_(), mime.subtype()) {
            (mime::APPLICATION, mime::JSON) => Ok(DirectStrategy {}),
            (mime::MULTIPART, mime::FORM_DATA) => Ok(DirectStrategy {}),
            (mime::IMAGE, _) => Ok(DirectStrategy {}),
            _ => Err(error::ErrorBadRequest("Unsupported MIME type")),
        }
    }
}

trait Strategy {
    fn handle_request(&self, req: &HttpRequest<AppState>) -> HandlerResult;
}

// fn std_error_into_internal_server(e: std::io::Error) -> Error {
    // let message = format!("{:?}", e);
    // error::ErrorInternalServerError(message)
// }
