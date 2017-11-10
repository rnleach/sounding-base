#![warn(missing_docs)]
//! Library to represent an atmospheric sounding.

extern crate chrono;

pub mod missing_value;
pub use missing_value::{OptionVal, MissingData};

pub mod sounding;
pub use sounding::{Sounding, DataRow, Profile, Surface, Index};
