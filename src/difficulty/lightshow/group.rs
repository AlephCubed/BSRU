//! The advanced group lighting system events.

pub mod color;
pub mod fx;
pub mod rotation;
pub mod translation;

#[doc(hidden)]
pub use color::*;
#[doc(hidden)]
pub use fx::*;
#[doc(hidden)]
pub use rotation::*;
#[doc(hidden)]
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
    #[deprecated(note = "Experimental. Does not consider random in filter calculations.")]
    fn get_beat_offset(&self, light_id: i32, group_size: i32) -> f32;

    /// Returns the value (i.e. brightness) that the event will be offset for a given light ID.
    /// # Panics
    /// Will panic if the light ID is greater than or equal to the group size.
    #[deprecated(note = "Experimental. Does not consider random in filter calculations.")]
    fn get_value_offset(&self, light_id: i32, group_size: i32) -> f32;

    /// Returns the duration of the group in beats.
    /// # Undefined
    /// If the [`FilterType`] is `Undefined` then the result will be zero.
    #[deprecated(note = "Experimental. Does not consider random in filter calculations.")]
    fn get_duration(&self, group_size: i32) -> f32;
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
                self.beat_dist_type.compute_beat_offset(
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

            #[allow(deprecated)]
            fn get_duration(&self, group_size: i32) -> f32 {
                let filtered_size = self.filter.count_filtered_without_limit(group_size);

                if filtered_size == 0 {
                    return 0.0;
                }

                let Some(data) = self.get_data().last() else {
                    return 0.0;
                };

                match self.beat_dist_type {
                    DistributionType::Wave => {
                        if let Some(limit_behaviour) = self.filter.limit_behaviour
                            && !limit_behaviour.beat_enabled()
                            && let Some(limit_percent) = self.filter.limit_percent
                            && limit_percent != 0.0
                        {
                            (self.beat_dist_value * limit_percent).max(data.beat_offset)
                        } else {
                            self.beat_dist_value.max(data.beat_offset)
                        }
                    }
                    DistributionType::Step => {
                        data.beat_offset + self.beat_dist_value * filtered_size as f32
                    }
                    DistributionType::Undefined(_) => 0.0,
                }
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

#[allow(deprecated)]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DistributionType, LimitBehaviour};

    #[test]
    fn get_duration_no_distribution() {
        assert_eq!(ColorEventGroup::default().get_duration(12), 0.0);
    }

    #[test]
    fn get_duration_wave() {
        let group = ColorEventGroup {
            beat_dist_type: DistributionType::Wave,
            beat_dist_value: 12.0,
            ..Default::default()
        };

        assert_eq!(group.get_duration(12), 12.0);
    }

    #[test]
    fn get_duration_step() {
        let group = ColorEventGroup {
            beat_dist_type: DistributionType::Step,
            beat_dist_value: 1.0,
            ..Default::default()
        };

        assert_eq!(group.get_duration(12), 12.0);
    }

    #[test]
    fn get_duration_wave_with_limit() {
        let group = ColorEventGroup {
            filter: Filter {
                limit_behaviour: Some(LimitBehaviour::None),
                limit_percent: Some(0.5),
                ..Default::default()
            },
            beat_dist_type: DistributionType::Wave,
            beat_dist_value: 12.0,
            ..Default::default()
        };

        assert_eq!(group.get_duration(12), 6.0);
    }

    #[test]
    fn get_duration_step_with_limit() {
        let group = ColorEventGroup {
            filter: Filter {
                limit_behaviour: Some(LimitBehaviour::None),
                limit_percent: Some(0.5),
                ..Default::default()
            },
            beat_dist_type: DistributionType::Step,
            beat_dist_value: 1.0,
            ..Default::default()
        };

        assert_eq!(group.get_duration(12), 12.0);
    }

    #[test]
    fn get_duration_wave_with_limit_adjusted() {
        let group = ColorEventGroup {
            filter: Filter {
                limit_behaviour: Some(LimitBehaviour::Beat),
                limit_percent: Some(0.5),
                ..Default::default()
            },
            beat_dist_type: DistributionType::Wave,
            beat_dist_value: 12.0,
            ..Default::default()
        };

        assert_eq!(group.get_duration(12), 12.0);
    }

    #[test]
    fn get_duration_step_with_limit_adjusted() {
        let group = ColorEventGroup {
            filter: Filter {
                limit_behaviour: Some(LimitBehaviour::Beat),
                limit_percent: Some(0.5),
                ..Default::default()
            },
            beat_dist_type: DistributionType::Step,
            beat_dist_value: 1.0,
            ..Default::default()
        };

        assert_eq!(group.get_duration(12), 12.0);
    }

    #[test]
    fn get_duration_step_with_limit_zero() {
        let group = ColorEventGroup {
            filter: Filter {
                limit_percent: Some(0.0),
                ..Default::default()
            },
            beat_dist_type: DistributionType::Step,
            beat_dist_value: 1.0,
            ..Default::default()
        };

        assert_eq!(group.get_duration(12), 12.0);
    }
}
