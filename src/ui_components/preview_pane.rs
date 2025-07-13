use crate::Rect;
use crate::PathBuf;
use std::fs;

pub struct PreviewPane {
    pub rect: Rect,
    pub file_name: String,
    pub size: String,
}

impl PreviewPane {
    pub fn new(rect: Rect, path: &PathBuf) -> Self {
        let file_name = path.file_name()
            .map(|name| name.to_string_lossy().into_owned()).unwrap_or(String::new());

        let mut preview = PreviewPane {
            rect,
            file_name,
            size: String::new(),
        };

        if let Ok(metadata) = fs::metadata(path) {
            let size = metadata.len();
            if size < 1000 {
                preview.size = format!("{} B", size);
            }
            else if size < 1_000_000 {
                preview.size = format!("{} KB", size/1000);
            }
            else if size < 1_000_000_000 {
                preview.size = format!("{} MB", size/1_000_000);
            }
            else if size < 1_000_000_000_000 {
                preview.size = format!("{} GB", size/1_000_000_000);
            }
            else {
                preview.size = format!("{} TB", size/1_000_000_000_000);
            }
        }

        preview

    }
}