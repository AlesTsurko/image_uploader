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
use crate::ImageType;
use super::{
    HandlerResult, 
    Strategy,
    std_error_into_internal_server,
};
use bytes::Bytes;

pub struct DirectStrategy;

impl Strategy for DirectStrategy {
    fn handle_request<S>(&self, req: &HttpRequest<S>) -> HandlerResult {
        let mime_type = req.mime_type();

        req.payload()
            .concat2()
            .from_err()
            .and_then(|body| DirectStrategy::respond_for_body_with_mime_type(&body, &mime_type?))
            .responder()
    }
}

impl DirectStrategy {
    fn respond_for_body_with_mime_type(body: &Bytes, mime_type: &Option<mime::Mime>) -> ActixResult<HttpResponse> {
        let image_type = DirectStrategy::get_image_type_from_mime_type(&mime_type)?;
        DirectStrategy::check_image_type(&image_type)?;

        DirectStrategy::write_image_from_content_with_type(body, &image_type)?;

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

    fn write_image_from_content_with_type(content: &Bytes, image_type: &ImageType) -> ActixResult<()> {
        create_dir_all("storage")
            .map_err(std_error_into_internal_server)?;

        let mut file = File::create(&format!("storage/image.{}", image_type.to_string()))
            .map_err(std_error_into_internal_server)?;

        file.write_all(&content)
            .map_err(std_error_into_internal_server)
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
