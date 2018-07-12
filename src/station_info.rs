use optional::Optioned;

/// Station information including location data and identification number.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct StationInfo {
    /// station number, USAF number, eg 727730
    num: Optioned<i32>,
    /// Latitude and longitude.
    location: Option<(f64, f64)>,
    /// Elevation in meters, this may be in model terrain, not necessarily the same as
    /// the real world.
    elevation: Optioned<f64>,
}

impl StationInfo {
    /// Create a new `StationInfo` object.
    ///
    /// # Arguments
    /// station_num: The USAF station identifier, or None.
    ///
    /// location: The latitude and longitude as a tuple, or None.
    ///
    /// elevation: The elevation of the station **in meters**.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # extern crate optional;
    /// # extern crate sounding_base;
    ///
    /// use sounding_base::StationInfo;
    /// use optional::{some, none};
    ///
    /// let stn = StationInfo::new_with_values(12345, (45.2,-113.5), 2000.0);
    ///
    /// // Note that lat-lon is an `Option` and not an `Optioned`
    /// let stn = StationInfo::new_with_values(some(12345), None, none());
    /// ```
    #[inline]
    pub fn new_with_values<T, U, V>(station_num: T, location: U, elevation: V) -> Self
    where
        T: Into<Optioned<i32>>,
        U: Into<Option<(f64, f64)>>,
        V: Into<Optioned<f64>>,
    {
        StationInfo {
            num: station_num.into(),
            location: location.into(),
            elevation: elevation.into(),
        }
    }

    /// Create a new object with default values.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # extern crate optional;
    /// # extern crate sounding_base;
    ///
    /// use sounding_base::StationInfo;
    /// use optional::{some, none};
    ///
    /// assert_eq!(StationInfo::new().station_num(), none());
    /// assert_eq!(StationInfo::new().location(), None);
    /// assert_eq!(StationInfo::new().elevation(), none());
    ///
    /// ```
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Builder method to add a station number.
    #[inline]
    pub fn with_station<T>(mut self, number: T) -> Self
    where
        Optioned<i32>: From<T>,
    {
        self.num = Optioned::from(number);

        self
    }

    /// Builder method to add a location.
    #[inline]
    pub fn with_lat_lon<T>(mut self, coords: T) -> Self
    where
        Option<(f64, f64)>: From<T>,
    {
        self.location = Option::from(coords);
        self
    }

    /// Builder method to add elevation.
    #[inline]
    pub fn with_elevation<T>(mut self, elev: T) -> Self
    where
        Optioned<f64>: From<T>,
    {
        self.elevation = Optioned::from(elev);
        self
    }

    /// station number, USAF number, eg 727730
    #[inline]
    pub fn station_num(&self) -> Optioned<i32> {
        self.num
    }

    /// Latitude and longitude.
    #[inline]
    pub fn location(&self) -> Option<(f64, f64)> {
        self.location
    }

    /// Elevation in meters, this may be in model terrain, not necessarily the same as
    /// the real world.
    #[inline]
    pub fn elevation(&self) -> Optioned<f64> {
        self.elevation
    }
}
