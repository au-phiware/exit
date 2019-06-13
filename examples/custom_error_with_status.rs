use exit;
use std::{error::Error, fmt, fmt::Display};

#[derive(Debug)]
enum CustomError {
    SadFace,
}

impl Error for CustomError {}

impl Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CustomError::SadFace => ":-(",
        }
        .fmt(f)
    }
}

impl From<CustomError> for i32 {
    fn from(err: CustomError) -> Self {
        match err {
            CustomError::SadFace => 2,
        }
    }
}

fn main() -> exit::Result {
    might_fail()?;

    exit::Result::Ok
}

fn might_fail() -> Result<(), CustomError> {
    return Err(CustomError::SadFace);
}
