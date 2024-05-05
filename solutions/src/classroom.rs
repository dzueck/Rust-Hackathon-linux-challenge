use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::options().read(false).write(true).open("Sally").unwrap();

    let mut message = String::new();
    file.read_to_string(&mut message).unwrap();

    println!("{message}");
}