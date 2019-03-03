mod direct_strategy;
use actix_web::{
    Error,
    HttpRequest,
    HttpResponse,
    error,
    dev::Handler,
};
use futures::Future;
use direct_strategy::DirectStrategy;

type HandlerResult = Box<Future<Item = HttpResponse, Error = Error>>;

pub struct UploadHandler;

impl<S> Handler<S> for UploadHandler {
    type Result = HandlerResult;

    fn handle(&self, req: &HttpRequest<S>) -> Self::Result {
        let strategy = self.choose_strategy(req);
        strategy.handle_request(req)
    }
}

impl UploadHandler {
    fn choose_strategy<S>(&self, req: &HttpRequest<S>) -> impl Strategy {
        DirectStrategy {}
    }
}

trait Strategy {
    fn handle_request<S>(&self, req: &HttpRequest<S>) -> HandlerResult;
}

fn std_error_into_internal_server(e: std::io::Error) -> Error {
    let message = format!("{:?}", e);
    error::ErrorInternalServerError(message)
}
