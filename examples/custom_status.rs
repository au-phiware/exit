use exit;

#[derive(Debug)]
enum CustomStatus {
    SadFace,
}

impl From<CustomStatus> for exit::Status {
    fn from(err: CustomStatus) -> Self {
        match err {
            CustomStatus::SadFace => exit::Status {
                status_code: 2,
                error: ":-(".to_string(),
            },
        }
    }
}

fn main() -> exit::Result {
    might_fail()?;

    exit::Result::Ok
}

fn might_fail() -> Result<(), CustomStatus> {
    return Err(CustomStatus::SadFace);
}
