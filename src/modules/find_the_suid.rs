use std::process::Command;

use crate::{background_tasks::{add_file, DEFAULT_MODE}, file_helpers::text_file, main_fs::get_unique_ino, special_files::trigger_file::TriggerFile, MOUNT_POINT};

const BASE_PATH: &str = "Where's_Waldo";
const FILE_NAME: &str = "Waldo";
const FAKE_MESSAGE: &str = 
"Hi I'm Waldo.
";

const REAL_MESSAGE: &str =
"No I am the real Waldo.
";

const NUM_WALDOS_PART_1: usize = 60;
const NUM_WALDOS_PART_2: usize = 33;

pub fn start() {
    add_file(BASE_PATH, Box::new(TriggerFile::new(found_trigger, FILE_NAME, REAL_MESSAGE.as_bytes().iter().map(|x| *x).collect(), get_unique_ino(), DEFAULT_MODE | libc::S_ISUID, 0)));
    let file_path = format!("{MOUNT_POINT}/{BASE_PATH}/{FILE_NAME}");
    Command::new("chmod").arg("+s").arg("arg").output().expect("Failed to run command");

    for _ in 0..NUM_WALDOS_PART_2 {
        add_file(BASE_PATH, text_file(FILE_NAME, FAKE_MESSAGE));
    }

    for _ in 0..NUM_WALDOS_PART_1 {
        add_file(BASE_PATH, text_file(FILE_NAME, FAKE_MESSAGE));
    }
}

fn found_trigger() {

}