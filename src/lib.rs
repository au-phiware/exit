#![feature(termination_trait_lib, process_exitcode_placeholder, try_trait)]
#![feature(specialization)]
#![feature(associated_type_bounds)]

use std::{
    fmt::Display,
    ops::Try,
    process::{ExitCode, Termination},
    result,
};

pub struct Status {
    pub status_code: i32,
    pub error: String,
}

impl<T: Display> From<T> for Status {
    default fn from(t: T) -> Status {
        Status {
            status_code: ExitCode::FAILURE.report(),
            error: format!("{}", t),
        }
    }
}

impl<T: Into<i32> + Display> From<T> for Status {
    default fn from(t: T) -> Status {
        Status {
            error: format!("{}", t),
            status_code: t.into(),
        }
    }
}

pub enum Result<T = Status> {
    Ok,
    Err(T),
}

impl<T: Into<Status>> Termination for Result<T> {
    default fn report(self) -> i32 {
        match self {
            Result::Ok => ExitCode::SUCCESS.report(),
            Result::Err(err) => {
                let err = err.into();
                eprintln!("error: {}", err.error);
                err.status_code
            }
        }
    }
}

impl<T> Try for Result<T> {
    type Ok = ();
    type Error = T;

    default fn into_result(self) -> result::Result<<Self as Try>::Ok, <Self as Try>::Error> {
        match self {
            Result::Ok => Ok(()),
            Result::Err(err) => Err(err),
        }
    }

    default fn from_ok(_: <Self as Try>::Ok) -> Self {
        Result::Ok
    }

    default fn from_error(err: Self::Error) -> Self {
        Result::Err(err)
    }
}
