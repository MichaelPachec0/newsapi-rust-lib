use std::convert::TryInto;
use std::num::TryFromIntError;
use std::time::Duration;

/// Try and convert a type that implements the `TryFrom<u64>` to a duration by method chaining.
/// the function `to_duration` returns a `Result` which allows for ? use.
pub trait ToDuration<T>
where
    u64: TryFrom<T>,
{
    fn to_duration(self) -> Result<Duration, <u64 as TryFrom<T>>::Error>;
}

impl<T> ToDuration<T> for T
where
    u64: TryFrom<T>,
{
    fn to_duration(self) -> Result<Duration, <u64 as TryFrom<T>>::Error> {
        Ok(Duration::from_secs(u64::try_from(self)?))
    }
}

pub fn optional_value<'a, T>(value: &'a Option<T>, default: &'a T) -> &'a T {
    match value {
        Some(ref value) => value,
        None => default,
    }
}
