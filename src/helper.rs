use std::path;
use std::io;
use std::fs;
use crate::generate::Path;

pub fn copy_assets_to_output_dir(assets_path: &str, output_dir_path: &str) {
    if path::Path::new(&assets_path).is_dir() {
       let _ = copy_files_in_dir_to_dst(assets_path, output_dir_path);
    }
}

pub fn copy_files_in_dir_to_dst(src_dir: &str, dst_dir: &str) -> io::Result<()> {
    fs::create_dir_all(dst_dir)?;

    for entry in fs::read_dir(&src_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let file_name = entry.file_name();
            let dest_path = path::Path::new(&dst_dir).join(file_name);
            fs::copy(&path, &dest_path)?;
        } else if path.is_dir() {
            println!("{}", path.to_str().unwrap());
            let _dst_dir = path::Path::new(&dst_dir).join(entry.file_name());
            fs::create_dir_all(&_dst_dir)?;
            let _ = copy_files_in_dir_to_dst(&path.to_str().unwrap(), _dst_dir.to_str().unwrap());
        }
    }

    Ok(())
}

pub fn check_is_root(directory_path: Path) -> bool {
    return path::Path::new(&(directory_path + "jet.toml")).is_file();
}

pub fn read_file_content(filepath: Path) -> String {
    let contents = match fs::read_to_string(filepath) {
        Ok(contents) => {
            contents
        }
        Err(_e) => {
           panic!("File doesn't exist.") 
        }
    };

    return contents;
}
