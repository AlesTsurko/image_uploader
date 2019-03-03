use bytes::Bytes;
use uuid::Uuid;
use std::fs::{File, create_dir_all};
use std::io::Write;
use crate::ImageUploaderResult;

#[derive(Debug, Clone)]
pub struct RawImage {
    pub content: Bytes,
    pub image_type: ImageType,
    pub storage_path: String,
    pub id: Uuid, 
}

impl RawImage {
    pub fn new(bytes: &Bytes, image_type: &ImageType, storage_path: &str) -> Self {
        RawImage {
            id: Uuid::new_v4(),
            storage_path: storage_path.to_string(),
            image_type: image_type.clone(),
            content: bytes.clone(),
        }
    }

    pub fn save(&self) -> ImageUploaderResult<()> {
        create_dir_all(&self.get_directory_path())?;
            
        let mut file = File::create(&self.get_file_path())?;

        Ok(file.write_all(&self.content)?)
    }

    pub fn get_directory_path(&self) -> String {
        format!("{}/{}", self.storage_path, self.id)
    }

    pub fn get_file_path(&self) -> String {
        format!("{}/image.{}", self.get_directory_path(), self.image_type.to_string())
    }
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
        match t.subtype() {
            mime::BMP => ImageType::Bmp,
            mime::GIF => ImageType::Gif,
            mime::JPEG => ImageType::Jpeg,
            mime::PNG => ImageType::Png,
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
