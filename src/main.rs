#![feature(termination_trait_lib, process_exitcode_placeholder, try_trait)]
#![feature(specialization)]
#![feature(never_type)]
#![feature(associated_type_bounds)]

use std::{
    convert::TryFrom,
    env,
    error::Error,
    fmt,
    fmt::Display,
    fs,
    ops::Try,
    process::{ExitCode, Termination},
};

struct TerminalFailure {
    status_code: i32,
    error: String,
}

impl<T: Display> From<T> for TerminalFailure {
    default fn from(t: T) -> TerminalFailure {
        TerminalFailure {
            status_code: ExitCode::FAILURE.report(),
            error: format!("{}", t),
        }
    }
}

impl<T: Into<i32> + Display> From<T> for TerminalFailure {
    default fn from(t: T) -> TerminalFailure {
        TerminalFailure {
            error: format!("{}", t),
            status_code: t.into(),
        }
    }
}

enum TerminalResult<T = TerminalFailure> {
    Success,
    Failure(T),
}

impl<T: Into<TerminalFailure>> Termination for TerminalResult<T> {
    fn report(self) -> i32 {
        match self {
            TerminalResult::Success => ExitCode::SUCCESS.report(),
            TerminalResult::Failure(fail) => {
                let fail = fail.into();
                eprintln!("error: {}", fail.error);
                fail.status_code
            }
        }
    }
}

impl<T: Into<TerminalFailure>> Try for TerminalResult<T> {
    type Ok = ();
    type Error = T;

    fn into_result(self) -> Result<Self::Ok, Self::Error> {
        match self {
            TerminalResult::Success => Ok(()),
            TerminalResult::Failure(fail) => Err(fail),
        }
    }

    fn from_ok(_: Self::Ok) -> Self {
        TerminalResult::Success
    }

    fn from_error(err: Self::Error) -> Self {
        TerminalResult::Failure(err)
    }
}

fn main() -> TerminalResult /*<Box<dyn Error>>*/ {
    let config = Config::try_from(env::args())?;

    let contents = fs::read_to_string(config.filename)?;

    println!("{}", contents);

    TerminalResult::Success
}

#[derive(Debug)]
struct Config {
    query: String,
    filename: String,
}

#[derive(Debug)]
enum ConfigError {
    TooFewArgs,
}

impl Error for ConfigError {}

impl Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConfigError::TooFewArgs => "too few arguments",
        }
        .fmt(f)
    }
}

impl From<ConfigError> for i32 {
    fn from(err: ConfigError) -> Self {
        match err {
            ConfigError::TooFewArgs => 2,
        }
    }
}

impl Termination for ConfigError {
    fn report(self) -> i32 {
        eprintln!("error: {}", self);
        self.into()
    }
}

impl From<ConfigError> for TerminalFailure {
    fn from(err: ConfigError) -> TerminalFailure {
        match err {
            ConfigError::TooFewArgs => TerminalFailure {
                status_code: 2,
                error: String::from("too few arguments"),
            },
        }
    }
}

impl TryFrom<Vec<String>> for Config {
    type Error = ConfigError;

    fn try_from(args: Vec<String>) -> Result<Self, Self::Error> {
        if args.len() < 3 {
            return Err(ConfigError::TooFewArgs);
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

impl TryFrom<env::Args> for Config {
    type Error = ConfigError;

    fn try_from(args: env::Args) -> Result<Self, Self::Error> {
        let args: Vec<String> = args.collect();
        Config::try_from(args)
    }
}
