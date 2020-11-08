extern crate num_bigint as bigint;
use bigint::BigUint;
use std::{error, fmt, io, io::Write, num::ParseIntError};
#[derive(Debug)]
enum ReadError {
    Os(io::Error),
    Parse(ParseIntError),
}
impl fmt::Display for ReadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ReadError::Os(ref e) => e.fmt(f),
            ReadError::Parse(ref e) => e.fmt(f),
        }
    }
}
impl error::Error for ReadError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            ReadError::Os(ref e) => Some(e),
            ReadError::Parse(ref e) => Some(e),
        }
    }
}
impl From<io::Error> for ReadError {
    fn from(err: io::Error) -> Self {
        ReadError::Os(err)
    }
}
impl From<ParseIntError> for ReadError {
    fn from(err: ParseIntError) -> Self {
        ReadError::Parse(err)
    }
}
fn read_num() -> Result<u32, ReadError> {
    print!("Please enter a number: ");
    io::stdout().flush()?;
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let num = buffer.trim().parse()?;
    Ok(num)
}
pub fn fibonacci(n: u32) -> BigUint {
    fn go(n: u32, last: BigUint, penultimate: BigUint) -> BigUint {
        match n {
            0 => penultimate,
            1 => last,
            _ => {
                let new = &last + penultimate;
                go(n - 1, new, last)
            }
        }
    }
    go(n, BigUint::from(1_u8), BigUint::from(0_u8))
}
fn main() {
    match read_num() {
        Ok(number) => {
            let result = fibonacci(number);
            println!("Fibonacci number {} is: {}", number, result);
        }
        Err(_) => println!("Failed to read input!"),
    }
}
