use std::{ffi::OsStr, path::{Component, Path, PathBuf}, str::FromStr, thread};

use crate::{dirs::Dir, files::File, main_fs::{get_unique_ino, lookup, FsData, Ino, DATA, GID, UID}, user_files::NormalDir};

pub const DEFAULT_MODE: u32 = 0o777;

pub fn add_file(path: &str, file: Box<dyn File>) {
    let path = path.to_string();
    println!("Adding-------------------");
    thread::spawn(move || {
        _add_file(&path, file);
    });
}

fn _add_file(path: &str, file: Box<dyn File>) {
    let mut data = DATA.lock().unwrap();
    let path = PathBuf::from_str(path).unwrap();
    let mut parent = 1;

    for component in path.components() {
        let Component::Normal(next) = component else {
            continue;
        };

        if let Ok(next_attr) = lookup(parent, next, &data) {
            parent = next_attr.ino;
        } else {
            parent = _add_one_dir(parent, default_dir(next), &mut data);
        }
    }
    let ino = file.attr().ino;
    data.inos.insert(ino, Ino::File(file));
    data.inos.get_mut(&parent).unwrap().unwrap_dir_mut().add_child(ino).unwrap();
}

fn _add_one_dir(parent: u64, dir: Box<dyn Dir>, data: &mut FsData) -> u64 {
    let new_ino = dir.attr().ino;
    data.inos.get_mut(&parent).unwrap().unwrap_dir_mut().add_child(new_ino).unwrap();
    data.inos.insert(new_ino, Ino::Dir(dir));
    return new_ino;
}

fn default_dir(name: &OsStr) -> Box<dyn Dir> {
    Box::new(NormalDir::new(name, false, get_unique_ino(), DEFAULT_MODE, *UID, *GID, 0))
}
// fn parent_ino(path: &Path, data: &FsData) -> Option<u64> {

    
//     Some(parent)
// }