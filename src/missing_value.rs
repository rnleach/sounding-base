//! A quantity that has a certain value that indicates missing or invalid data.
//!
//! Semantically this is no different than the `Option` type provided by the standard library.
//! However, that type uses an enum, which takes extra space as opposed to using a special value
//! to indicate missing or invalid data.

/// Defines the numeric value used to indicate missing data.
pub trait MissingData<T>: Default
where
    T: PartialEq + Copy,
{
    /// The value that indicates this quantity is invalid or missing.
    const MISSING: T;
}

/// A newtype to wrap a type and implement the MissingData trait.
#[derive(Clone, Copy)]
pub struct OptionVal<T: PartialEq + Copy + MissingData<T>> {
    value: T,
}

impl<T> OptionVal<T>
where
    T: PartialEq + Copy + MissingData<T>,
{
    /// Convert it to an option. The into trait should cover this, but the compiler has troubles
    /// inferring the type, so this function is provided. The Into trait is still useful in contexts
    /// where the compiler can infer the type correctly, like the body of this function.
    pub fn as_option(self) -> Option<T> {
        self.into()
    }

    /// Despite its name, this method does not panic, it just returns the interior value even if
    /// that value is the 'Missing Data' flag value.
    pub fn unwrap(self) -> T {
        self.value
    }
}

impl<T> Into<Option<T>> for OptionVal<T>
where
    T: PartialEq + Copy + MissingData<T>,
{
    fn into(self) -> Option<T> {
        if self.value == T::MISSING {
            None
        } else {
            Some(self.value)
        }
    }
}

impl<T> Default for OptionVal<T>
where
    T: PartialEq + Copy + MissingData<T>,
{
    fn default() -> Self {
        OptionVal::from(T::MISSING)
    }
}

impl<T> From<Option<T>> for OptionVal<T>
where
    T: PartialEq + Copy + MissingData<T>,
{
    fn from(src: Option<T>) -> OptionVal<T> {
        if let Some(val) = src {
            OptionVal::from(val)
        } else {
            OptionVal::from(T::MISSING)
        }
    }
}

impl<T> From<T> for OptionVal<T>
where
    T: PartialEq + Copy + MissingData<T>,
{
    fn from(src: T) -> OptionVal<T> {
        OptionVal { value: src }
    }
}

impl MissingData<f32> for f32 {
    const MISSING: f32 = -9999.0;
}

impl MissingData<i32> for i32 {
    const MISSING: i32 = -9999;
}
