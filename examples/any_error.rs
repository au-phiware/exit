use exit;
use std::{fs, io};

fn main() -> exit::Result {
    should_fail()?;

    exit::Result::Ok
}

fn should_fail() -> io::Result<()> {
    fs::read_to_string("/empty/missing")?;
    Ok(println!("Please remove /empty/missing and try again"))
}
