use std::{
    fs::File,
    io::{self, Read},
};

fn main() {
    let open_file: io::Result<File> = File::open("hello.txt");

    let mut file: File = match open_file {
        Ok(file) => file,
        Err(e) => panic!("Failed to Open file: {e}"),
    };

    let mut user_file: String = String::new();

    match file.read_to_string(&mut user_file) {
        Ok(val) => val,
        Err(e) => panic!("Error to Read file {e}"),
    };
    println!("{}", user_file);
}
