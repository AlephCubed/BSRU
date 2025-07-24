#![doc = include_str!("../README.md")]

pub mod difficulty;
pub mod info;
pub mod timing_traits;

mod utils;

#[doc(hidden)]
pub use difficulty::*;
#[doc(hidden)]
pub use info::*;
#[doc(hidden)]
pub use timing_traits::*;
/// An integer repr bool, with 0 being false and 1 being true. Any other value will be saved as `Unknown`.
pub use utils::LooseBool;
