use actix_web::{
    AsyncResponder,
    HttpMessage,
    HttpRequest,
    client,
    client::ClientResponse,
    error,
    error::Result as ActixResult,
    http::StatusCode,
};
use futures::{future, Future};
use crate::AppState;
use super::{
    HandlerResult, 
    Strategy,
    direct_strategy::DirectStrategy,
};

pub struct UrlStrategy;

impl Strategy for UrlStrategy {
    fn handle_request(&self, req: &HttpRequest<AppState>) -> HandlerResult {
        let state = req.state().clone();
        if let Some(url) = req.query().get("url") {
            UrlStrategy::perform_download_and_respond(&url, state)
        } else {
            Box::new(future::err(error::ErrorBadRequest("Unknown file format")))
        }
    }
}

impl UrlStrategy {
    fn perform_download_and_respond(url: &str, state: AppState) -> HandlerResult {
        let request = match client::get(url).finish() {
            Ok(r) => r,
            Err(e) => return Box::new(future::err(error::ErrorInternalServerError(format!("Can't construct request {}", e))))
        };

        request.send()
            .from_err()
            .and_then(|response| UrlStrategy::process_client_response_with_state(response, state))
            .responder()
    }

    fn process_client_response_with_state(response: ClientResponse, state: AppState) -> HandlerResult {
        if let Err(e) = UrlStrategy::check_status_of_response(&response) {
            return Box::new(future::err(e))
        }

        let mime_type = response.mime_type();

        response.body()
            .from_err()
            .and_then(|bytes| DirectStrategy::respond_for_body_with_mime_type(state, &bytes, &mime_type?))
            .responder()
    }

    fn check_status_of_response(response: &ClientResponse) -> ActixResult<()> {
        let status = response.status();
        if status != StatusCode::OK {
            let err_message = format!("Can't get the image: server responses with status code {}", status);
            return Err(error::ErrorInternalServerError(err_message));
        }
        Ok(())
    }

}
