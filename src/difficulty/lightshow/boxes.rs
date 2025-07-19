pub mod color;
pub mod rotation;
pub mod translation;

use crate::difficulty::lightshow::filter::Filter;
use crate::timing_traits::Timed;

pub trait EventBox: Timed {
    type Group: EventGroup<Data = Self::Data>;
    type Data: EventData;

    fn get_groups(&self) -> &Vec<Self::Group>;
}

#[macro_export]
macro_rules! impl_event_box {
    ($ident:ident, $group:ident, $data:ident) => {
        impl crate::difficulty::lightshow::boxes::EventBox for $ident {
            type Group = $group;
            type Data = $data;

            fn get_groups(&self) -> &Vec<Self::Group> {
                &self.groups
            }
        }
    };
}

pub trait EventGroup {
    type Data: EventData;

    fn get_filter(&self) -> &Filter;
    fn get_data(&self) -> &Vec<Self::Data>;

    /// Returns the number of beats that the event will be offset for a given light ID.
    /// # Panics
    /// Will panic if the light ID is greater than or equal to the group size.
    fn get_beat_offset(&self, light_id: i32, group_size: i32) -> f32;

    /// Returns the value that the event will be offset for a given light ID (i.e. brightness offset).
    /// # Panics
    /// Will panic if the light ID is greater than or equal to the group size.
    fn get_value_offset(&self, light_id: i32, group_size: i32) -> f32;
}

#[macro_export]
macro_rules! impl_event_group {
    ($ident:ident::$value_offset:ident, $data:ident) => {
        impl crate::difficulty::lightshow::boxes::EventGroup for $ident {
            type Data = $data;

            fn get_filter(&self) -> &Filter {
                &self.filter
            }

            fn get_data(&self) -> &Vec<Self::Data> {
                &self.data
            }

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

            fn get_value_offset(&self, light_id: i32, group_size: i32) -> f32 {
                self.$value_offset(light_id, group_size)
            }
        }
    };
}

pub trait EventData {
    fn get_beat_offset(&self) -> f32;
}

#[macro_export]
macro_rules! impl_event_data {
    ($ident:ident) => {
        impl crate::difficulty::lightshow::boxes::EventData for $ident {
            fn get_beat_offset(&self) -> f32 {
                self.beat_offset
            }
        }
    };
}
