//! Data type and methods to store an atmospheric sounding.

use chrono::NaiveDateTime;

/// All the variables stored in the sounding.
///
/// The upper air profile variables are stored in parallel vectors. If a profile lacks a certain
/// variable, e.g. cloud fraction, that whole vector has length 0 instead of being full of missing
/// values.
///
#[derive(Default)]
pub struct Sounding {
    // Station info section
    /// station number, USAF number, eg 727730
    num: Option<i32>,
    /// Valid time of sounding
    valid_time: Option<NaiveDateTime>,
    /// Difference in model initialization time and `valid_time` in hours.
    lead_time: Option<i32>,
    /// Latitude of grid point used to make sounding.
    lat: Option<f64>,
    /// Longitude of grid point used to make sounding.
    lon: Option<f64>,
    /// Elevation of grid point in meters, this is in model terrain, not necessarily the same as
    /// the real world.
    elevation: Option<f64>,

    // Sounding Indexes
    /// Showalter index
    show: Option<f64>,
    /// Lifted index
    li: Option<f64>,
    /// Severe Weather Threat Index
    swet: Option<f64>,
    /// K-index
    kinx: Option<f64>,
    /// Lifting Condensation Level, or LCL (hPa), pressure vertical coordinate.
    lclp: Option<f64>,
    /// Precipitable Water (mm)
    pwat: Option<f64>,
    /// Total-Totals
    totl: Option<f64>,
    /// Convective Available Potential Energy, or CAPE. (J/kg)
    cape: Option<f64>,
    /// Temperature at LCL (K)
    lclt: Option<f64>,
    /// Convective Inhibitive Energy, or CIN (J/kg)
    cins: Option<f64>,
    /// Equilibrium Level (hPa), pressure vertical coordinate
    eqlv: Option<f64>,
    /// Level of Free Convection (hPa), pressure vertical coordinate
    lfc: Option<f64>,
    /// Bulk Richardson Number
    brch: Option<f64>,

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
    /// U - wind speed (m/s) (West -> East is positive)
    uwind: Option<f64>,
    /// V - wind speed (m/s) (South -> North is positive)
    vwind: Option<f64>,
}

/// A view of a row of the sounding data.
#[derive(Clone, Default, Copy, Debug)]
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

/// Profile variables
#[derive(Debug, Clone, Copy)]
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

/// Surface observed variables
#[derive(Debug, Clone, Copy)]
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
    /// U - wind speed (m/s) (West -> East is positive)
    UWind,
    /// V - wind speed (m/s) (South -> North is positive)
    VWind,
}

/// Sounding indexes.
///
/// Note that the `Sounding` data type only saves indexes that may be loaded from a serialied data
/// format such as a .bufr file or bufkit file. But this enum supports many more indexes which
/// might be calculated by a sounding analysis crate. When using `get_index`, it will always
/// return a missing value if the index is not stored in this data type. If you try to `set_index`
/// with an index that is not supported by the `Sounding` data type, it will panic in debug mode
/// and silently fail in release mode.
#[derive(Debug)]
pub enum Index {
    /// Showalter index
    Showalter,
    /// Lifted index
    LI,
    /// Severe Weather Threat Index
    SWeT,
    /// K-index
    K,
    /// Lifting Condensation Level, or LCL (hPa), pressure vertical coordinate.
    LCL,
    /// Precipitable Water (mm)
    PWAT,
    /// Total-Totals
    TotalTotals,
    /// Convective Available Potential Energy, or CAPE. (J/kg)
    CAPE,
    /// Temperature at LCL (K)
    LCLTemperature,
    /// Convective Inhibitive Energy, or CIN (J/kg)
    CIN,
    /// Equilibrium Level (hPa), pressure vertical coordinate
    EquilibrimLevel,
    /// Level of Free Convection (hPa), pressure vertical coordinate
    LFC,
    /// Bulk Richardson Number
    BulkRichardsonNumber,
    /// Haines index
    Haines,
}

impl Sounding {
    /// Create a new sounding with default values. This is a proxy for default with a clearer name.
    #[inline]
    pub fn new() -> Self {
        Sounding::default()
    }

    /// Set a profile variable
    #[inline]
    pub fn set_profile(mut self, var: Profile, values: Vec<Option<f64>>) -> Self {
        use self::Profile::*;
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
        };

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
        use self::Surface::*;
        match var {
            MSLP => self.mslp = Option::from(value),
            StationPressure => self.station_pres = Option::from(value),
            LowCloud => self.low_cloud = Option::from(value),
            MidCloud => self.mid_cloud = Option::from(value),
            HighCloud => self.hi_cloud = Option::from(value),
            UWind => self.uwind = Option::from(value),
            VWind => self.vwind = Option::from(value),
        };

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
            UWind => self.uwind,
            VWind => self.vwind,
        }
    }

    /// Set an index value
    #[inline]
    pub fn set_index<T>(mut self, var: Index, value: T) -> Self
    where
        Option<f64>: From<T>,
    {
        use self::Index::*;

        match var {
            Showalter => self.show = Option::from(value),
            LI => self.li = Option::from(value),
            SWeT => self.swet = Option::from(value),
            K => self.kinx = Option::from(value),
            LCL => self.lclp = Option::from(value),
            PWAT => self.pwat = Option::from(value),
            TotalTotals => self.totl = Option::from(value),
            CAPE => self.cape = Option::from(value),
            LCLTemperature => self.lclt = Option::from(value),
            CIN => self.cins = Option::from(value),
            EquilibrimLevel => self.eqlv = Option::from(value),
            LFC => self.lfc = Option::from(value),
            BulkRichardsonNumber => self.brch = Option::from(value),
            _not_used => {
                #[cfg(debug_assert)]
                {
                    panic!(format!(
                        "The index {:?} is not stored in the Sounding datatype, \
                         perhaps you want to use the sounding-analysis crate to create it.",
                        _not_used
                    ));
                }
                #[cfg(not(debug_assert))]
                {}
            }
        }

        self
    }

    /// Get an index value
    #[inline]
    pub fn get_index(&self, var: Index) -> Option<f64> {
        use self::Index::*;

        match var {
            Showalter => self.show,
            LI => self.li,
            SWeT => self.swet,
            K => self.kinx,
            LCL => self.lclp,
            PWAT => self.pwat,
            TotalTotals => self.totl,
            CAPE => self.cape,
            LCLTemperature => self.lclt,
            CIN => self.cins,
            EquilibrimLevel => self.eqlv,
            LFC => self.lfc,
            BulkRichardsonNumber => self.brch,
            _not_used => {
                #[cfg(debug_assert)]
                {
                    panic!(format!(
                        "The index {:?} is not stored in the Sounding datatype, \
                         perhaps you want to use the sounding-analysis crate to create it.",
                        _not_used
                    ));
                }
                #[cfg(not(debug_assert))]
                {
                    None
                }
            }
        }
    }

    /// Get location information.
    ///
    /// # returns
    /// `(latitude, longitude, elevation in meters)`
    #[inline]
    pub fn get_location(&self) -> (Option<f64>, Option<f64>, Option<f64>) {
        (self.lat, self.lon, self.elevation)
    }

    /// Set location information
    #[inline]
    pub fn set_location<T, U, V>(mut self, latitude: T, longitude: U, elevation: V) -> Self
    where
        Option<f64>: From<T> + From<U> + From<V>,
    {
        self.lat = Option::from(latitude);
        self.lon = Option::from(longitude);
        self.elevation = Option::from(elevation);

        self
    }

    /// Station number, USAF number, eg 727730
    #[inline]
    pub fn set_station_num<T>(mut self, station_num: T) -> Self
    where
        Option<i32>: From<T>,
    {
        self.num = Option::from(station_num);
        self
    }

    /// Station number, USAF number, eg 727730
    #[inline]
    pub fn get_station_num(&self) -> Option<i32> {
        self.num
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

    /// Valid time of sounding
    #[inline]
    pub fn get_valid_time(&self) -> Option<NaiveDateTime> {
        self.valid_time
    }

    /// Valid time of sounding
    #[inline]
    pub fn set_valid_time<T>(mut self, valid_time: T) -> Self
    where
        Option<NaiveDateTime>: From<T>,
    {
        self.valid_time = Option::from(valid_time);
        self
    }

    /// Get a bottom up iterator over the data rows.
    #[inline]
    pub fn bottom_up(&self) -> ProfileIterator {
        ProfileIterator {
            next: 0,
            direction: 1,
            src: self,
        }
    }

    /// Get a top down iterator over the data rows
    #[inline]
    pub fn top_down(&self) -> ProfileIterator {
        ProfileIterator {
            next: (self.pressure.len() - 1) as isize,
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

    /// Given a target pressure, return the row of data values closest to this one.
    pub fn fetch_nearest_pnt(&self, target_p: f64) -> DataRow {
        let mut idx: usize = 0;
        let mut best_abs_diff: f64 = ::std::f64::MAX;
        for (i, p) in self.pressure.iter().enumerate() {
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

        self.get_data_row(idx).unwrap()
    }
}

/// Iterator over the data rows of a sounding.
pub struct ProfileIterator<'a> {
    next: isize,
    direction: isize, // +1 for bottom up, -1 for top down
    src: &'a Sounding,
}

impl<'a> Iterator for ProfileIterator<'a> {
    type Item = DataRow;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let result = self.src.get_data_row(self.next as usize);
        self.next += self.direction;
        result
    }
}
