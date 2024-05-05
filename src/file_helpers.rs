use std::{ffi::{c_int, OsString}, str::FromStr};

use crate::{background_tasks::DEFAULT_MODE, files::File, main_fs::get_unique_ino, special_files::trigger_file::TriggerFile};



pub fn read(data: &[u8], offset: i64, size: u32) -> Result<&[u8], c_int> {
    if offset.is_negative() ||  offset as usize >= data.len() {
        return Ok(&[]);
    }

    let offset = offset as usize;
    let end = offset + size as usize;

    if end >= data.len() {
        return Ok(&data[offset..]);
    } else {
        return Ok(&data[offset..end]);
    }
}

pub fn str_to_vec(str: &str) -> Vec<u8> {
    str.as_bytes().iter().map(|x| *x).collect()
}

const VICTORY_FILE_MESSAGE: &str = 
"Congradulations you have beaten this module!
";
pub fn victory_file() -> Box<dyn File> {
    text_file("Victory", VICTORY_FILE_MESSAGE)
}

pub fn nothing(){}

pub fn text_file(name: &str, text: &str) -> Box<dyn File> {
    Box::new(TriggerFile::new(nothing, &OsString::from_str(name).unwrap(), str_to_vec(text), get_unique_ino(), DEFAULT_MODE, 0))
}