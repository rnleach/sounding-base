#![warn(missing_docs)]
#![recursion_limit = "1024"]
//! Library to represent an atmospheric sounding.

extern crate chrono;

#[macro_use]
extern crate error_chain;

pub mod error;
pub use error::*;

pub mod missing_value;
pub use missing_value::{OptionVal, MissingData};

pub mod sounding;
pub use sounding::{Sounding, DataRow, Profile, Surface, Index};
