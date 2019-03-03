mod image;
mod upload_handler;
mod app_state;
pub use image::{
    ImageType,
    RawImage,
};
pub use upload_handler::UploadHandler;
pub use app_state::AppState;

pub type ImageUploaderResult<T> = Result<T, failure::Error>;
