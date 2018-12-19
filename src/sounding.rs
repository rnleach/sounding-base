//! Data type and methods to store an atmospheric sounding.

use chrono::NaiveDateTime;
use metfor::{Celsius, HectoPascal, Kelvin, Meters, Mm, PaPS, Quantity, WindSpdDir};
use optional::Optioned;

use crate::data_row::DataRow;
use crate::station_info::StationInfo;

/// All the variables stored in the sounding.
///
/// The upper air profile variables are stored in parallel vectors. If a profile lacks a certain
/// variable, e.g. cloud fraction, that whole vector has length 0 instead of being full of missing
/// values.
///
#[derive(Clone, Debug, Default)]
pub struct Sounding {
    // Station info
    station: StationInfo,

    // Valid time of sounding
    valid_time: Option<NaiveDateTime>,
    // Difference in model initialization time and `valid_time` in hours.
    lead_time: Optioned<i32>,

    // Profiles
    pressure: Vec<Optioned<HectoPascal>>,
    temperature: Vec<Optioned<Celsius>>,
    wet_bulb: Vec<Optioned<Celsius>>,
    dew_point: Vec<Optioned<Celsius>>,
    theta_e: Vec<Optioned<Kelvin>>,
    wind: Vec<Optioned<WindSpdDir>>,
    pvv: Vec<Optioned<PaPS>>,
    height: Vec<Optioned<Meters>>,
    cloud_fraction: Vec<Optioned<f64>>,

    // Surface variables
    mslp: Optioned<HectoPascal>,
    station_pressure: Optioned<HectoPascal>,
    sfc_temperature: Optioned<Celsius>,
    sfc_dew_point: Optioned<Celsius>,
    low_cloud: Optioned<f64>,
    mid_cloud: Optioned<f64>,
    high_cloud: Optioned<f64>,
    precipitation: Optioned<Mm>,
    sfc_wind: Optioned<WindSpdDir>,
}

macro_rules! make_profile_setter {
    ($(#[$attr:meta])* => $name:tt, $sfc_val:ident, $inner_type:tt, $p_var:ident) => {
        $(#[$attr])*
        pub fn $name(self, mut profile: Vec<Optioned<$inner_type>>) -> Self {
            if !profile.is_empty() {
                profile.insert(0, self.$sfc_val);
            }
            Self {$p_var: profile, ..self}
        }
    };
    ($(#[$attr:meta])* => $name:tt, $method:ident(), $inner_type:tt, $p_var:ident) => {
        $(#[$attr])*
        pub fn $name(self, mut profile: Vec<Optioned<$inner_type>>) -> Self {
            if !profile.is_empty() {
                profile.insert(0, self.$method().into());
            }
            Self {$p_var: profile, ..self}
        }
    };
    ($(#[$attr:meta])* => $name:tt, $sfc_val:expr, $inner_type:tt, $p_var:ident) => {
        $(#[$attr])*
        pub fn $name(self, mut profile: Vec<Optioned<$inner_type>>) -> Self {
            if !profile.is_empty() {
                profile.insert(0, $sfc_val);
            }
            Self {$p_var: profile, ..self}
        }
    };
}

impl Sounding {
    /// Create a new sounding with default values. This is a proxy for default with a clearer name.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use sounding_base::Sounding;
    ///
    /// let snd = Sounding::new();
    /// println!("{:?}", snd);
    /// ```
    #[inline]
    pub fn new() -> Self {
        Sounding::default()
    }

    /// Set the station info.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use sounding_base::{Sounding, StationInfo};
    ///
    /// let stn = StationInfo::new();
    /// // set station values
    ///
    /// let _snd = Sounding::new()
    ///     .set_station_info(stn);
    ///
    /// ```
    #[inline]
    pub fn set_station_info(mut self, new_value: StationInfo) -> Self {
        self.station = new_value;
        self
    }

    /// Get the station info
    ///
    /// # Examples
    ///
    /// ```rust
    /// use sounding_base::StationInfo;
    /// # use sounding_base::doctest::make_test_sounding;
    ///
    /// let snd = make_test_sounding();
    /// let stn: StationInfo = snd.station_info();
    ///
    /// println!("{:?}", stn);
    ///
    /// ```
    #[inline]
    pub fn station_info(&self) -> StationInfo {
        self.station
    }

    make_profile_setter!(
        /// Set the pressure profile.
        ///
        /// # Examples
        /// ```rust
        /// use sounding_base::Sounding;
        /// use metfor::HectoPascal;
        /// use optional::{some, Optioned};
        /// 
        /// let data = vec![1000.0, 925.0, 850.0, 700.0, 500.0, 300.0, 250.0, 200.0, 150.0, 100.0];
        /// let pressure_data: Vec<Optioned<HectoPascal>> = data.into_iter()
        ///     .map(HectoPascal)
        ///     .map(some)
        ///     .collect();
        ///
        /// let _snd = Sounding::new()
        ///     .set_pressure_profile(pressure_data);
        /// ```
        #[inline]
        => set_pressure_profile, station_pressure, HectoPascal, pressure
    );

    /// Get the pressure profile
    ///
    /// # Examples
    ///
    /// ```rust
    /// use sounding_base::Sounding;
    /// # use sounding_base::doctest::make_test_sounding;
    ///
    /// let snd = make_test_sounding();
    /// let data = snd.pressure_profile();
    ///
    /// for p in data {
    ///     if let Some(p) = p.into_option() {
    ///         println!("{}", p);
    ///     } else {
    ///         println!("missing value!");
    ///     }
    /// }
    ///
    /// // Uninitialized profiles just return an empty vector.
    /// let snd = Sounding::new();
    /// let data = snd.pressure_profile();
    /// assert!(data.is_empty());
    ///
    /// ```
    #[inline]
    pub fn pressure_profile(&self) -> &[Optioned<HectoPascal>] {
        &self.pressure
    }

    make_profile_setter!(
        /// Set the temperature profile.
        ///
        /// See `set_pressure_profile` for an example of usage, keeping mind the units type may
        /// be different.
        #[inline]
        => set_temperature_profile, sfc_temperature, Celsius, temperature
    );

    /// Get the temperature profile.
    ///
    /// See `pressure_profile` for an example of using getters, keeping in mind the units type may
    /// be different.
    #[inline]
    pub fn temperature_profile(&self) -> &[Optioned<Celsius>] {
        &self.temperature
    }

    make_profile_setter!(
        /// Set the dew point profile.
        ///
        /// See `set_pressure_profile` for an example of usage, keeping mind the units type may
        /// be different.
        #[inline]
        => set_dew_point_profile, sfc_dew_point, Celsius, dew_point
    );

    /// Get the dew point profile.
    ///
    /// See `pressure_profile` for an example of using getters, keeping in mind the units type may
    /// be different.
    #[inline]
    pub fn dew_point_profile(&self) -> &[Optioned<Celsius>] {
        &self.dew_point
    }

    make_profile_setter!(
        /// Set the wet bulb profile.
        ///
        /// See `set_pressure_profile` for an example of usage, keeping mind the units type may
        /// be different.
        #[inline]
        => set_wet_bulb_profile, surface_wet_bulb(), Celsius, wet_bulb
    );

    /// Get the wet bulb temperature profile.
    ///
    /// See `pressure_profile` for an example of using getters, keeping in mind the units type may
    /// be different.
    #[inline]
    pub fn wet_bulb_profile(&self) -> &[Optioned<Celsius>] {
        &self.wet_bulb
    }

    make_profile_setter!(
        /// Set the theta e profile.
        ///
        /// See `set_pressure_profile` for an example of usage, keeping mind the units type may
        /// be different.
        #[inline]
        => set_theta_e_profile, surface_theta_e(), Kelvin, theta_e
    );

    /// Get the equivalent potential temperature profile.
    ///
    /// See `pressure_profile` for an example of using getters, keeping in mind the units type may
    /// be different.
    #[inline]
    pub fn theta_e_profile(&self) -> &[Optioned<Kelvin>] {
        &self.theta_e
    }

    make_profile_setter!(
        /// Set the wind profile.
        ///
        /// See `set_pressure_profile` for an example of usage, keeping mind the units type may
        /// be different.
        #[inline]
        => set_wind_profile, sfc_wind, WindSpdDir, wind
    );

    /// Get the wind profile.
    ///
    /// See `pressure_profile` for an example of using getters, keeping in mind the units type may
    /// be different.
    #[inline]
    pub fn wind_profile(&self) -> &[Optioned<WindSpdDir>] {
        &self.wind
    }

    make_profile_setter!(
        /// Set the pressure vertical velocity profile.
        ///
        /// See `set_pressure_profile` for an example of usage, keeping mind the units type may
        /// be different.
        #[inline]
        => set_pvv_profile, optional::some(PaPS(0.0)), PaPS, pvv
    );

    /// Get the pressure vertical velocity profile.
    ///
    /// See `pressure_profile` for an example of using getters, keeping in mind the units type may
    /// be different.
    #[inline]
    pub fn pvv_profile(&self) -> &[Optioned<PaPS>] {
        &self.pvv
    }

    make_profile_setter!(
        /// Set the geopotential height profile.
        ///
        /// See `set_pressure_profile` for an example of usage, keeping mind the units type may
        /// be different.
        #[inline]
        => set_height_profile, surface_height(), Meters, height
    );

    /// Get the geopotential height profile.
    ///
    /// See `pressure_profile` for an example of using getters, keeping in mind the units type may
    /// be different.
    #[inline]
    pub fn height_profile(&self) -> &[Optioned<Meters>] {
        &self.height
    }

    make_profile_setter!(
        /// Set the cloud cover profile.
        ///
        /// See `set_pressure_profile` for an example of usage, keeping mind the units type may
        /// be different.
        #[inline]
        => set_cloud_fraction_profile, optional::some(0.0), f64, cloud_fraction
    );

    /// Get the cloud fraction profile.
    ///
    /// See `pressure_profile` for an example of using getters, keeping in mind the units type may
    /// be different.
    #[inline]
    pub fn cloud_fraction_profile(&self) -> &[Optioned<f64>] {
        &self.cloud_fraction
    }

    /// Set the mean sea level pressure.
    #[inline]
    pub fn set_mslp<T, U>(self, value: Optioned<T>) -> Self
    where
        HectoPascal: From<T>,
        T: optional::Noned + metfor::Pressure,
    {
        let mslp = value.map_t(HectoPascal::from);
        Self { mslp, ..self }
    }

    /// Get the mean sea level pressure
    #[inline]
    pub fn mslp(&self) -> Optioned<HectoPascal> {
        self.mslp
    }

    /// Set the station pressure.
    #[inline]
    pub fn set_station_pressure<T>(mut self, value: Optioned<T>) -> Self
    where
        HectoPascal: From<T>,
        T: optional::Noned + metfor::Pressure,
    {
        // Add it in to the profile.
        let station_pressure = value.map_t(HectoPascal::from);
        if !self.pressure.is_empty() {
            self.pressure[0] = station_pressure;
        }

        self = Self {
            station_pressure,
            ..self
        };
        self.update_sfc_wet_bulb_theta_e(); // updates wet bulb and theta_e profiles
        self
    }

    /// Get the mean sea level pressure
    #[inline]
    pub fn station_pressure(&self) -> Optioned<HectoPascal> {
        self.station_pressure
    }

    /// Set the surface temperature.
    #[inline]
    pub fn set_sfc_temperature<T>(mut self, value: Optioned<T>) -> Self
    where
        Celsius: From<T>,
        T: optional::Noned + metfor::Temperature,
    {
        // Add it in to the profile.
        let sfc_temperature = value.map_t(Celsius::from);
        if !self.temperature.is_empty() {
            self.temperature[0] = sfc_temperature;
        }

        self = Self {
            sfc_temperature,
            ..self
        };
        self.update_sfc_wet_bulb_theta_e(); // updates wet bulb and theta_e profiles
        self
    }

    /// Get the surface temperature
    #[inline]
    pub fn sfc_temperature(&self) -> Optioned<Celsius> {
        self.sfc_temperature
    }

    /// Set the surface dew point
    #[inline]
    pub fn set_sfc_dew_point<T>(mut self, value: Optioned<T>) -> Self
    where
        Celsius: From<T>,
        T: optional::Noned + metfor::Temperature,
    {
        // Add it in to the profile.
        let sfc_dew_point = value.map_t(Celsius::from);
        if !self.dew_point.is_empty() {
            self.dew_point[0] = sfc_dew_point;
        }

        self = Self {
            sfc_dew_point,
            ..self
        };
        self.update_sfc_wet_bulb_theta_e(); // updates wet bulb and theta_e profiles
        self
    }

    /// Get the surface dew point
    #[inline]
    pub fn sfc_dew_point(&self) -> Optioned<Celsius> {
        self.sfc_dew_point
    }

    /// Set the surface wind
    #[inline]
    pub fn set_sfc_wind<T>(mut self, value: Optioned<T>) -> Self
    where
        WindSpdDir: From<T>,
        T: optional::Noned + metfor::Wind,
    {
        let sfc_wind = value.map_t(WindSpdDir::from);
        if !self.wind.is_empty() {
            self.wind[0] = sfc_wind;
        }

        Self { sfc_wind, ..self }
    }

    /// Get the surface wind.
    #[inline]
    pub fn sfc_wind(&self) -> Optioned<WindSpdDir> {
        self.sfc_wind
    }

    /// Set the precipitation.
    ///
    /// # Examples
    ///```rust
    /// use sounding_base::Sounding;
    /// use metfor::{Meters, Feet};
    /// use optional::{some, none};
    ///
    /// let _snd = Sounding::new().set_precipitation(some(Feet(1.0/12.0)));
    /// let _snd = Sounding::new().set_precipitation(some(Meters(0.025)));
    /// let _snd = Sounding::new().set_precipitation(none::<Meters>());
    ///```
    #[inline]
    pub fn set_precipitation<T>(self, value: Optioned<T>) -> Self
    where
        Mm: From<T>,
        T: optional::Noned + metfor::Length,
    {
        let precipitation = value.map_t(Mm::from);

        Self {
            precipitation,
            ..self
        }
    }

    /// Get the precipitation
    #[inline]
    pub fn precipitation(&self) -> Optioned<Mm> {
        self.precipitation
    }

    /// Set the low cloud amount
    #[inline]
    pub fn set_low_cloud<T>(self, value: T) -> Self
    where
        Optioned<f64>: From<T>,
    {
        let low_cloud: Optioned<f64> = Optioned::from(value);

        Self { low_cloud, ..self }
    }

    /// Get the low cloud
    #[inline]
    pub fn low_cloud(&self) -> Optioned<f64> {
        self.low_cloud
    }

    /// Set the mid cloud amount
    #[inline]
    pub fn set_mid_cloud<T>(self, value: T) -> Self
    where
        Optioned<f64>: From<T>,
    {
        let mid_cloud: Optioned<f64> = Optioned::from(value);

        Self { mid_cloud, ..self }
    }

    /// Get the mid cloud
    #[inline]
    pub fn mid_cloud(&self) -> Optioned<f64> {
        self.mid_cloud
    }

    /// Set the high cloud amount
    #[inline]
    pub fn set_high_cloud<T>(self, value: T) -> Self
    where
        Optioned<f64>: From<T>,
    {
        let high_cloud: Optioned<f64> = Optioned::from(value);

        Self { high_cloud, ..self }
    }

    /// Get the high cloud
    #[inline]
    pub fn high_cloud(&self) -> Optioned<f64> {
        self.high_cloud
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
            ($result:ident, $profile:ident, $idx:ident) => {
                match self.$profile.get($idx) {
                    None => {}
                    Some(opt_val) => $result.$profile = *opt_val,
                }
            };
        }

        if idx >= self.pressure.len() {
            return None;
        }

        let mut result = DataRow::default();

        copy_to_result!(result, pressure, idx);
        copy_to_result!(result, temperature, idx);
        copy_to_result!(result, wet_bulb, idx);
        copy_to_result!(result, dew_point, idx);
        copy_to_result!(result, theta_e, idx);
        copy_to_result!(result, wind, idx);
        copy_to_result!(result, pvv, idx);
        copy_to_result!(result, height, idx);
        copy_to_result!(result, cloud_fraction, idx);

        Some(result)
    }

    /// Get the surface values in a `DataRow` format.
    #[inline]
    pub fn surface_as_data_row(&self) -> Option<DataRow> {
        self.get_data_row(0)
    }

    /// Given a target pressure, return the row of data values closest to this one.
    pub fn fetch_nearest_pnt<P>(&self, target_p: P) -> DataRow
    where
        HectoPascal: From<P>,
        P: metfor::Pressure,
    {
        let tgt_p = HectoPascal::from(target_p);

        let mut idx: usize = 0;
        let mut best_abs_diff: f64 = ::std::f64::MAX;
        for (i, &p_opt) in self.pressure.iter().enumerate() {
            if let Some(p) = p_opt.into_option() {
                let abs_diff = (tgt_p.unpack() - p.unpack()).abs();
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
            self.surface_as_data_row().unwrap()
        } else {
            self.get_data_row(idx - 1).unwrap()
        }
    }

    #[inline]
    fn surface_wet_bulb(&self) -> Option<Celsius> {
        let sfc_t = self.sfc_temperature.into_option()?;
        let sfc_p = self.station_pressure.into_option()?;
        let sfc_dp = self.sfc_dew_point.into_option()?;

        metfor::wet_bulb(sfc_t, sfc_dp, sfc_p)
    }

    #[inline]
    fn surface_theta_e(&self) -> Option<Kelvin> {
        let sfc_t = self.sfc_temperature.into_option()?;
        let sfc_p = self.station_pressure.into_option()?;
        let sfc_dp = self.sfc_dew_point.into_option()?;

        metfor::theta_e(sfc_t, sfc_dp, sfc_p)
    }

    #[inline]
    fn surface_height(&self) -> Option<Meters> {
        self.station_info().elevation().into_option()
    }

    #[inline]
    fn update_sfc_wet_bulb_theta_e(&mut self) {
        if let (Some(sfc_p), Some(sfc_t), Some(sfc_dp)) = (
            self.station_pressure.into_option(),
            self.sfc_temperature.into_option(),
            self.sfc_dew_point.into_option(),
        ) {
            if !self.wet_bulb.is_empty() {
                self.wet_bulb[0] = metfor::wet_bulb(sfc_t, sfc_dp, sfc_p).into();
            }

            if !self.theta_e.is_empty() {
                self.theta_e[0] = metfor::theta_e(sfc_t, sfc_dp, sfc_p).into();
            }
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

// FIXME: only configure for test and doc tests, not possible as of 1.26
#[doc(hidden)]
pub mod doctest {
    use super::*;

    pub fn make_test_sounding() -> super::Sounding {
        use optional::some;

        let p = vec![
            some(HectoPascal(1000.0)),
            some(HectoPascal(925.0)),
            some(HectoPascal(850.0)),
            some(HectoPascal(700.0)),
        ];
        let t = vec![
            some(Celsius(20.0)),
            some(Celsius(18.0)),
            some(Celsius(10.0)),
            some(Celsius(2.0)),
        ];

        Sounding::new()
            .set_pressure_profile(p)
            .set_temperature_profile(t)
            .set_sfc_temperature(some(Celsius(21.0)))
            .set_station_pressure(some(HectoPascal(1005.0)))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_profile() {
        let snd = doctest::make_test_sounding();

        println!("snd = {:#?}", snd);
        assert!(snd.pressure_profile().iter().all(|t| t.is_some()));
        assert!(snd.temperature_profile().iter().all(|t| t.is_some()));
        assert_eq!(
            snd.pressure_profile()
                .iter()
                .filter(|p| p.is_some())
                .count(),
            5
        );

        assert_eq!(
            snd.temperature_profile()
                .iter()
                .filter(|t| t.is_some())
                .count(),
            5
        );
    }
}
