mod direct_strategy;
mod multipart_strategy;
use actix_web::{
    HttpRequest,
    HttpResponse,
    HttpMessage,
    error,
    dev::Handler,
    error::Result as ActixResult,
    AsyncResponder,
    FutureResponse,
};
use futures::future::{result};
use direct_strategy::DirectStrategy;
use multipart_strategy::MultipartStrategy;
use crate::AppState;
use serde_derive::Serialize;

type HandlerResult = FutureResponse<HttpResponse>;

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
    fn choose_strategy(&self, req: &HttpRequest<AppState>) -> ActixResult<Box<Strategy>> {
        if req.query().contains_key("url") {
            //TODO Ok(UrlStrategy {})
            Ok(Box::new(DirectStrategy {}))
        } else {
            self.choose_content_based_strategy(req)
        }
    }

    fn choose_content_based_strategy(&self, req: &HttpRequest<AppState>) -> ActixResult<Box<Strategy>> {
        let mime = match req.mime_type()? {
            Some(t) => t,
            None => return Err(error::ErrorBadRequest("MIME not specified"))
        };

        match (mime.type_(), mime.subtype()) {
            (mime::APPLICATION, mime::JSON) => Ok(Box::new(DirectStrategy {})),
            (mime::MULTIPART, mime::FORM_DATA) => Ok(Box::new(MultipartStrategy {})),
            (mime::IMAGE, _) => Ok(Box::new(DirectStrategy {})),
            _ => Err(error::ErrorBadRequest("Unsupported MIME type")),
        }
    }
}

trait Strategy {
    fn handle_request(&self, req: &HttpRequest<AppState>) -> HandlerResult;
}

#[derive(Serialize, Debug)]
struct SuccessResponse {
    ids: Vec<String>,
}
