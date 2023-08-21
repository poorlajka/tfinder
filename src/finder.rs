use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::Path;

pub fn get_folders(folders: &mut Vec<fs::DirEntry>, path: &Path) -> io::Result<()> {
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            folders.push(entry?);
        }
    }
    Ok(())
}
