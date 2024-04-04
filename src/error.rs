use std::io;

#[derive(Debug)]
pub enum PestError {
    IOError(io::Error),
    Unreachable(String),
    ParseFail(String),
    FormatFail(String),
}

pub type PestResult<T> = Result<T, PestError>;

impl From<io::Error> for PestError {
    fn from(e: io::Error) -> Self {
        PestError::IOError(e)
    }
}

impl From<&str> for PestError {
    fn from(s: &str) -> Self {
        PestError::ParseFail(String::from(s))
    }
}

impl std::error::Error for PestError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            PestError::IOError(e) => Some(e),
            _ => None,
        }
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }
}

impl std::fmt::Display for PestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PestError::IOError(e) => f.write_fmt(format_args!("{}", e)),
            PestError::Unreachable(s) => f.write_str(s.as_str()),
            PestError::ParseFail(s) => f.write_str(s.as_str()),
            PestError::FormatFail(s) => f.write_str(s.as_str()),
        }
    }
}

#[macro_export]
macro_rules! unreachable_rule {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        format!("Unreachable Rule: {} at line {}", &name[..name.len() - 3], line!())
    }};
}

#[macro_export]
macro_rules! debug_cases {
    ($i:ident) => {{
        println!("Rule::{:?}=>continue,", $i.as_rule());
        println!("Span: {:?}", $i.as_span());
        println!("Text: {}", $i.as_str());
        unreachable!();
    }};
}
