use crate::{
    ImageUploaderResult, 
    PREVIEW_SIZE,
};
use super::{
    Image,
    ImageType,
    PreviewMaker,
};
use image::{
    DynamicImage,
    FilterType,
    ImageFormat,
    ImageOutputFormat,
    load_from_memory_with_format,
};
use failure::Fail;

#[derive(Clone)]
pub struct NativePreviewMaker;

impl PreviewMaker for NativePreviewMaker {
    fn make_preview_from_image(&self, image: &Image) -> ImageUploaderResult<Image> {
        let image_format = self.get_image_format_for_image_type(&image.image_type)?;
        let preview = self.get_preview_for_image_with_format(image, &image_format)?;
        let buf = self.dyn_image_with_format_into_buf(&preview, &image_format)?;

        Ok(Image {
            content: buf.into(),
            image_type: image.image_type.clone(),
            storage_path: image.storage_path.clone(),
            id: image.id.clone(), 
            preview_maker: None,
        })
    }

    fn box_clone(&self) -> Box<PreviewMaker> {
        Box::new(self.clone())
    }
}

impl NativePreviewMaker {
    pub fn new() -> Self {
        NativePreviewMaker {}
    }

    fn get_image_format_for_image_type(&self, image_type: &ImageType) -> ImageUploaderResult<ImageFormat> {
        match image_type.clone().into() {
            Some(f) => Ok(f),
            None => return Err(NativePreviewMakerError::UnknownImageType(image_type.to_string()).into())
        }
    }

    fn get_preview_for_image_with_format(&self, image: &Image, image_format: &ImageFormat) -> ImageUploaderResult<DynamicImage> {
        let dyn_image = load_from_memory_with_format(&image.content, *image_format)?;
        Ok(dyn_image.resize_to_fill(PREVIEW_SIZE.0, PREVIEW_SIZE.1, FilterType::Nearest))
    }

    fn dyn_image_with_format_into_buf(&self, dyn_image: &DynamicImage, image_format: &ImageFormat) -> ImageUploaderResult<Vec<u8>> {
        let mut buf: Vec<u8> = Vec::new();
        let image_output_format: ImageOutputFormat = image_format.clone().into();
        dyn_image.write_to(&mut buf, image_output_format)?;
        Ok(buf)
    }
}

#[derive(Fail, Debug)]
enum NativePreviewMakerError {
    #[fail(display = "Unknown image type: {}", _0)]
    UnknownImageType(String),
}

impl Into<Option<ImageFormat>> for ImageType {
    fn into(self) -> Option<ImageFormat> {
        match self {
            ImageType::Bmp => Some(ImageFormat::BMP),
            ImageType::Gif => Some(ImageFormat::GIF),
            ImageType::Jpeg => Some(ImageFormat::JPEG),
            ImageType::Png => Some(ImageFormat::PNG),
            ImageType::Unknown => None,
        }
    }
}
