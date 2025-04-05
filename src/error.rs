use std::{fmt::Display, io::Read};
use serde::{de::value::Error, ser::Impossible};
#[derive(Debug)]
pub enum BPErr {
    NotImplementable,
    NotSpecified,
    IoErr(std::io::Error),
    Custom(String)
}

impl From<std::io::Error> for BPErr {
    fn from(value: std::io::Error) -> Self {
        Self::IoErr(value)
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