[![Build Status](https://travis-ci.org/rnleach/sounding-base.svg?branch=master)](https://travis-ci.org/rnleach/sounding-base)
[![Build status](https://ci.appveyor.com/api/projects/status/3rodnk8johi24r0t/branch/master?svg=true)](https://ci.appveyor.com/project/rnleach/sounding-base/branch/master)
[![Latest Version](https://img.shields.io/crates/v/sounding-base.svg)](https://crates.io/crates/sounding-base)
[![docs](https://docs.rs/sounding-base/badge.svg)](https://docs.rs/sounding-base)

This crate is deprecated as a whole and the functionality has been moved into the [sounding-analysis](https://github.com/rnleach/sounding-analysis) crate. The reasoning for the move was that the two crates are almost never used separately, and a change in this crate that required a version bump would then require a bump in the version of all the crates that depend on it. It became quite a chore to maintain related crates.

# sounding-base

## Related crates
* [sounding-analysis](https://github.com/rnleach/sounding-analysis)
* [sounding-bufkit](https://github.com/rnleach/sounding-bufkit)
* [metfor](https://github.com/rnleach/metfor)
* [sonde](https://github.com/rnleach/sonde)


Library to represent an atmospheric sounding with pressure as the vertical coordinate.
The base crate is meant to be a common base for other crates to build on. These crates may be for
managing a data-store, displaying data, or saving and loading files.

The emphasis of this crate is data representation and a common type for systems using sounding
data to build on and use.

## Examples
```rust
use optional::{Optioned, some};
use metfor::{HectoPascal, Celsius, Feet};

use sounding_base::{Sounding, StationInfo};

fn main() {

    // Create  pressure profile
    let pressure_profile: Vec<Optioned<HectoPascal>> =
        vec![1000.0, 925.0, 850.0, 700.0, 500.0, 300.0, 250.0, 100.0]
            .into_iter()
            .map(HectoPascal)
            .map(some)
            .collect();

    // Create a temperature profile
    let temperature_profile: Vec<Optioned<Celsius>> =
        vec![13.0, 7.0, 5.0, -4.5, -20.6, -44.0, -52.0, -56.5]
            .into_iter()
            .map(Celsius)
            .map(some)
            .collect();

    // Create some station info
    let stn = StationInfo::new_with_values(None, (45.6789, -115.6789), Feet(992.0));

    // Create a valid time. This uses a `chrono::NaiveDateTime`, and you should always assume
    // that valid times are in UTC.
    let vt = chrono::NaiveDate::from_ymd(2018,3,8).and_hms(12,0,0);

    // Use the builder pattern to construct a sounding.
    let snd = Sounding::new()
        .with_station_info(stn)
        .with_valid_time(vt)
        .with_lead_time(24)  // Lead time in hours for forecast soundings.
        .with_pressure_profile(pressure_profile)
        .with_temperature_profile(temperature_profile)
        .with_station_pressure(some(HectoPascal(1013.25)))
        .with_sfc_temperature(some(Celsius(15.0)));

    // Top down and bottom up iterators are provided. If surface data is available, it is
    // inserted into the profile.
    let mut iter = snd.top_down();

    let mut data_row = iter.next().unwrap();
    assert_eq!(data_row.pressure, some(HectoPascal(100.0)));
    assert_eq!(data_row.temperature, some(Celsius(-56.5)));

    data_row = iter.next().unwrap();
    assert_eq!(data_row.pressure, some(HectoPascal(250.0)));
    assert_eq!(data_row.temperature, some(Celsius(-52.0)));

    data_row = iter.next().unwrap();
    assert_eq!(data_row.pressure, some(HectoPascal(300.0)));
    assert_eq!(data_row.temperature, some(Celsius(-44.0)));

    data_row = iter.next().unwrap();
    assert_eq!(data_row.pressure, some(HectoPascal(500.0)));
    assert_eq!(data_row.temperature, some(Celsius(-20.6)));

    data_row = iter.next().unwrap();
    assert_eq!(data_row.pressure, some(HectoPascal(700.0)));
    assert_eq!(data_row.temperature, some(Celsius(-4.5)));

    data_row = iter.next().unwrap();
    assert_eq!(data_row.pressure, some(HectoPascal(850.0)));
    assert_eq!(data_row.temperature, some(Celsius(5.0)));

    data_row = iter.next().unwrap();
    assert_eq!(data_row.pressure, some(HectoPascal(925.0)));
    assert_eq!(data_row.temperature, some(Celsius(7.0)));

    data_row = iter.next().unwrap();
    assert_eq!(data_row.pressure, some(HectoPascal(1000.0)));
    assert_eq!(data_row.temperature, some(Celsius(13.0)));

    // THIS ONE IS THE SURFACE DATA!
    data_row = iter.next().unwrap();
    assert_eq!(data_row.pressure, some(HectoPascal(1013.25)));
    assert_eq!(data_row.temperature, some(Celsius(15.0)));

    assert_eq!(iter.next(), None);

    // Profiles and surface values can also be accessed via getter methods. Read the docs!
}
```

You probably noticed a lot of `optional::Optioned`s in the example. Basically, anything can be
missing, and missing values are common in upper air soundings. For example, at high altitude the
dew point or humidity are often missing (if not totally inaccurate).

