mod image;
mod upload_handler;
mod get_image_handler;
mod app_state;
pub use image::{
    ImageType,
    Image,
};
pub use upload_handler::UploadHandler;
pub use app_state::AppState;
pub use get_image_handler::GetImageHandler;

pub(crate) const IMAGE_NAME: &str = "00";
pub(crate) const PREVIEW_NAME: &str = "01";

pub type ImageUploaderResult<T> = Result<T, failure::Error>;
