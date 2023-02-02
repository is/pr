use std::{fs::File, io::Write};

fn main() {
    let mut file = File::create("my_file.txt").expect("unable to create file.");
    let data = "Hello World!";
    file.write_all(data.as_bytes()).expect("unable to write data");
}
