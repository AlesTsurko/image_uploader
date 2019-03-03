use actix_web::{
    AsyncResponder,
    HttpMessage,
    HttpRequest,
    HttpResponse,
    error,
    error::Result as ActixResult,
};
use futures::{Future, Stream};
use serde_json::{
    json,
    Value as JsonValue,
};
use std::fs::{File, create_dir_all};
use std::io::Write;
use crate::{
    ImageType, 
    AppState,
};
use super::{
    HandlerResult, 
    Strategy,
    std_error_into_internal_server,
};
use bytes::Bytes;

pub struct DirectStrategy;

impl Strategy for DirectStrategy {
    fn handle_request(&self, req: &HttpRequest<AppState>) -> HandlerResult {
        let mime_type = req.mime_type();
        let app_state = req.state().clone();

        req.payload()
            .concat2()
            .from_err()
            .and_then(|body| DirectStrategy::respond_for_body_with_mime_type(app_state, &body, &mime_type?))
            .responder()
    }
}

impl DirectStrategy {
    fn respond_for_body_with_mime_type(app_state: AppState, body: &Bytes, mime_type: &Option<mime::Mime>) -> ActixResult<HttpResponse> {
        let image_type = DirectStrategy::get_image_type_from_mime_type(&mime_type)?;
        DirectStrategy::check_image_type(&image_type)?;

        DirectStrategy::write_image_from_content_with_type(body, &image_type, &app_state.storage_path)?;

        let response = DirectStrategy::prepare_succesful_response();

        Ok(HttpResponse::Ok().json(response))
    }

    fn get_image_type_from_mime_type(mime_type: &Option<mime::Mime>) -> ActixResult<ImageType> {
        match mime_type {
            Some(mime_type) => Ok(mime_type.into()),
            None => Err(error::ErrorBadRequest("mime type isn't specified"))
        }
    }

    fn check_image_type(image_type: &ImageType) -> ActixResult<()> {
        if *image_type == ImageType::Unknown {
            return Err(error::ErrorBadRequest("unsupported image format"));
        }
        Ok(())
    }

    fn write_image_from_content_with_type(content: &Bytes, image_type: &ImageType, storage_path: &str) -> ActixResult<()> {
        create_dir_all(storage_path)
            .map_err(std_error_into_internal_server)?;

        let file_path = DirectStrategy::get_image_path_for_storage_path_and_image_type(image_type, storage_path)?;
        let mut file = File::create(&file_path)
            .map_err(std_error_into_internal_server)?;

        file.write_all(&content)
            .map_err(std_error_into_internal_server)
    }

    fn get_image_path_for_storage_path_and_image_type(image_type: &ImageType, storage_path: &str) -> ActixResult<String> {
        //TODO generate id
        let id = "image";
        Ok(format!("{}/{}.{}", storage_path, id, image_type.to_string()))
    }

    fn prepare_succesful_response() -> JsonValue {
        json!({
            //ids
            //preview_urls
            //formats
            "status": "ok"
        })
    }

}
