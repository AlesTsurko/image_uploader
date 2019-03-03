#[derive(PartialEq, Eq)]
pub enum ImageType {
    Jpeg,
    Bmp,
    Gif,
    Png,
    Unknown,
}

impl From<mime::Mime> for ImageType {
    fn from(t: mime::Mime) -> Self {
        match t.subtype().as_str() {
            "bmp" => ImageType::Bmp,
            "gif" => ImageType::Gif,
            "jpeg" => ImageType::Jpeg,
            "png" => ImageType::Png,
            _ => ImageType::Unknown,
        }
    }
}

impl From<&mime::Mime> for ImageType {
    fn from(t: &mime::Mime) -> Self {
        match t.subtype().as_str() {
            "bmp" => ImageType::Bmp,
            "gif" => ImageType::Gif,
            "jpeg" => ImageType::Jpeg,
            "png" => ImageType::Png,
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
