//! Taken from [this example](https://github.com/AlephCubed/loose_enum/blob/main/examples/loose_bool.rs).

use core::error::Error;
use core::fmt::{Display, Formatter};
use loose_enum::loose_enum;

loose_enum! {
    /// An integer repr bool, with 0 being false and 1 being true. Any other value will be saved as Undefined.
    #[derive(Default, Debug, Clone, Copy, Eq, PartialEq, Hash)]
    #[cfg_attr(
        feature = "bevy_reflect",
        derive(bevy_reflect::Reflect),
        reflect(Debug, Clone, PartialEq)
    )]
    pub enum LooseBool: i32 {
        /// A falsy value of zero.
        #[default]
        False = 0,
        /// A truthy value of one.
        True = 1,
    }
}

impl LooseBool {
    /// Returns true if the value is [`True`](Self::True).
    pub fn is_true(&self) -> bool {
        matches!(self, Self::True)
    }

    /// Returns true if the value is [`False`](Self::False).
    pub fn is_false(&self) -> bool {
        matches!(self, Self::False)
    }
}

impl From<bool> for LooseBool {
    fn from(value: bool) -> Self {
        match value {
            true => Self::True,
            false => Self::False,
        }
    }
}

impl TryFrom<LooseBool> for bool {
    type Error = UndefinedBoolError;

    fn try_from(value: LooseBool) -> Result<Self, Self::Error> {
        match value {
            LooseBool::False => Ok(false),
            LooseBool::True => Ok(true),
            LooseBool::Undefined(_) => Err(UndefinedBoolError),
        }
    }
}

/// Error returned when attempting to convert a [`LooseBool::Undefined`] into a `bool`.
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct UndefinedBoolError;

impl Display for UndefinedBoolError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "Cannot convert `LooseBool::Undefined` into `bool`.")
    }
}

impl Error for UndefinedBoolError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loose_to_bool() {
        assert_eq!(bool::try_from(LooseBool::True), Ok(true));
        assert_eq!(bool::try_from(LooseBool::False), Ok(false));

        for i in 2..256 {
            assert_eq!(
                bool::try_from(LooseBool::Undefined(i)),
                Err(UndefinedBoolError),
                "Failed for i={i}"
            );
        }
    }

    #[test]
    fn bool_to_loose() {
        assert_eq!(LooseBool::from(false), LooseBool::False);
        assert_eq!(LooseBool::from(true), LooseBool::True);
    }
}
