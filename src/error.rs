use std::{fmt::Display, io::Read, num::TryFromIntError};
use serde::{de::value::Error, ser::Impossible};
#[derive(Debug)]
pub enum BPErr {
    NotImplementable,
    NotSpecified,

    NonAsciiChar,

    IoErr(std::io::Error),
    Custom(String),

    NotBool,
    NotOption,
    NotUnit,
    Utf8Err(std::str::Utf8Error),
    IntCastErr(TryFromIntError)
}

impl From<std::io::Error> for BPErr {
    fn from(value: std::io::Error) -> Self {
        Self::IoErr(value)
    }
}

impl From<std::str::Utf8Error> for BPErr {
    fn from(value: std::str::Utf8Error) -> Self {
        Self::Utf8Err(value)
    }
}

impl From<TryFromIntError> for BPErr {
    fn from(value: TryFromIntError) -> Self {
        Self::IntCastErr(value)
    }
}

impl std::fmt::Display for BPErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl std::error::Error for BPErr {}

impl serde::ser::Error for BPErr {
    fn custom<T>(msg:T) -> Self where T:Display {
        Self::Custom(msg.to_string())
    }
}

impl serde::de::Error for BPErr {
fn custom<T>(msg:T) -> Self where T:Display {
        Self::Custom(msg.to_string())
    }
}