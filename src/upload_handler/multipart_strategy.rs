use actix_web::{
    AsyncResponder,
    HttpMessage,
    HttpRequest,
    HttpResponse,
    error,
    error::Result as ActixResult,
    multipart,
    error::Error,
    dev::Payload,
    FutureResponse,
};
use futures::{future::err, Future, Stream};
use crate::{
    ImageType, 
    Image,
    AppState,
};
use super::{
    HandlerResult, 
    Strategy,
    SuccessResponse,
};
use bytes::Bytes;

pub struct MultipartStrategy;

impl Strategy for MultipartStrategy {
    fn handle_request(&self, req: &HttpRequest<AppState>) -> HandlerResult {
        let app_state = req.state().clone();

        req.multipart()
            .map_err(error::ErrorInternalServerError)
            .map(move |item| MultipartStrategy::process_item_with_state(item, app_state.clone()))
            .flatten()
            .collect()
            .map(MultipartStrategy::get_response)
            .responder()
    }
}

impl MultipartStrategy {
    fn process_item_with_state(item: multipart::MultipartItem<Payload>, state: AppState) -> Box<Stream<Item = String, Error = Error>> {
        match item {
            multipart::MultipartItem::Field(field) => Box::new(MultipartStrategy::process_field_with_state(field, state).into_stream()),
            multipart::MultipartItem::Nested(nested) => Box::new(
                nested.map_err(error::ErrorInternalServerError)
                .map(move |nested_item| MultipartStrategy::process_item_with_state(nested_item, state.clone()))
                .flatten()
            )
        }
    }

    fn process_field_with_state(field: multipart::Field<Payload>, state: AppState) -> FutureResponse<String> {
        let image_type = match MultipartStrategy::check_content_type(&field) {
            Ok(t) => t,
            Err(e) => return Box::new(err(e))
        };

        let future_id = field.concat2()
            .from_err::<error::Error>()
            .and_then(move |body| Ok(MultipartStrategy::save_image(&body, &image_type, &state.storage_path)?))
            .map_err(error::ErrorInternalServerError);

        Box::new(future_id)
    }

    fn check_content_type(field: &multipart::Field<Payload>) -> ActixResult<ImageType> {
        let image_type: ImageType = field.content_type().into();
        match image_type {
            ImageType::Unknown => Err(error::ErrorBadRequest("Unknown file format")),
            _ => Ok(image_type)
        }
    }

    fn save_image(bytes: &Bytes, image_type: &ImageType, storage_path: &str) -> ActixResult<String> {
        let image = Image::new(bytes, image_type, storage_path);
        image.save()?;
        Ok(image.id.to_string())
    }


    fn get_response(ids: Vec<String>) -> HttpResponse {
        let success = match serde_json::to_value(SuccessResponse { ids }) {
            Ok(s) => s,
            Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
        };
        HttpResponse::Ok().json(success)
    }
}
