use std::fs::File;

fn main() {
    let mut files = Vec::new();
    for _ in 0..10 {
        files.push(File::open("./Heavy_Door"));
    }

    println!("{}", std::fs::read_to_string("./Heavy_Door").unwrap());
}
