//! The advanced group lighting system events.

pub mod color;
pub mod rotation;
pub mod translation;

pub use color::*;
pub use rotation::*;
pub use translation::*;

use crate::difficulty::lightshow::filter::Filter;
use crate::timing_traits::Timed;

/// A collection of [`EventGroup`]s that share the same group ID and beat.
pub trait EventBox: Timed {
    type Group: EventGroup<Data = Self::Data>;
    type Data: EventData;

    fn get_groups(&self) -> &Vec<Self::Group>;
}

#[macro_export]
#[doc(hidden)]
macro_rules! impl_event_box {
    ($ident:ident, $group:ident, $data:ident) => {
        impl crate::difficulty::lightshow::group::EventBox for $ident {
            type Group = $group;
            type Data = $data;

            fn get_groups(&self) -> &Vec<Self::Group> {
                &self.groups
            }
        }
    };
}

/// A collection of [`EventData`] that share the same [`Filter`] and distribution.
pub trait EventGroup {
    type Data: EventData;

    fn get_filter(&self) -> &Filter;
    fn get_data(&self) -> &Vec<Self::Data>;

    /// Returns the number of beats that the event will be offset for a given light ID.
    /// # Panics
    /// Will panic if the light ID is greater than or equal to the group size.
    #[deprecated(note = "Experimental. Does not consider random or limit in filter calculations.")]
    fn get_beat_offset(&self, light_id: i32, group_size: i32) -> f32;

    /// Returns the value (i.e. brightness) that the event will be offset for a given light ID.
    /// # Panics
    /// Will panic if the light ID is greater than or equal to the group size.
    #[deprecated(note = "Experimental. Does not consider random or limit in filter calculations.")]
    fn get_value_offset(&self, light_id: i32, group_size: i32) -> f32;
}

#[macro_export]
#[doc(hidden)]
macro_rules! impl_event_group {
    ($ident:ident::$value_offset:ident, $data:ident) => {
        impl crate::difficulty::lightshow::group::EventGroup for $ident {
            type Data = $data;

            fn get_filter(&self) -> &Filter {
                &self.filter
            }

            fn get_data(&self) -> &Vec<Self::Data> {
                &self.data
            }

            #[allow(deprecated)]
            fn get_beat_offset(&self, light_id: i32, group_size: i32) -> f32 {
                self.beat_dist_type.compute_offset(
                    light_id,
                    group_size,
                    &self.filter,
                    self.beat_dist_value,
                    self.data.last().map(|data| data.beat_offset),
                    None,
                )
            }

            #[allow(deprecated)]
            fn get_value_offset(&self, light_id: i32, group_size: i32) -> f32 {
                self.$value_offset(light_id, group_size)
            }
        }
    };
}

/// The lowest-level group event type, which determines the base value of the event.
pub trait EventData {
    /// Returns the number of beats the event will be offset from the [`EventBox`]'s beat.
    fn get_beat_offset(&self) -> f32;
}

#[macro_export]
#[doc(hidden)]
macro_rules! impl_event_data {
    ($ident:ident) => {
        impl crate::difficulty::lightshow::group::EventData for $ident {
            fn get_beat_offset(&self) -> f32 {
                self.beat_offset
            }
        }
    };
}
