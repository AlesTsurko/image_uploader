use actix_web::{
    HttpRequest,
    error,
    dev::Handler,
    Result as ActixResult,
    fs::NamedFile,
};
use crate::{
    AppState,
    ImageUploaderResult, 
    IMAGE_NAME,
    PREVIEW_NAME,
};
use std::path::PathBuf;
use std::fs::{read_dir, DirEntry};
use failure::Fail;

pub struct GetImageHandler;

impl Handler<AppState> for GetImageHandler {
    type Result = ActixResult<NamedFile>;

    fn handle(&self, req: &HttpRequest<AppState>) -> Self::Result {
        let directory_path = self.get_directory_path(req)?;
        let is_preview = req.query().contains_key("preview");

        let item = if is_preview { (1, PREVIEW_NAME) } else { (0, IMAGE_NAME) };
        if let Some(Ok(dir_entry)) = read_dir(directory_path)?.nth(item.0) {
            self.check_image_name(&dir_entry, item.1)?;
            return Ok(NamedFile::open(dir_entry.path())?)
        }

        Err(error::ErrorNotFound("Image not found"))
    }
}

impl GetImageHandler {
    fn get_directory_path(&self, req: &HttpRequest<AppState>) -> ImageUploaderResult<PathBuf> {
        let id: String = req.match_info().query("id")?;
        let storage_path = &req.state().storage_path;
        let mut path = PathBuf::from(storage_path);
        path.push(id);
        Ok(path)
    }

    fn check_image_name(&self, dir_entry: &DirEntry, name: &str) -> ImageUploaderResult<()> {
        let image_name = name.to_string();
        let file_name = self.get_file_stem_from_path(&dir_entry.path())?;
        if image_name == file_name {
            Ok(())
        } else {
            Err(GetImageError::CheckingImageName(image_name).into())
        }
    }

    fn get_file_stem_from_path(&self, path: &PathBuf) -> ImageUploaderResult<String> {
        let file_stem_result = match path.file_stem() {
            Some(s) => s,
            None => return Err(GetImageError::GettingFileName(format!("{:?}", path)).into())
        };

        match file_stem_result.to_str() {
            Some(s) => Ok(s.to_string()),
            None => Err(GetImageError::GettingFileName(format!("{:?}", path)).into())
        }
    }
}

#[derive(Debug, Fail)]
enum GetImageError {
    #[fail(display = "image with name {} doesn't exist", _0)]
    CheckingImageName(String),
    #[fail(display = "can't get file name for path {}", _0)]
    GettingFileName(String),
}
