#![deny(missing_docs)]

/*!

Library to represent an atmospheric sounding with pressure as the vertical coordinate.
Data formats and algorithms for atmospheric soundings. The base crate is meant to be a common 
base  for other crates to build on. These crates may be for managing a data-store, displaying
data, or saving and loading files.

The emphasis of this crate is data representation and a common type for systems using sounding
data to build on and use.

*/

//
// API
//

pub use sounding::{DataRow, Profile, Sounding, StationInfo, Surface};

//
// Internal use only
//

extern crate chrono;
extern crate smallvec;

extern crate metfor;

mod sounding;
