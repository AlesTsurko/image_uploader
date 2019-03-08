use actix_web::{
    AsyncResponder,
    HttpMessage,
    HttpRequest,
    HttpResponse,
    error,
    error::Result as ActixResult,
};
use futures::Future;
use crate::{
    ImageType, 
    Image,
    AppState,
    MAX_JSON_PAYLOAD_SIZE,
};
use super::{
    HandlerResult, 
    Strategy,
    SuccessResponse,
};
use bytes::Bytes;
use base64;
use serde_derive::Deserialize;

pub struct Base64Strategy;

impl Strategy for Base64Strategy {
    fn handle_request(&self, req: &HttpRequest<AppState>) -> HandlerResult {
        let app_state = req.state().clone();

        req.json()
            .limit(MAX_JSON_PAYLOAD_SIZE)
            .from_err()
            .and_then(|json_req: ExpectedJsonRequest| Base64Strategy::process_json_request_with_state(json_req, app_state))
            .responder()
    }
}

impl Base64Strategy {
    fn process_json_request_with_state(request: ExpectedJsonRequest, state: AppState) -> ActixResult<HttpResponse> {
        let mut ids: Vec<String> = Vec::new();

        for encoded in request.data.iter() {
            let image = Base64Strategy::base64_into_image(&encoded, &state)?;
            image.save()?;
            image.generate_preview()?;
            ids.push(image.id.to_string());
        }

        Ok(SuccessResponse { ids }.into())
    }

    fn base64_into_image(data: &str, state: &AppState) -> ActixResult<Image> {
        let bytes: Bytes = base64::decode(data)
            .map_err(error::ErrorInternalServerError)?
            .into();
        let image_type = Base64Strategy::check_image_type(&bytes)?;
        Ok(Image::new(&bytes, &image_type, &state.storage_path))
    }

    fn check_image_type(bytes: &Bytes) -> ActixResult<ImageType> {
        let image_type = Image::guess_type_for_bytes(&bytes)?;
        if image_type == ImageType::Unknown {
            return Err(error::ErrorBadRequest("Unknown file format"));
        }
        Ok(image_type)
    }

}

#[derive(Deserialize, Debug)]
struct ExpectedJsonRequest {
    data: Vec<String>,
}
