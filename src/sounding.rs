//! Data type and methods to store an atmospheric sounding.

use chrono::NaiveDateTime;
use optional::{Optioned, none, wrap};

use data_row::DataRow;
use enums::{Profile, Surface};
use station_info::StationInfo;

/// All the variables stored in the sounding.
///
/// The upper air profile variables are stored in parallel vectors. If a profile lacks a certain
/// variable, e.g. cloud fraction, that whole vector has length 0 instead of being full of missing
/// values.
///
#[derive(Clone, Debug, Default)]
pub struct Sounding {
    /// Station info
    station: StationInfo,

    /// Valid time of sounding
    valid_time: Option<NaiveDateTime>,
    /// Difference in model initialization time and `valid_time` in hours.
    lead_time: Optioned<i32>,

    // Upper air profile
    /// Pressure (hPa) profile
    pressure: Vec<Optioned<f64>>,
    /// Temperature (c) profile
    temperature: Vec<Optioned<f64>>,
    /// Wet-bulb (c) profile
    wet_bulb: Vec<Optioned<f64>>,
    /// Dew Point (C) profile
    dew_point: Vec<Optioned<f64>>,
    /// Equivalent Potential Temperature (K) profile
    theta_e: Vec<Optioned<f64>>,
    /// Wind direction (degrees) profile
    direction: Vec<Optioned<f64>>,
    /// Wind speed (knots) profile
    speed: Vec<Optioned<f64>>,
    /// Vertical velocity (Pa/sec), pressure vertical coordinate
    omega: Vec<Optioned<f64>>,
    /// Geopotential Height (m) profile
    height: Vec<Optioned<f64>>,
    /// Cloud coverage fraction in percent
    cloud_fraction: Vec<Optioned<f64>>,

    // Surface data
    /// Surface pressure reduce to mean sea level (hPa)
    mslp: Optioned<f64>,
    /// Surface pressure (hPa)
    station_pres: Optioned<f64>,
    /// Low cloud fraction
    low_cloud: Optioned<f64>,
    /// Mid cloud fraction
    mid_cloud: Optioned<f64>,
    /// Hi cloud fraction
    hi_cloud: Optioned<f64>,
    /// Wind direction
    wind_dir: Optioned<f64>,
    /// Wind speed in knots
    wind_spd: Optioned<f64>,
    /// 2 meter  temperature
    sfc_temperature: Optioned<f64>,
    /// 2 meter dew point
    sfc_dew_point: Optioned<f64>,
    /// Precipitation in mm
    precip: Optioned<f64>,
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
    pub fn set_profile(mut self, var: Profile, mut values: Vec<Optioned<f64>>) -> Self {
        use self::Profile::*;

        let sfc_val = match var {
            Pressure => self.station_pres,
            Temperature => self.sfc_temperature,
            WetBulb => self.station_pres.and_then(|p| {
                self.sfc_temperature.and_then(|t| {
                    self.sfc_dew_point
                        .and_then(|dp| ::metfor::wet_bulb_c(t, dp, p).ok().into())
                })
            }),
            DewPoint => self.sfc_dew_point,
            ThetaE => self.station_pres.and_then(|p| {
                self.sfc_temperature.and_then(|t| {
                    self.sfc_dew_point
                        .and_then(|dp| ::metfor::theta_e_kelvin(t, dp, p).ok().into())
                })
            }),
            WindDirection => self.wind_dir,
            WindSpeed => self.wind_spd,
            PressureVerticalVelocity => wrap(0.0),
            GeopotentialHeight => Optioned::from(self.station.elevation()),
            CloudFraction => none(),
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
    pub fn get_profile(&self, var: Profile) -> &[Optioned<f64>] {
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
        Optioned<f64>: From<T>,
    {
        let value = Optioned::from(value);

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
                                .and_then(|dp| ::metfor::wet_bulb_c(t, dp, p).ok().into())
                        })
                    });
                }

                if !self.theta_e.is_empty() {
                    self.theta_e[0] = self.station_pres.and_then(|p| {
                        self.sfc_temperature.and_then(|t| {
                            self.sfc_dew_point
                                .and_then(|dp| ::metfor::theta_e_kelvin(t, dp, p).ok().into())
                        })
                    });
                }
            }
        }

        self
    }

    /// Get a surface variable
    #[inline]
    pub fn get_surface_value(&self, var: Surface) -> Optioned<f64> {
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
            Precipitation => self.precip.map_t(|pp| pp * 25.4), // convert from mm to inches.
        }
    }

    /// Difference in model initialization time and `valid_time` in hours.
    #[inline]
    pub fn set_lead_time<T>(mut self, lt: T) -> Self
    where
        Optioned<i32>: From<T>,
    {
        self.lead_time = Optioned::from(lt);
        self
    }

    /// Difference in model initialization time and `valid_time` in hours.
    #[inline]
    pub fn get_lead_time(&self) -> Optioned<i32> {
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
    pub fn bottom_up<'a>(&'a self) -> impl Iterator<Item = DataRow> + 'a {
        ProfileIterator {
            next_idx: 0,
            direction: 1,
            src: self,
        }
    }

    /// Get a top down iterator over the data rows. The last value returned is the surface values.
    #[inline]
    pub fn top_down<'a>(&'a self) -> impl Iterator<Item = DataRow> + 'a {
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
                    None => {}
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
                    .and_then(|dp| ::metfor::wet_bulb_c(t, dp, p).ok().into())
            })
        });

        result.theta_e = self.station_pres.and_then(|p| {
            self.sfc_temperature.and_then(|t| {
                self.sfc_dew_point
                    .and_then(|dp| ::metfor::theta_e_kelvin(t, dp, p).ok().into())
            })
        });

        result.direction = self.wind_dir;
        result.speed = self.wind_spd;
        result.omega = wrap(0.0);
        result.height = self.station.elevation().map_or(none(),|elev| wrap(elev));

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
            if let Some(p) = p.map_or(None, |p| Some(p)) {
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
struct ProfileIterator<'a> {
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
    fn test_profile() {
        use optional::{some};

        let snd = Sounding::new();

        println!("snd = {:#?}", snd);
        let p = vec![some(1000.0), some(925.0), some(850.0), some(700.0)];
        let t = vec![some(20.0), some(18.0), some(10.0), some(2.0)];

        let snd = snd.set_profile(Profile::Pressure, p)
            .set_profile(Profile::Temperature, t)
            .set_surface_value(Surface::Temperature, wrap(21.0))
            .set_surface_value(Surface::StationPressure, wrap(1005.0));

        println!("snd = {:#?}", snd);
        assert!(
            snd.get_profile(Profile::Pressure)
                .iter()
                .all(|p| p.is_some())
        );
        assert!(
            snd.get_profile(Profile::Temperature)
                .iter()
                .all(|t| t.is_some())
        );
    }
}
