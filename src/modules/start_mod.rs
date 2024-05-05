use std::{ffi::OsString, str::FromStr};

use crate::{background_tasks::{add_file, DEFAULT_MODE}, file_helpers::str_to_vec, main_fs::get_unique_ino, special_files::trigger_file::TriggerFile};

use super::{classroom, many_open};



const WELCOME_MESSAGE: &str = 
"Hello and welcome to the linux challenge.
You may be wondering where the challenge is.
Well you are actually just blind so check again.
";

const WELCOME_MESSAGE_2: &str = 
"Huh that was weird.
You should probably get your eyes checked.
Anyways welcome to the challenge.
This is your main hub directory.
You can navagate using standard linux commands.
Each directory is a different challenge and is contained in that directory.
You are free to do them in any order.
You may have to code a custom program (in Rust if you want) to complete some challenges.
Remember that you may have to think outside the box to solve the challenge.
Things are not always as they seem.
";

pub fn start() {
    add_file("", Box::new(TriggerFile::new(spawn_welcome_2, &OsString::from_str("Welcome").unwrap(), str_to_vec(WELCOME_MESSAGE), get_unique_ino(), DEFAULT_MODE, 0)));

    start_mods();
}

fn spawn_welcome_2() {
    add_file("", Box::new(TriggerFile::new(start_mods, &OsString::from_str("Welcome?").unwrap(), str_to_vec(WELCOME_MESSAGE_2), get_unique_ino(), DEFAULT_MODE, 0)));
}

fn start_mods() {
    many_open::start();
    classroom::start();
}
