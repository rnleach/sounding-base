#![deny(missing_docs)]

/*!

Library to represent an atmospheric sounding with pressure as the vertical coordinate.
The base crate is meant to be a common base  for other crates to build on. These crates may be for 
managing a data-store, displaying data, or saving and loading files.

The emphasis of this crate is data representation and a common type for systems using sounding
data to build on and use.

# Examples
```
extern crate chrono;
extern crate sounding_base;

use sounding_base::{Sounding, StationInfo, Profile, Surface};

fn main() {

    // Create  pressure profile
    let pressure_profile: Vec<Option<f64>> = 
        vec![1000.0, 925.0, 850.0, 700.0, 500.0, 300.0, 250.0, 100.0]
            .iter()
            .map(|p| Some(*p))
            .collect();

    // Create a temperature profile
    let temperature_profile: Vec<Option<f64>> = 
        vec![13.0, 7.0, 5.0, -4.5, -20.6, -44.0, -52.0, -56.5]
            .iter()
            .map(|t| Some(*t))
            .collect();

    // Create some station info
    let stn = StationInfo::new_with_values(None, (45.6789, -115.6789), 992.0);

    // Create a valid time. This uses a `chrono::NaiveDateTime`, and you should always assume 
    // that valid times are in UTC.
    let vt = chrono::NaiveDate::from_ymd(2018,3,8).and_hms(12,0,0);

    // Use the builder pattern to construct a sounding.
    let snd = Sounding::new()
        .set_station_info(stn)
        .set_valid_time(vt)
        .set_lead_time(24)  // Lead time in hours for forecast soundings.
        .set_profile(Profile::Pressure, pressure_profile)
        .set_profile(Profile::Temperature, temperature_profile)
        // Surface values don't have to be `Option`
        .set_surface_value(Surface::StationPressure, 1013.25)
        // But they can be
        .set_surface_value(Surface::Temperature, Some(15.0)); 

    // Top down and bottom up iterators are provided. If surface data is available, it is 
    // inserted into the profile.
    let mut iter = snd.top_down();

    let mut data_row = iter.next().unwrap();
    assert!(data_row.get_value(Profile::Pressure) == Some(100.0));
    assert!(data_row.get_value(Profile::Temperature) == Some(-56.5));

    data_row = iter.next().unwrap();
    assert!(data_row.get_value(Profile::Pressure) == Some(250.0));
    assert!(data_row.get_value(Profile::Temperature) == Some(-52.0));

    data_row = iter.next().unwrap();
    assert!(data_row.get_value(Profile::Pressure) == Some(300.0));
    assert!(data_row.get_value(Profile::Temperature) == Some(-44.0));

    data_row = iter.next().unwrap();
    assert!(data_row.get_value(Profile::Pressure) == Some(500.0));
    assert!(data_row.get_value(Profile::Temperature) == Some(-20.6));

    data_row = iter.next().unwrap();
    assert!(data_row.get_value(Profile::Pressure) == Some(700.0));
    assert!(data_row.get_value(Profile::Temperature) == Some(-4.5));

    data_row = iter.next().unwrap();
    assert!(data_row.get_value(Profile::Pressure) == Some(850.0));
    assert!(data_row.get_value(Profile::Temperature) == Some(5.0));

    data_row = iter.next().unwrap();
    assert!(data_row.get_value(Profile::Pressure) == Some(925.0));
    assert!(data_row.get_value(Profile::Temperature) == Some(7.0));

    data_row = iter.next().unwrap();
    assert!(data_row.get_value(Profile::Pressure) == Some(1000.0));
    assert!(data_row.get_value(Profile::Temperature) == Some(13.0));


    // THIS ONE IS THE SURFACE DATA!
    data_row = iter.next().unwrap();
    assert!(data_row.get_value(Profile::Pressure) == Some(1013.25));
    assert!(data_row.get_value(Profile::Temperature) == Some(15.0));

    assert!(iter.next() == None);

    // Profiles and surface values can also be accessed via getter methods. Read the docs!
}
```

You probably noticed a lot of `Option`s in the example. Basically, anything can be missing, and
missing values are common in upper air soundings. For example, at high altitude the dew point or 
humidity are often missing (if not totally inaccurate).

*/

//
// API
//

pub use sounding::{DataRow, Profile, RowView, Sounding, StationInfo, Surface};

//
// Internal use only
//

extern crate chrono;
extern crate smallvec;

extern crate metfor;

mod sounding;
