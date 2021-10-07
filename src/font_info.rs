use std::path::PathBuf;

#[derive(Debug)]
pub struct FontInfo {
    pub id: String,
    pub family_name: String,
    pub full_name: String,
    pub variation_name: String,
    pub file_path: Option<PathBuf>,
}

impl FontInfo {
    pub fn new(id: String, family_name: String, full_name: String, variation_name: String) -> Self {
        FontInfo {
            id,
            family_name,
            full_name,
            variation_name,
            file_path: None,
        }
    }
}
