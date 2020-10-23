use std::fmt;

use serde::{de, ser};

pub type Result<T> = std::result::Result<T, Error>;

// This is a bare-bones implementation. A real library would provide additional
// information in its error type, for example the line and column at which the
// error occurred, the byte offset into the input, or the current key being
// processed.
#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    // One or more variants that can be created by data structures through the
    // `ser::Error` and `de::Error` traits. For example the Serialize impl for
    // Mutex<T> might return an error because the mutex is poisoned, or the
    // Deserialize impl for a struct may return an error because a required
    // field is missing.
    Message(String),

    // Zero or more variants that can be created directly by the Serializer and
    // Deserializer without going through `ser::Error` and `de::Error`. These
    // are specific to the format, in this case zzz.
    Eof,
    Syntax,
    ExpectedBoolean,
    ExpectedF32,
    ExpectedI32,
    ExpectedString,
    ExpectedNode,
    ExpectedNull,

    // Potentially non-applicable errors that should probably be removed in the future, but are
    // needed for now.
    ExpectedArray,
    ExpectedArrayComma,
    ExpectedArrayEnd,
    ExpectedEnum,
    ExpectedMap,
    ExpectedMapComma,
    ExpectedMapColon,
    ExpectedMapEnd,
    TrailingCharacters,
}

impl ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Message(msg) => formatter.write_str(msg),
            Error::Eof => formatter.write_str("unexpected end of input"),
            Error::ExpectedBoolean => formatter.write_str("expected boolean"),
            Error::ExpectedF32 => formatter.write_str("expected boolean"),
            Error::ExpectedI32 => formatter.write_str("expected boolean"),
            Error::ExpectedString => formatter.write_str("expected boolean"),
            Error::ExpectedNode => formatter.write_str("expected boolean"),
            Error::ExpectedNull => formatter.write_str("expected boolean"),
            /* and so forth */
            _ => formatter.write_str("I dunno."),
        }
    }
}

impl std::error::Error for Error {}
