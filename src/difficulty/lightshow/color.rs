use crate::difficulty::lightshow::DistributionType;
use crate::difficulty::lightshow::easing::Easing;
use crate::difficulty::lightshow::filter::Filter;
use crate::utils::LooseBool;
use crate::{impl_get_beat_offset, impl_timed, loose_enum};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(
    feature = "bevy_reflect",
    derive(bevy_reflect::Reflect),
    reflect(Debug, Clone, PartialEq)
)]
pub struct ColorEventBox {
    #[serde(rename = "b")]
    pub beat: f32,
    #[serde(rename = "g")]
    pub group_id: i32,
    #[serde(rename = "e")]
    pub groups: Vec<ColorEventGroup>,
}

impl_timed!(ColorEventBox::beat);

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
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
    #[serde(rename = "w")]
    pub beat_dist_value: f32,
    #[serde(rename = "t")]
    pub bright_dist_type: DistributionType,
    #[serde(rename = "r")]
    pub bright_dist_value: f32,
    #[serde(rename = "b")]
    pub bright_dist_effect_first: LooseBool,
    /// Only present in difficulty file V3.2 or higher.
    #[serde(rename = "i")]
    pub bright_dist_easing: Option<Easing>,
    #[serde(rename = "e")]
    pub data: Vec<ColorEventData>,
}

impl_get_beat_offset!(ColorEventGroup);

impl ColorEventGroup {
    pub fn get_brightness_offset(&self, light_id: i32, group_size: i32) -> f32 {
        self.bright_dist_type.compute_offset(
            light_id,
            group_size,
            &self.filter,
            self.bright_dist_value,
            self.data.last().map(|data| data.beat_offset),
            self.bright_dist_easing,
        )
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(
    feature = "bevy_reflect",
    derive(bevy_reflect::Reflect),
    reflect(Debug, Clone, PartialEq)
)]
pub struct ColorEventData {
    #[serde(rename = "b")]
    pub beat_offset: f32,
    #[serde(rename = "i")]
    pub transition_type: ColorTransitionType,
    #[serde(rename = "c")]
    pub color: LightColor,
    #[serde(rename = "s")]
    pub brightness: f32,
    #[serde(rename = "f")]
    pub strobe_frequency: i32,
}

loose_enum! {
    #[derive(Default, Copy)]
    ColorTransitionType: i32 {
        /// Replaced with `Transition` and [`Easing::None`] in difficulty file V3.2 or higher.
        Instant = 0,
        #[default]
        Transition = 1,
        Extend = 2,
    }
}

loose_enum! {
    #[derive(Default, Copy)]
    LightColor: i32 {
        #[default]
        Primary = 0,
        Secondary = 1,
        White = 2,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difficulty::lightshow::filter::FilterType;

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
}
