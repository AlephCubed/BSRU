//! Events that control the color of lights.

use crate::difficulty::lightshow::DistributionType;
use crate::difficulty::lightshow::easing::Easing;
use crate::difficulty::lightshow::filter::Filter;
use crate::utils::LooseBool;
use crate::{impl_event_box, impl_event_data, impl_event_group, impl_timed, loose_enum};
use serde::{Deserialize, Serialize};

/// A collection of [`ColorEventGroup`]s that share the same group ID and beat.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(
    feature = "bevy_reflect",
    derive(bevy_reflect::Reflect),
    reflect(Debug, Clone, PartialEq)
)]
pub struct ColorEventBox {
    /// The time the event takes place.
    #[serde(rename = "b")]
    pub beat: f32,
    /// The ID of the collection of objects that this event effects.
    #[serde(rename = "g")]
    pub group_id: i32,
    #[serde(rename = "e")]
    pub groups: Vec<ColorEventGroup>,
}

impl Default for ColorEventBox {
    fn default() -> Self {
        Self {
            beat: 0.0,
            group_id: 0,
            groups: vec![ColorEventGroup::default()],
        }
    }
}

impl_timed!(ColorEventBox::beat);
impl_event_box!(ColorEventBox, ColorEventGroup, ColorEventData);

/// A collection of [`ColorEventData`] that share the same [`Filter`] and distribution.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(
    feature = "bevy_reflect",
    derive(bevy_reflect::Reflect),
    reflect(Debug, Clone, PartialEq)
)]
pub struct ColorEventGroup {
    #[serde(rename = "f")]
    pub filter: Filter,
    #[serde(rename = "d")]
    pub beat_dist_type: DistributionType,
    /// The strength of the beat distribution. Dependent on the [distribution type](Self::beat_dist_type).
    ///
    /// A value of zero will have no effect.
    #[serde(rename = "w")]
    pub beat_dist_value: f32,

    #[serde(rename = "t")]
    pub bright_dist_type: DistributionType,
    /// The strength of the brightness distribution. Dependent on the [distribution type](Self::bright_dist_type).
    ///
    /// A value of zero will have no effect.
    #[serde(rename = "r")]
    pub bright_dist_value: f32,
    /// Whether the first [`ColorEventData`] of the group will be effected by brightness distribution.
    #[serde(rename = "b")]
    pub bright_dist_effect_first: LooseBool,
    /// > Only present in difficulty file V3.2 or higher.
    #[serde(rename = "i")]
    pub bright_dist_easing: Option<Easing>,
    #[serde(rename = "e")]
    pub data: Vec<ColorEventData>,
}

impl Default for ColorEventGroup {
    fn default() -> Self {
        Self {
            filter: Default::default(),
            beat_dist_type: Default::default(),
            beat_dist_value: 0.0,
            bright_dist_type: Default::default(),
            bright_dist_value: 0.0,
            bright_dist_effect_first: Default::default(),
            bright_dist_easing: Some(Easing::Linear),
            data: vec![ColorEventData::default()],
        }
    }
}

impl_event_group!(ColorEventGroup::get_brightness_offset, ColorEventData);

impl ColorEventGroup {
    /// Returns the brightness that the event will be offset for a given light ID.
    /// # Panics
    /// Will panic if the light ID is greater than or equal to the group size.
    #[deprecated(note = "Experimental. Does not consider random in filter calculations.")]
    #[allow(deprecated)]
    pub fn get_brightness_offset(&self, light_id: i32, group_size: i32) -> f32 {
        self.bright_dist_type.compute_value_offset(
            light_id,
            group_size,
            &self.filter,
            self.bright_dist_value,
            self.data.last().map(|data| data.beat_offset),
            self.bright_dist_easing,
        )
    }
}

/// The lowest-level group event type, which determines the color of the event.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(
    feature = "bevy_reflect",
    derive(bevy_reflect::Reflect),
    reflect(Debug, Clone, PartialEq)
)]
pub struct ColorEventData {
    /// The number of beats the event will be offset from the [`ColorEventBox`]'s beat.
    #[serde(rename = "b")]
    pub beat_offset: f32,
    #[serde(rename = "i")]
    pub transition_type: ColorTransitionType,
    #[serde(rename = "c")]
    pub color: LightColor,
    /// Controls how bright the light is, with zero being off and one being normal brightness.
    #[serde(rename = "s")]
    pub brightness: f32,
    /// Determines the number of strobes that will take place each beat.
    /// A value of zero will result in no strobing.
    #[serde(rename = "f")]
    pub strobe_frequency: i32,
    /// > Only present in difficulty file V3.3 or higher.
    ///
    /// Controls the brightness of the "off" strobe state.
    /// If this equals the event's [brightness](Self::brightness), strobing will have no effect.
    #[serde(rename = "sb")]
    pub strobe_brightness: Option<f32>,
    /// > Only present in difficulty file V3.3 or higher.
    ///
    /// Whether to fade between strobe states or not.
    #[serde(rename = "sf")]
    pub strobe_fade: Option<LooseBool>,
}

impl Default for ColorEventData {
    fn default() -> Self {
        Self {
            beat_offset: 0.0,
            transition_type: Default::default(),
            color: Default::default(),
            brightness: 1.0,
            strobe_frequency: 0,
            strobe_brightness: Some(0.0),
            strobe_fade: Some(LooseBool::False),
        }
    }
}

impl_event_data!(ColorEventData);

loose_enum! {
    /// Controls how the state is changed relative to the previous event.
    #[derive(Default, Copy)]
    ColorTransitionType: i32 {
        /// Unique to color events.
        /// Has the same effect as using [`TransitionType::Transition`](crate::lightshow::TransitionType::Transition)
        /// and [`Easing::None`] in rotation/translation events.
        #[default]
        Instant = 0,
        /// The state will blend from the previous event's state, using the events [easing](Easing).
        Transition = 1,
        /// The event's state will be ignored, replaced with the state from the previous event.
        Extend = 2,
    }
}

loose_enum! {
    /// Controls which color to display, based on a map or environment's [color scheme](crate::info::color_scheme::ColorScheme).
    #[derive(Default, Copy)]
    LightColor: i32 {
        #[default]
        Primary = 0,
        Secondary = 1,
        White = 2,
    }
}

#[allow(deprecated)]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::LimitBehaviour;
    use crate::difficulty::lightshow::filter::FilterType;
    use crate::difficulty::lightshow::group::EventGroup;

    #[test]
    fn beat_wave() {
        let group = ColorEventGroup {
            beat_dist_type: DistributionType::Wave,
            beat_dist_value: 12.0,
            ..Default::default()
        };

        assert!((0..12).all(|i| group.get_beat_offset(i, 12) == i as f32));
    }

    #[test]
    fn beat_step() {
        let group = ColorEventGroup {
            beat_dist_type: DistributionType::Step,
            beat_dist_value: 1.0,
            ..Default::default()
        };

        assert!((0..12).all(|i| group.get_beat_offset(i, 12) == i as f32));
    }

    #[test]
    fn beat_wave_with_division_filter() {
        let group = ColorEventGroup {
            filter: Filter {
                filter_type: FilterType::Division,
                parameter1: 2,
                parameter2: 1,
                ..Default::default()
            },
            beat_dist_type: DistributionType::Wave,
            beat_dist_value: 6.0,
            ..Default::default()
        };

        assert!((0..6).all(|i| group.get_beat_offset(i + 6, 12) == i as f32));
    }

    #[test]
    fn brightness_wave_with_division_filter() {
        let group = ColorEventGroup {
            filter: Filter {
                filter_type: FilterType::Division,
                parameter1: 2,
                parameter2: 1,
                ..Default::default()
            },
            bright_dist_type: DistributionType::Wave,
            bright_dist_value: 6.0,
            ..Default::default()
        };

        assert!((0..6).all(|i| group.get_brightness_offset(i + 6, 12) == i as f32));
    }

    #[test]
    fn beat_step_with_division_filter() {
        let group = ColorEventGroup {
            filter: Filter {
                filter_type: FilterType::Division,
                parameter1: 2,
                parameter2: 1,
                ..Default::default()
            },
            beat_dist_type: DistributionType::Step,
            beat_dist_value: 1.0,
            ..Default::default()
        };

        assert!((0..6).all(|i| group.get_beat_offset(i + 6, 12) == i as f32));
    }

    #[test]
    fn brightness_step_with_division_filter() {
        let group = ColorEventGroup {
            filter: Filter {
                filter_type: FilterType::Division,
                parameter1: 2,
                parameter2: 1,
                ..Default::default()
            },
            bright_dist_type: DistributionType::Step,
            bright_dist_value: 1.0,
            ..Default::default()
        };

        assert!((0..6).all(|i| group.get_brightness_offset(i + 6, 12) == i as f32));
    }

    #[test]
    fn beat_wave_with_step_filter() {
        let group = ColorEventGroup {
            filter: Filter {
                filter_type: FilterType::StepAndOffset,
                parameter1: 0,
                parameter2: 2,
                ..Default::default()
            },
            beat_dist_type: DistributionType::Wave,
            beat_dist_value: 6.0,
            ..Default::default()
        };

        assert!((0..6).all(|i| group.get_beat_offset(i * 2, 12) == i as f32));
    }

    #[test]
    fn brightness_wave_with_step_filter() {
        let group = ColorEventGroup {
            filter: Filter {
                filter_type: FilterType::StepAndOffset,
                parameter1: 0,
                parameter2: 2,
                ..Default::default()
            },
            bright_dist_type: DistributionType::Wave,
            bright_dist_value: 6.0,
            ..Default::default()
        };

        assert!((0..6).all(|i| group.get_brightness_offset(i * 2, 12) == i as f32));
    }

    #[test]
    fn beat_step_with_step_filter() {
        let group = ColorEventGroup {
            filter: Filter {
                filter_type: FilterType::StepAndOffset,
                parameter1: 0,
                parameter2: 2,
                ..Default::default()
            },
            beat_dist_type: DistributionType::Step,
            beat_dist_value: 1.0,
            ..Default::default()
        };

        assert!((0..6).all(|i| group.get_beat_offset(i * 2, 12) == i as f32));
    }

    #[test]
    fn brightness_step_with_step_filter() {
        let group = ColorEventGroup {
            filter: Filter {
                filter_type: FilterType::StepAndOffset,
                parameter1: 0,
                parameter2: 2,
                ..Default::default()
            },
            bright_dist_type: DistributionType::Step,
            bright_dist_value: 1.0,
            ..Default::default()
        };

        assert!((0..6).all(|i| group.get_brightness_offset(i * 2, 12) == i as f32));
    }

    #[test]
    fn beat_wave_with_reverse_filter() {
        let group = ColorEventGroup {
            filter: Filter {
                reverse: LooseBool::True,
                ..Default::default()
            },
            beat_dist_type: DistributionType::Wave,
            beat_dist_value: 12.0,
            ..Default::default()
        };

        assert!((0..12).all(|i| group.get_beat_offset(i, 12) == 12.0 - i as f32));
    }

    #[test]
    fn brightness_wave_with_reverse_filter() {
        let group = ColorEventGroup {
            filter: Filter {
                reverse: LooseBool::True,
                ..Default::default()
            },
            bright_dist_type: DistributionType::Wave,
            bright_dist_value: 12.0,
            ..Default::default()
        };

        assert!((0..12).all(|i| group.get_brightness_offset(i, 12) == 12.0 - i as f32));
    }

    #[test]
    fn beat_step_with_reverse_filter() {
        let group = ColorEventGroup {
            filter: Filter {
                reverse: LooseBool::True,
                ..Default::default()
            },
            beat_dist_type: DistributionType::Step,
            beat_dist_value: 1.0,
            ..Default::default()
        };

        assert!((0..12).all(|i| group.get_beat_offset(i, 12) == 12.0 - i as f32));
    }

    #[test]
    fn brightness_step_with_reverse_filter() {
        let group = ColorEventGroup {
            filter: Filter {
                reverse: LooseBool::True,
                ..Default::default()
            },
            bright_dist_type: DistributionType::Step,
            bright_dist_value: 1.0,
            ..Default::default()
        };

        assert!((0..12).all(|i| group.get_brightness_offset(i, 12) == 12.0 - i as f32));
    }

    #[test]
    fn beat_wave_with_limit_filter() {
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

        assert!((0..6).all(|i| group.get_beat_offset(i, 12) == (i * 2) as f32));
    }

    #[test]
    fn brightness_wave_with_limit_filter() {
        let group = ColorEventGroup {
            filter: Filter {
                limit_behaviour: Some(LimitBehaviour::Value),
                limit_percent: Some(0.5),
                ..Default::default()
            },
            bright_dist_type: DistributionType::Wave,
            bright_dist_value: 12.0,
            ..Default::default()
        };

        assert!((0..6).all(|i| group.get_brightness_offset(i, 12) == (i * 2) as f32));
    }
}
