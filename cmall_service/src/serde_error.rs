
use std::{self, fmt};
use std::fmt::{ Display};

use serde::{de, ser};
use tracing::info;

pub type Result<T> = std::result::Result<T, Error>;

// This is a bare-bones implementation. A real library would provide additional
// information in its error type, for example the line and column at which the
// error occurred, the byte offset into the input, or the current key being
// processed.
#[derive(Debug)]
pub enum Error {
    // 通过 `ser::Error` 和 `de::Error` traits 可以由数据结构创建的一个或多个变体。例如，对于 Mutex<T> 的 Serialize 实现可能会返回一个错误，因为 mutex 被 poisoned，或者对于结构体的 Deserialize 实现可能会因为一个必需的字段丢失而返回错误。
    Message(String),

    // 通过 Serializer 和 Deserializer 直接创建的一个或多个变体，无需经过`ser::Error` 和 `de::Error`。这些特定于格式，在这种情况下是 JSON。
    Eof,
    // Syntax,
    // ExpectedBoolean,
    // ExpectedInteger,
    // ExpectedString,
    // ExpectedNull,
    // ExpectedArray,
    // ExpectedArrayComma,
    // ExpectedArrayEnd,
    // ExpectedMap,
    // ExpectedMapColon,
    // ExpectedMapComma,
    // ExpectedMapEnd,
    // ExpectedEnum,
    // TrailingCharacters,
}

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Message(msg) => formatter.write_str(msg),
            Error::Eof =>{
                info!("error");
                formatter.write_str("unexpected end of input")
            },
            /* and so forth */
        }
    }
}

impl std::error::Error for Error {}

