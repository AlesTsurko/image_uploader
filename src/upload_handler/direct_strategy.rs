use actix_web::{
    AsyncResponder,
    HttpMessage,
    HttpRequest,
    HttpResponse,
    error,
    error::Result as ActixResult,
};
use futures::{Future, Stream};
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
    pub fn respond_for_body_with_mime_type(app_state: AppState, body: &Bytes, mime_type: &Option<mime::Mime>) -> ActixResult<HttpResponse> {
        let image_type = DirectStrategy::get_image_type_from_mime_type(&mime_type)?;
        DirectStrategy::check_image_type(&image_type)?;

        let image = Image::new(body, &image_type, &app_state.storage_path);
        image.save()?;
        image.generate_preview()?;

        Ok(SuccessResponse { ids: vec![image.id.to_string()] }.into())
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

}
