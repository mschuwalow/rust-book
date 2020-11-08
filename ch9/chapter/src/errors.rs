use std::{
    fs::File,
    io::{Error, ErrorKind, Read},
};
fn open_file() -> File {
    match File::open("hello.text") {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.text") {
                Ok(fc) => fc,
                Err(e) => panic!(
                    "Tried to create file but there was a problem: {:?}",
                    e
                ),
            }
        }
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        }
    }
}

fn read_username_from_file() -> Result<String, Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_() -> Result<String, Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
