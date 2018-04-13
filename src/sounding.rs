//! Data type and methods to store an atmospheric sounding.
use std::fmt;

use chrono::NaiveDateTime;

/// The names of the profiles which may be stored in a sounding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Profile {
    /// Pressure in hPa
    Pressure,
    /// Temperature in C
    Temperature,
    /// Wet bulb temperature in C
    WetBulb,
    /// Dew point in C
    DewPoint,
    /// Equivalent potential temperature in Kelvin
    ThetaE,
    /// Wind direction (from) in degrees.
    WindDirection,
    /// Wind speed in knots
    WindSpeed,
    /// Pressure vertical velocity in Pa/sec
    PressureVerticalVelocity,
    /// Geopotential Height in meters
    GeopotentialHeight,
    /// Cloud fraction in percent
    CloudFraction,
}

impl fmt::Display for Profile {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use Profile::*;
        let string_rep = match *self {
            Pressure => "pressure",
            Temperature => "temperature",
            WetBulb => "wet bulb temperature",
            DewPoint => "dew point temperature",
            ThetaE => "equivalent potential temperature",
            WindDirection => "wind direction",
            WindSpeed => "wind speed",
            PressureVerticalVelocity => "vertical velocity",
            GeopotentialHeight => "height",
            CloudFraction => "cloud fraction",
        };

        write!(f, "{}", string_rep)
    }
}

/// Surface based values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Surface {
    /// Surface pressure reduce to mean sea level (hPa)
    MSLP,
    /// Surface pressure (hPa)
    StationPressure,
    /// Low cloud fraction
    LowCloud,
    /// Mid cloud fraction
    MidCloud,
    /// Hi cloud fraction
    HighCloud,
    /// Wind Direction in degrees. This is the direction the wind is coming from.
    WindDirection,
    /// Wind speed in knots.
    WindSpeed,
    /// 2 meter temperatures (C)
    Temperature,
    /// 2 meter dew point (C)
    DewPoint,
    /// Precipitation (in)
    Precipitation,
}

impl fmt::Display for Surface {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use Surface::*;
        let string_rep = match *self {
            MSLP => "sea level pressure",
            StationPressure => "station pressure",
            LowCloud => "low cloud fraction",
            MidCloud => "mid cloud fraction",
            HighCloud => "high cloud fraction",
            WindDirection => "wind direction",
            WindSpeed => "wind speed",
            Temperature => "2-meter temperature",
            DewPoint => "2-meter dew point",
            Precipitation => "precipitation (liquid equivalent)",
        };

        write!(f, "{}", string_rep)
    }
}

/// Station information including location data and identification number.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct StationInfo {
    /// station number, USAF number, eg 727730
    num: Option<i32>,
    /// Latitude and longitude.
    location: Option<(f64, f64)>,
    /// Elevation in meters, this may be in model terrain, not necessarily the same as
    /// the real world.
    elevation: Option<f64>,
}

impl StationInfo {
    /// Create a new `StationInfo` object.
    pub fn new_with_values<T, U, V>(station_num: T, location: U, elevation: V) -> Self
    where
        T: Into<Option<i32>>,
        U: Into<Option<(f64, f64)>>,
        V: Into<Option<f64>>,
    {
        StationInfo {
            num: station_num.into(),
            location: location.into(),
            elevation: elevation.into(),
        }
    }

    /// Create a new object with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Builder method to add a station number.
    pub fn with_station<T>(mut self, number: T) -> Self
    where
        Option<i32>: From<T>,
    {
        self.num = Option::from(number);

        self
    }

    /// Builder method to add a location.
    pub fn with_lat_lon<T>(mut self, coords: T) -> Self
    where
        Option<(f64, f64)>: From<T>,
    {
        self.location = Option::from(coords);
        self
    }

    /// Builder method to add elevation.
    pub fn with_elevation<T>(mut self, elev: T) -> Self
    where
        Option<f64>: From<T>,
    {
        self.elevation = Option::from(elev);
        self
    }

    /// station number, USAF number, eg 727730
    pub fn station_num(&self) -> Option<i32> {
        self.num
    }

    /// Latitude and longitude.
    pub fn location(&self) -> Option<(f64, f64)> {
        self.location
    }

    /// Elevation in meters, this may be in model terrain, not necessarily the same as
    /// the real world.
    pub fn elevation(&self) -> Option<f64> {
        self.elevation
    }
}

/// A copy of a row of the sounding data.
#[derive(Clone, Default, Copy, Debug, PartialEq)]
pub struct DataRow {
    /// Pressure in hPa
    pub pressure: Option<f64>,
    /// Temperature in C
    pub temperature: Option<f64>,
    /// Wet bulb temperature in C
    pub wet_bulb: Option<f64>,
    /// Dew point in C
    pub dew_point: Option<f64>,
    /// Equivalent potential temperature in Kelvin
    pub theta_e: Option<f64>,
    /// Wind direction (from) in degrees.
    pub direction: Option<f64>,
    /// Wind speed in knots
    pub speed: Option<f64>,
    /// Pressure vertical velocity in Pa/sec
    pub omega: Option<f64>,
    /// Geopotential Height in meters
    pub height: Option<f64>,
    /// Cloud fraction in percent
    pub cloud_fraction: Option<f64>,
}

/// All the variables stored in the sounding.
///
/// The upper air profile variables are stored in parallel vectors. If a profile lacks a certain
/// variable, e.g. cloud fraction, that whole vector has length 0 instead of being full of missing
/// values.
///
#[derive(Debug, Default)]
pub struct Sounding {
    /// Station info
    station: StationInfo,

    /// Valid time of sounding
    valid_time: Option<NaiveDateTime>,
    /// Difference in model initialization time and `valid_time` in hours.
    lead_time: Option<i32>,

    // Upper air profile
    /// Pressure (hPa) profile
    pressure: Vec<Option<f64>>,
    /// Temperature (c) profile
    temperature: Vec<Option<f64>>,
    /// Wet-bulb (c) profile
    wet_bulb: Vec<Option<f64>>,
    /// Dew Point (C) profile
    dew_point: Vec<Option<f64>>,
    /// Equivalent Potential Temperature (K) profile
    theta_e: Vec<Option<f64>>,
    /// Wind direction (degrees) profile
    direction: Vec<Option<f64>>,
    /// Wind speed (knots) profile
    speed: Vec<Option<f64>>,
    /// Vertical velocity (Pa/sec), pressure vertical coordinate
    omega: Vec<Option<f64>>,
    /// Geopotential Height (m) profile
    height: Vec<Option<f64>>,
    /// Cloud coverage fraction in percent
    cloud_fraction: Vec<Option<f64>>,

    // Surface data
    /// Surface pressure reduce to mean sea level (hPa)
    mslp: Option<f64>,
    /// Surface pressure (hPa)
    station_pres: Option<f64>,
    /// Low cloud fraction
    low_cloud: Option<f64>,
    /// Mid cloud fraction
    mid_cloud: Option<f64>,
    /// Hi cloud fraction
    hi_cloud: Option<f64>,
    /// Wind direction
    wind_dir: Option<f64>,
    /// Wind speed in knots
    wind_spd: Option<f64>,
    /// 2 meter  temperature
    sfc_temperature: Option<f64>,
    /// 2 meter dew point
    sfc_dew_point: Option<f64>,
    /// Precipitation in mm
    precip: Option<f64>,
}

impl Sounding {
    /// Create a new sounding with default values. This is a proxy for default with a clearer name.
    #[inline]
    pub fn new() -> Self {
        Sounding::default()
    }

    /// Set the station info.
    #[inline]
    pub fn set_station_info(mut self, new_value: StationInfo) -> Self {
        self.station = new_value;
        self
    }

    /// Get the station info
    #[inline]
    pub fn get_station_info(&self) -> StationInfo {
        self.station
    }

    /// Set a profile variable
    #[inline]
    pub fn set_profile(mut self, var: Profile, mut values: Vec<Option<f64>>) -> Self {
        use self::Profile::*;

        let sfc_val = match var {
            Pressure => self.station_pres,
            Temperature => self.sfc_temperature,
            WetBulb => self.station_pres.and_then(|p| {
                self.sfc_temperature.and_then(|t| {
                    self.sfc_dew_point
                        .and_then(|dp| ::metfor::wet_bulb_c(t, dp, p).ok())
                })
            }),
            DewPoint => self.sfc_dew_point,
            ThetaE => self.station_pres.and_then(|p| {
                self.sfc_temperature.and_then(|t| {
                    self.sfc_dew_point
                        .and_then(|dp| ::metfor::theta_e_kelvin(t, dp, p).ok())
                })
            }),
            WindDirection => self.wind_dir,
            WindSpeed => self.wind_spd,
            PressureVerticalVelocity => Some(0.0),
            GeopotentialHeight => self.station.elevation,
            CloudFraction => None,
        };

        if !values.is_empty() {
            values.insert(0, sfc_val);
        }

        match var {
            Pressure => self.pressure = values,
            Temperature => self.temperature = values,
            WetBulb => self.wet_bulb = values,
            DewPoint => self.dew_point = values,
            ThetaE => self.theta_e = values,
            WindDirection => self.direction = values,
            WindSpeed => self.speed = values,
            PressureVerticalVelocity => self.omega = values,
            GeopotentialHeight => self.height = values,
            CloudFraction => self.cloud_fraction = values,
        }

        self
    }

    /// Get a profile variable as a slice
    #[inline]
    pub fn get_profile(&self, var: Profile) -> &[Option<f64>] {
        use self::Profile::*;
        match var {
            Pressure => &self.pressure,
            Temperature => &self.temperature,
            WetBulb => &self.wet_bulb,
            DewPoint => &self.dew_point,
            ThetaE => &self.theta_e,
            WindDirection => &self.direction,
            WindSpeed => &self.speed,
            PressureVerticalVelocity => &self.omega,
            GeopotentialHeight => &self.height,
            CloudFraction => &self.cloud_fraction,
        }
    }

    /// Set a surface variable
    #[inline]
    pub fn set_surface_value<T>(mut self, var: Surface, value: T) -> Self
    where
        Option<f64>: From<T>,
    {
        let value = Option::from(value);

        use self::Surface::*;
        match var {
            MSLP => self.mslp = value,
            StationPressure => self.station_pres = value,
            LowCloud => self.low_cloud = value,
            MidCloud => self.mid_cloud = value,
            HighCloud => self.hi_cloud = value,
            WindDirection => self.wind_dir = value,
            WindSpeed => self.wind_spd = value,
            Temperature => self.sfc_temperature = value,
            DewPoint => self.sfc_dew_point = value,
            Precipitation => self.precip = value,
        };

        // Set the first element of some of the profiles if necessary.
        {
            if let Some(profile) = match var {
                StationPressure => Some(&mut self.pressure),
                Temperature => Some(&mut self.temperature),
                DewPoint => Some(&mut self.dew_point),
                WindDirection => Some(&mut self.direction),
                WindSpeed => Some(&mut self.speed),
                _ => None,
            } {
                if profile.len() > 0 {
                    profile[0] = value;
                }
            }

            if var == StationPressure || var == Temperature || var == DewPoint {
                if !self.wet_bulb.is_empty() {
                    self.wet_bulb[0] = self.station_pres.and_then(|p| {
                        self.sfc_temperature.and_then(|t| {
                            self.sfc_dew_point
                                .and_then(|dp| ::metfor::wet_bulb_c(t, dp, p).ok())
                        })
                    });
                }

                if !self.theta_e.is_empty() {
                    self.theta_e[0] = self.station_pres.and_then(|p| {
                        self.sfc_temperature.and_then(|t| {
                            self.sfc_dew_point
                                .and_then(|dp| ::metfor::theta_e_kelvin(t, dp, p).ok())
                        })
                    });
                }
            }
        }

        self
    }

    /// Get a surface variable
    #[inline]
    pub fn get_surface_value(&self, var: Surface) -> Option<f64> {
        use self::Surface::*;
        match var {
            MSLP => self.mslp,
            StationPressure => self.station_pres,
            LowCloud => self.low_cloud,
            MidCloud => self.mid_cloud,
            HighCloud => self.hi_cloud,
            WindDirection => self.wind_dir,
            WindSpeed => self.wind_spd,
            Temperature => self.sfc_temperature,
            DewPoint => self.sfc_dew_point,
            Precipitation => self.precip.map(|pp| pp * 25.4), // convert from mm to inches.
        }
    }

    /// Difference in model initialization time and `valid_time` in hours.
    #[inline]
    pub fn set_lead_time<T>(mut self, lt: T) -> Self
    where
        Option<i32>: From<T>,
    {
        self.lead_time = Option::from(lt);
        self
    }

    /// Difference in model initialization time and `valid_time` in hours.
    #[inline]
    pub fn get_lead_time(&self) -> Option<i32> {
        self.lead_time
    }

    /// Valid time of the sounding
    #[inline]
    pub fn get_valid_time(&self) -> Option<NaiveDateTime> {
        self.valid_time
    }

    /// Builder method to set the valid time of the sounding
    #[inline]
    pub fn set_valid_time<T>(mut self, valid_time: T) -> Self
    where
        Option<NaiveDateTime>: From<T>,
    {
        self.valid_time = Option::from(valid_time);
        self
    }

    /// Get a bottom up iterator over the data rows. The first value returned from the iterator is
    /// surface values.
    #[inline]
    pub fn bottom_up(&self) -> ProfileIterator {
        ProfileIterator {
            next_idx: 0,
            direction: 1,
            src: self,
        }
    }

    /// Get a top down iterator over the data rows. The last value returned is the surface values.
    #[inline]
    pub fn top_down(&self) -> ProfileIterator {
        ProfileIterator {
            next_idx: (self.pressure.len() - 1) as isize,
            direction: -1,
            src: self,
        }
    }

    /// Get a row of data values from this sounding.
    #[inline]
    pub fn get_data_row(&self, idx: usize) -> Option<DataRow> {
        macro_rules! copy_to_result {
            ($result:ident, $field:ident, $idx:ident) => {
                match self.$field.get($idx) {
                    None => {},
                    Some(opt_val) => $result.$field = *opt_val,
                }
            };
        }

        if self.pressure.len() <= idx {
            return None;
        }

        let mut result = DataRow::default();

        copy_to_result!(result, pressure, idx);
        copy_to_result!(result, temperature, idx);
        copy_to_result!(result, wet_bulb, idx);
        copy_to_result!(result, dew_point, idx);
        copy_to_result!(result, theta_e, idx);
        copy_to_result!(result, direction, idx);
        copy_to_result!(result, speed, idx);
        copy_to_result!(result, omega, idx);
        copy_to_result!(result, height, idx);
        copy_to_result!(result, cloud_fraction, idx);

        Some(result)
    }

    /// Get the surface values in a `DataRow` format.
    #[inline]
    pub fn surface_as_data_row(&self) -> DataRow {
        let mut result = DataRow::default();
        result.pressure = self.station_pres;
        result.temperature = self.sfc_temperature;
        result.dew_point = self.sfc_dew_point;

        result.wet_bulb = self.station_pres.and_then(|p| {
            self.sfc_temperature.and_then(|t| {
                self.sfc_dew_point
                    .and_then(|dp| ::metfor::wet_bulb_c(t, dp, p).ok())
            })
        });

        result.theta_e = self.station_pres.and_then(|p| {
            self.sfc_temperature.and_then(|t| {
                self.sfc_dew_point
                    .and_then(|dp| ::metfor::theta_e_kelvin(t, dp, p).ok())
            })
        });

        result.direction = self.wind_dir;
        result.speed = self.wind_spd;
        result.omega = Some(0.0);
        result.height = self.station.elevation;

        result
    }

    /// Given a target pressure, return the row of data values closest to this one.
    pub fn fetch_nearest_pnt(&self, target_p: f64) -> DataRow {
        let mut idx: usize = 0;
        let mut best_abs_diff: f64 = ::std::f64::MAX;
        let sfc_pressure = &self.get_surface_value(Surface::StationPressure);
        for (i, p) in ::std::iter::once(sfc_pressure)
            .chain(self.pressure.iter())
            .enumerate()
        {
            if let Some(p) = *p {
                let abs_diff = (target_p - p).abs();
                if abs_diff < best_abs_diff {
                    best_abs_diff = abs_diff;
                    idx = i;
                }
                if abs_diff > best_abs_diff {
                    break;
                }
            }
        }

        if idx == 0 {
            self.surface_as_data_row()
        } else {
            self.get_data_row(idx - 1).unwrap()
        }
    }
}

/// Iterator over the data rows of a sounding. This may be a top down or bottom up iterator where
/// either the last or first row returned is the surface data.
pub struct ProfileIterator<'a> {
    next_idx: isize,
    direction: isize, // +1 for bottom up, -1 for top down
    src: &'a Sounding,
}

impl<'a> Iterator for ProfileIterator<'a> {
    type Item = DataRow;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let result = self.src.get_data_row(self.next_idx as usize);
        self.next_idx += self.direction;
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_profile(){
        let snd = Sounding::new();

        println!("snd = {:#?}", snd);
        let p = vec![Some(1000.0), Some(925.0), Some(850.0), Some(700.0)];
        let t = vec![Some(20.0), Some(18.0), Some(10.0), Some(2.0)];

        let snd = snd
            .set_profile(Profile::Pressure, p)
            .set_profile(Profile::Temperature, t)
            .set_surface_value(Surface::Temperature, 21.0)
            .set_surface_value(Surface::StationPressure, 1005.0);

        println!("snd = {:#?}", snd);
        assert!(snd.get_profile(Profile::Pressure).iter().all(|p| p.is_some()));
        assert!(snd.get_profile(Profile::Temperature).iter().all(|t| t.is_some()));
    }
}
