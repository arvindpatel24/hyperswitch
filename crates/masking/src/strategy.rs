use core::fmt;

/// Debugging trait which is specialized for handling secret values
pub trait Strategy<T> {
    /// Format information about the secret's type.
    fn fmt(value: &T, fmt: &mut fmt::Formatter<'_>) -> std::fmt::Result;
}

/// Debug with type
pub struct WithType;

impl<T> Strategy<T> for WithType {
    fn fmt(_: &T, fmt: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.write_str("*** ")?;
        fmt.write_str(std::any::type_name::<T>())?;
        fmt.write_str(" ***")
    }
}

/// Debug without type
pub struct WithoutType;

impl<T> Strategy<T> for WithoutType {
    fn fmt(_: &T, fmt: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.write_str("*** ***")
    }
}

pub struct NoMasking;

impl<T> Strategy<T> for NoMasking
where
    T: fmt::Display,
{
    fn fmt(val: &T, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&format!("{}", val))
    }
}