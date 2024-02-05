use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;

pub fn get_folders(folders: &mut Vec<fs::DirEntry>, path: &Path) -> io::Result<()> {
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            folders.push(entry?);
        }
    }
    Ok(())
}

pub fn create_file(root: &Path, name: String) -> io::Result<()> {
    let mut root_str = root.to_path_buf().into_os_string();
    root_str.push("/");
    root_str.push(name);
    let path = Path::new(&root_str);

    let _ = fs::File::create(path)?;

    Ok(())
}
