#[cfg(test)] mod tests;
mod native_preview_maker;
use native_preview_maker::NativePreviewMaker;
use bytes::Bytes;
use uuid::Uuid;
use std::fs::{File, create_dir_all};
use std::io::Write;
use crate::{
    ImageUploaderResult,
    IMAGE_NAME,
    PREVIEW_NAME,
};
use failure::Fail;
use image::ImageFormat;

#[derive(Clone)]
pub struct Image {
    pub content: Bytes,
    pub image_type: ImageType,
    pub storage_path: String,
    pub id: Uuid, 
    preview_maker: Option<Box<PreviewMaker>>,
}

impl Image {
    pub fn new(bytes: &Bytes, image_type: &ImageType, storage_path: &str) -> Self {
        let mut image = Image {
            id: Uuid::new_v4(),
            storage_path: storage_path.to_string(),
            image_type: image_type.clone(),
            content: bytes.clone(),
            preview_maker: None,
        };

        let preview_maker = NativePreviewMaker::new();
        image.set_preview_maker(Box::new(preview_maker));
        image
    }

    fn set_preview_maker(&mut self, preview_maker: Box<PreviewMaker>) {
        self.preview_maker = Some(preview_maker);
    }

    pub fn save(&self) -> ImageUploaderResult<()> {
        self.save_at_path(&self.get_file_path())
    }

    pub fn generate_preview(&self) -> ImageUploaderResult<()> {
        if let Some(ref preview_maker) = self.preview_maker {
            let preview = preview_maker.make_preview_from_image(self)?;
            preview.save_at_path(&self.get_preview_file_path())
        } else {
            Err(ImageError::ErrorMakingPreview.into())
        }
    }

    fn save_at_path(&self, path: &str) -> ImageUploaderResult<()> {
        create_dir_all(&self.get_directory_path())?;
            
        let mut file = File::create(path)?;

        Ok(file.write_all(&self.content)?)
    }

    pub fn get_directory_path(&self) -> String {
        format!("{}/{}", self.storage_path, self.id) }

    pub fn get_file_path(&self) -> String {
        format!("{}/{}.{}", self.get_directory_path(), IMAGE_NAME, self.image_type.to_string())
    }

    pub fn get_preview_file_path(&self) -> String {
        format!("{}/{}.{}", self.get_directory_path(), PREVIEW_NAME, self.image_type.to_string())
    }

    pub fn guess_type_for_bytes(bytes: &Bytes) -> ImageUploaderResult<ImageType> {
        let image_format = image::guess_format(bytes)?;
        Ok(image_format.into())
    }
}

#[derive(Fail, Debug)]
pub enum ImageError {
    #[fail(display = "Can't make preview for image")]
    ErrorMakingPreview,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ImageType {
    Jpeg,
    Bmp,
    Gif,
    Png,
    Unknown,
}

impl From<mime::Mime> for ImageType {
    fn from(t: mime::Mime) -> Self {
        match t.subtype() {
            mime::BMP => ImageType::Bmp,
            mime::GIF => ImageType::Gif,
            mime::JPEG => ImageType::Jpeg,
            mime::PNG => ImageType::Png,
            _ => ImageType::Unknown,
        }
    }
}

impl From<&mime::Mime> for ImageType {
    fn from(t: &mime::Mime) -> Self {
        ImageType::from(t.clone())
    }
}

impl From<ImageFormat> for ImageType {
    fn from(image_format: ImageFormat) -> Self {
        match image_format {
            ImageFormat::BMP => ImageType::Bmp,
            ImageFormat::GIF => ImageType::Gif,
            ImageFormat::JPEG => ImageType::Jpeg,
            ImageFormat::PNG => ImageType::Png,
            _ => ImageType::Unknown,
        }
    }
}

impl ToString for ImageType {
    fn to_string(&self) -> String {
        match self {
            ImageType::Bmp => "bmp".to_string(),
            ImageType::Gif => "gif".to_string(),
            ImageType::Jpeg => "jpg".to_string(),
            ImageType::Png => "png".to_string(),
            ImageType::Unknown => String::new(),
        }
    }
}

pub trait PreviewMaker {
    fn make_preview_from_image(&self, image: &Image) -> ImageUploaderResult<Image>;
    // a "hack" to make this trait work in cloneable trait object
    fn box_clone(&self) -> Box<PreviewMaker>;
}

impl Clone for Box<PreviewMaker> {
    fn clone(&self) -> Box<PreviewMaker> {
        self.box_clone()
    }
}
