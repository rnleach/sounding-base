#![warn(missing_docs)]
//! Library to represent an atmospheric sounding.

#![recursion_limit = "1024"]
extern crate chrono;

#[macro_use]
extern crate error_chain;

pub mod error;
pub use error::*;

pub mod missing_value;
pub use missing_value::{OptionVal, MissingData};

pub mod sounding;
pub use sounding::{Sounding, DataRow};
