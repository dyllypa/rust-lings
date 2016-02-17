// This is a bigger error exercise than the previous ones!
//
// Edit the `read_and_validate` function so that it compiles and
// passes the tests... so many things could go wrong!
//
// - Reading from stdin could produce an io::Error
// - Parsing the input could produce a num::ParseIntError
// - Validating the input could produce a CreationError (defined below)
//
// How can we lump these errors into one general error? That is, what
// type goes where the question marks are, and how do we return
// that type from the body of read_and_validate?
//
// Scroll down for hints :)

use std::error;
use std::fmt;
use std::io;

// PositiveNonzeroInteger is a struct defined below the tests.
fn read_and_validate(b: &mut io::BufRead) -> Result<PositiveNonzeroInteger, ???> {
    let mut line = String::new();
    b.read_line(&mut line);
    let num: i64 = line.trim().parse();
    PositiveNonzeroInteger::new(num)
}

// This is a test helper function that turns a &str into a BufReader.
fn test_with_str(s: &str) -> Result<PositiveNonzeroInteger, Box<error::Error>> {
    let mut b = io::BufReader::new(s.as_bytes());
    read_and_validate(&mut b)
}

#[test]
fn test_success() {
    let x = test_with_str("42\n");
    assert_eq!(PositiveNonzeroInteger(42), x.unwrap());
}

#[test]
fn test_not_num() {
    let x = test_with_str("eleven billion\n");
    assert!(x.is_err());
}

#[test]
fn test_non_positive() {
    let x = test_with_str("-40\n");
    assert!(x.is_err());
}

#[test]
fn test_ioerror() {
    struct Broken;
    impl io::Read for Broken {
        fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
            Err(io::Error::new(io::ErrorKind::BrokenPipe, "uh-oh!"))
        }
    }
    let mut b = io::BufReader::new(Broken);
    assert!(read_and_validate(&mut b).is_err());
}

#[derive(PartialEq,Debug)]
struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        if value == 0 {
            Err(CreationError::Zero)
        } else if value < 0 {
            Err(CreationError::Negative)
        } else {
            Ok(PositiveNonzeroInteger(value as u64))
        }
    }
}

#[test]
fn test_positive_nonzero_integer_creation() {
    assert!(PositiveNonzeroInteger::new(10).is_ok());
    assert_eq!(Err(CreationError::Negative), PositiveNonzeroInteger::new(-10));
    assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
}

#[derive(PartialEq,Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str((self as &error::Error).description())
    }
}

impl error::Error for CreationError {
    fn description(&self) -> &str {
        match *self {
            CreationError::Negative => "Negative",
            CreationError::Zero => "Zero",
        }
    }
}

// First hint: To figure out what type should go where the ??? is, take a look
// at the test helper function `test_with_str`, since it returns whatever
// `read_and_validate` returns and`test_with_str` has its signature fully
// specified.

// Next hint: anywhere in `read_and_validate` that we call a function that
// returns a `Result`, wrap that call in a `try!` macro call. Use the compiler
// error messages and warnings to guide you to all the places you need to do
// this. You might need to rewrap some `try!` return values in a `Result::Ok`!

// This works because under the hood, the `try!` macro calls `From::from`
// on the error value to convert it to a boxed trait object, a Box<error::Error>,
// which is polymorphic.
