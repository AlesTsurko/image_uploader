#[derive(Clone, Debug)]
pub struct AppState {
    pub storage_path: String,
}

impl AppState {
    pub fn new(storage_path: String) -> Self {
        AppState { storage_path }
    }
}
