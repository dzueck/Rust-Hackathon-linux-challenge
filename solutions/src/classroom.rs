use std::fs::File;
use std::io::Read;
use std::os::unix::fs::OpenOptionsExt;

fn main() {
    let mut file = File::options().read(true).write(true).open("Sally").unwrap();

    let mut message = String::new();
    file.read_to_string(&mut message).unwrap();

    println!("{message}");
}