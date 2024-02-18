
use ratatui_image::{picker::Picker, protocol::StatefulProtocol};
use crate::Path;
use image::io::Reader;
use crate::Rect;
use std::ffi::OsStr;

pub enum PreviewType {
    Image(ImageType),
    Folder,
    File,
    None,
}

pub struct ImageType {
    pub image: Box<dyn StatefulProtocol>,
}

pub struct Preview {
    pub picker: Picker,
    pub preview_type: PreviewType,
    pub rect: Rect,
    pub is_rendered: bool,
}

impl Preview {

    pub fn load (&mut self, path: &Path) {
        if path.is_dir() {
            self.preview_type = PreviewType::Folder;
        }
        else {
            match path.extension().and_then(OsStr::to_str) {
                Some("jpg") | Some("jpeg") | Some("png") => {
                    let dyn_img = Reader::open(path).unwrap().decode().unwrap();
                    self.preview_type = PreviewType::Image(
                        ImageType { image: self.picker.new_resize_protocol(dyn_img)},
                    );
                }
                Some(_) => {
                    self.preview_type = PreviewType::File;
                }
                None => ()

            }
            self.is_rendered = false;
        }

    }
}
