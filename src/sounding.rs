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
    /// U - wind speed (m/s) (West -> East is positive)
    UWind,
    /// V - wind speed (m/s) (South -> North is positive)
    VWind,
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
            UWind => "west to east wind",
            VWind => "south to north wind",
            Temperature => "2-meter temperature",
            DewPoint => "2-meter dew point",
            Precipitation => "precipitation (liquid equivalent)",
        };

        write!(f, "{}", string_rep)
    }
}

/// Sounding indexes.
///
/// Note that the `Sounding` data type only saves indexes that may be loaded from a serialized data
/// format such as a .bufr file or bufkit file. But this enum supports many more indexes which
/// might be calculated by a sounding analysis crate. When using `get_index`, it will always
/// return a missing value if the index is not stored in this data type. If you try to `set_index`
/// with an index that is not supported by the `Sounding` data type, it will panic in debug mode
/// and silently fail in release mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

impl fmt::Display for Index {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use Index::*;
        let string_rep = match *self {
            Showalter => "Showalter index",
            LI => "lifted index (LI)",
            SWeT => "severe weather threat index",
            K => "k index",
            LCL => "lifting condensation level (LCL)",
            PWAT => "precipitable water (PWAT)",
            TotalTotals => "total totals index (TT)",
            CAPE => "convective available potential energy (CAPE)",
            LCLTemperature => "temperature at the lifting condensation level (LCL)",
            CIN => "convective inhibition (CIN)",
            EquilibrimLevel => "equilibruim level",
            LFC => "level of free convection",
            BulkRichardsonNumber => "bulk Richardson number",
            Haines => "Haines index",
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
    /// U - wind speed (m/s) (West -> East is positive) // FIXME: really m/s?
    uwind: Option<f64>,
    /// V - wind speed (m/s) (South -> North is positive)
    vwind: Option<f64>,
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
            Temperature => self.sfc_temperature = Option::from(value),
            DewPoint => self.sfc_dew_point = Option::from(value),
            Precipitation => self.precip = Option::from(value),
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
            Temperature => self.sfc_temperature,
            DewPoint => self.sfc_dew_point,
            Precipitation => self.precip.map(|pp| pp * 25.4), // convert from mm to inches.
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
    #[deprecated]
    pub fn get_location(&self) -> (Option<f64>, Option<f64>, Option<f64>) {
        let mut lat = None;
        let mut lon = None;
        if let Some((latitdue, longitude)) = self.station.location {
            lat = Some(latitdue);
            lon = Some(longitude);
        }

        (lat, lon, self.station.elevation)
    }

    /// Set location information
    #[inline]
    #[deprecated]
    pub fn set_location<T, U, V>(mut self, latitude: T, longitude: U, elevation: V) -> Self
    where
        Option<f64>: From<T> + From<U> + From<V>,
    {
        let lat = Option::from(latitude);
        let lon = Option::from(longitude);
        let elevation = Option::from(elevation);

        let mut location = None;
        if lat.is_some() && lon.is_some() {
            location = Some((lat.unwrap(), lon.unwrap()));
        }

        self.station.location = location;
        self.station.elevation = elevation;

        self
    }

    /// Station number, USAF number, eg 727730
    #[inline]
    #[deprecated]
    pub fn set_station_num<T>(mut self, station_num: T) -> Self
    where
        Option<i32>: From<T>,
    {
        self.station.num = Option::from(station_num);
        self
    }

    /// Station number, USAF number, eg 727730
    #[inline]
    #[deprecated]
    pub fn get_station_num(&self) -> Option<i32> {
        self.station.station_num()
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
            next_value: Some(self.surface_as_data_row()),
            next_idx: 0,
            direction: 1,
            src: self,
        }
    }

    /// Get a top down iterator over the data rows. The last value returned is the surface values.
    #[inline]
    pub fn top_down(&self) -> ProfileIterator {
        ProfileIterator {
            next_value: self.get_data_row(self.pressure.len() - 1),
            next_idx: (self.pressure.len() - 2) as isize,
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
        if result.temperature.is_some() && result.dew_point.is_some() && result.pressure.is_some() {
            let temp = result.temperature.unwrap();
            let dp = result.dew_point.unwrap();
            let pres = result.pressure.unwrap();
            // TODO: Calculate wet bulb.
        }
        // TODO: Calculate theta-e
        // TODO: Calculate wind direction
        // TODO: Calculate wind speed
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
    next_value: Option<DataRow>,
    next_idx: isize,
    direction: isize, // +1 for bottom up, -1 for top down
    src: &'a Sounding,
}

impl<'a> Iterator for ProfileIterator<'a> {
    type Item = DataRow;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let result = self.next_value;
        self.next_value = if self.next_idx > 0 {
            self.src.get_data_row(self.next_idx as usize)
        } else if self.next_idx == -1 {
            Some(self.src.surface_as_data_row())
        } else {
            None
        };

        self.next_idx += self.direction;
        result
    }
}
