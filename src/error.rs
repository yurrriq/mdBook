use std::error;
use std::fmt;
use std::io;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct ErrorList {
    errors: Vec<BookError>,
}

#[derive(Debug, Clone)]
pub struct BookError {
    pub kind: ErrorKind,
    pub message: String,
    pub details: String,
}

#[derive(Debug, Clone)]
pub enum ErrorKind {
    FileNotFound,
    PermissionError,
    ParsingError,
    Unkown,
}

// -----------------------------------------------------------------------------------
// Implementations for ErrorList
// -----------------------------------------------------------------------------------

impl ErrorList {

    /// Creates a new ErrorList struct
    pub fn new() -> Self {
        ErrorList { errors: vec![] }
    }

    /// Creates a new ErrorList and adds the given error to the list
    pub fn from(error: BookError) -> Self {
        ErrorList { errors: vec![error] }
    }

    pub fn join(&mut self, error_list: ErrorList) {
        self.errors.extend_from_slice(&error_list.errors)
    }

    pub fn add(&mut self, error: BookError) {
        self.errors.push(error)
    }

    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn len(&self) -> usize {
        self.errors.len()
    }
}


impl IntoIterator for ErrorList {
    type Item = BookError;
    type IntoIter = ::std::vec::IntoIter<BookError>;

    fn into_iter(self) -> Self::IntoIter {
        self.errors.into_iter()
    }
}


// -----------------------------------------------------------------------------------
// Implementations for BookError
// -----------------------------------------------------------------------------------

impl BookError {
    pub fn new(message: &str, details: &str, kind: ErrorKind) -> Self {
        BookError {
            message: message.to_owned(),
            details: details.to_owned(),
            kind: kind,
        }
    }

    pub fn set_details(mut self, details: &str) -> Self {
        self.details = details.to_owned();
        self
    }

    pub fn set_kind(mut self, kind: ErrorKind) -> Self {
        self.kind = kind;
        self
    }

    pub fn add_details(&mut self, details: &str) {
        self.details.push_str("\n");
        self.details.push_str(details);
    }
}

impl fmt::Display for BookError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl error::Error for BookError {
    fn description(&self) -> &str {
        &self.message
    }

    fn cause(&self) -> Option<&error::Error> {
        // FIXME: It compiles but it is certainly not what it is supposed to do
        Some(self)
    }
}



impl<'a> From<&'a str> for BookError {
    fn from(s: &str) -> Self {
        BookError {
            message: s.to_owned(),
            details: String::new(),
            kind: ErrorKind::Unkown,
        }
    }
}

impl From<io::Error> for BookError {
    fn from(io_error: io::Error) -> Self {
        let mut error = BookError::from(io_error.description());
        let mut cause_error: &Error = &io_error;

        while let Some(cause) = cause_error.cause() {
            error.add_details(cause.description());
            cause_error = cause;
        }

        error
    }
}

impl From<Box<Error>> for BookError {
    fn from(std_error: Box<Error>) -> Self {
        let mut error = BookError::from(std_error.description());
        let mut cause_error: &Error = &*std_error;

        while let Some(cause) = cause_error.cause() {
            error.add_details(cause.description());
            cause_error = cause;
        }

        error
    }
}
