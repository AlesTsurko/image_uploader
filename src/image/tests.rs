use super::*;

const UUID: &'static str = "936da01f-9abd-4d9d-80c7-02af85c822a8";

#[test]
fn get_directory_path() {
    let image = init_image();
    assert_eq!(format!("storage/{}", UUID), image.get_directory_path());
}

#[test]
fn get_file_path() {
    let image = init_image();
    assert_eq!(format!("storage/{}/{}.jpg", UUID, IMAGE_NAME), image.get_file_path());
}

#[test]
fn get_preview_file_path() {
    let image = init_image();
    assert_eq!(format!("storage/{}/{}.jpg", UUID, PREVIEW_NAME), image.get_preview_file_path());
}

#[test]
fn image_type_to_string() {
    assert_eq!("bmp".to_string(), ImageType::Bmp.to_string());
    assert_eq!("gif".to_string(), ImageType::Gif.to_string());
    assert_eq!("jpg".to_string(), ImageType::Jpeg.to_string());
    assert_eq!("png".to_string(), ImageType::Png.to_string());
}

#[test]
fn image_type_from_mime() {
    assert_eq!(ImageType::Bmp, ImageType::from(mime::IMAGE_BMP));
    assert_eq!(ImageType::Gif, ImageType::from(mime::IMAGE_GIF));
    assert_eq!(ImageType::Jpeg, ImageType::from(mime::IMAGE_JPEG));
    assert_eq!(ImageType::Png, ImageType::from(mime::IMAGE_PNG));
}

fn init_image() -> Image {
    let content = Bytes::from(&b"This is a test"[..]);
    let id = Uuid::parse_str(UUID).unwrap();
    Image {
        content,
        image_type: ImageType::Jpeg,
        storage_path: "storage".to_string(),
        id, 
        preview_maker: None,
    }
}
