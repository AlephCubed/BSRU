//! Controls which light IDs are affected by an event.

use crate::loose_enum;
use crate::utils::LooseBool;
use serde::{Deserialize, Serialize};

/// Controls which light IDs are affected by an event.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(
    feature = "bevy_reflect",
    derive(bevy_reflect::Reflect),
    reflect(Debug, Clone, PartialEq)
)]
pub struct Filter {
    // V3.0:
    /// Controls how [`parameter1`](Self::parameter1) and [`parameter2`](Self::parameter2) are used.
    #[serde(rename = "f")]
    pub filter_type: FilterType,
    /// Dependent on the [`FilterType`].
    #[serde(rename = "p")]
    pub parameter1: i32,
    /// Dependent on the [`FilterType`].
    #[serde(rename = "t")]
    pub parameter2: i32,
    /// If true, the filter will start at the end of a group and work backwards.
    #[serde(rename = "r")]
    pub reverse: LooseBool,
    // V3.1:
    /// > Only present in difficulty file V3.1 or higher.
    ///
    /// Chunks will divide the group into multiple chunks, which will each behave as a single object.
    ///
    /// To see this in practice, check out [this video](https://youtube.com/watch?v=NJPPBvyHJjg&t=197).
    #[serde(rename = "c")]
    pub chunks: Option<i32>,
    /// > Only present in difficulty file V3.1 or higher.
    #[serde(rename = "n")]
    pub random_behaviour: Option<RandomBehaviour>,
    /// > Only present in difficulty file V3.1 or higher.
    #[serde(rename = "s")]
    pub random_seed: Option<i32>,
    /// > Only present in difficulty file V3.1 or higher.
    ///
    /// Determines how [the limit](Filter::limit_percent) behaves. This is applied *after* the [`FilterType`] behaviour.
    ///
    /// To see this in practice, check out [this video](https://youtube.com/watch?v=NJPPBvyHJjg&t=338).
    #[serde(rename = "d")]
    pub limit_behaviour: Option<LimitBehaviour>,
    /// > Only present in difficulty file V3.1 or higher.
    ///
    /// A value from 0.0 to 1.0 which represents the percent of lights that will be effected,
    /// and the behaviour is dependent on [`LimitBehaviour`].
    #[serde(rename = "l")]
    pub limit_percent: Option<f32>,
}

impl Default for Filter {
    fn default() -> Self {
        Self {
            filter_type: FilterType::default(),
            parameter1: 1,
            parameter2: 0,
            reverse: LooseBool::False,
            chunks: Some(0),
            random_behaviour: Some(RandomBehaviour::None),
            random_seed: Some(0),
            limit_behaviour: Some(LimitBehaviour::None),
            limit_percent: Some(1.0),
        }
    }
}

impl Filter {
    /// Returns true if the light ID is in the filter.
    /// # Unknown
    /// If the [`FilterType`] is `Unknown` then the result will be `true`.
    /// # Panics
    /// Will panic if the light ID is greater than or equal to the group size.
    #[must_use]
    #[inline]
    #[deprecated(
        note = "Experimental. Does not consider chunks, random, or limit in calculations."
    )]
    pub fn is_in_filter(&self, mut light_id: i32, mut group_size: i32) -> bool {
        assert!(light_id < group_size);

        if self.reverse.is_true() {
            light_id = group_size - light_id - 1;
        }

        if let Some(chunks) = self.chunks
            && chunks > 0
        {
            light_id /= group_size / chunks;
            group_size /= group_size / chunks;
        }

        match self.filter_type {
            FilterType::Division => {
                let start = self.parameter2 * group_size / self.parameter1.max(1);
                let end = (self.parameter2 + 1) * group_size / self.parameter1.max(1);
                light_id >= start && light_id < end.max(start + 1)
            }
            FilterType::StepAndOffset => {
                let offset_light_id = light_id - self.parameter1;
                offset_light_id % self.parameter2.max(1) == 0 && offset_light_id >= 0
            }
            FilterType::Unknown(_) => true,
        }
    }

    /// Returns the number of light chunks effected by the filter.
    /// # Unknown
    /// If the [`FilterType`] is `Unknown` then the result will be the same as `group_size`.
    #[must_use]
    #[inline]
    #[deprecated(
        note = "Experimental. Does not consider chunks, random, or limit in calculations."
    )]
    pub fn count_filtered(&self, mut group_size: i32) -> i32 {
        if let Some(chunks) = self.chunks
            && chunks > 0
        {
            group_size /= group_size / chunks;
        }

        match self.filter_type {
            FilterType::Division => {
                let start = self.parameter2 * group_size / self.parameter1.max(1);
                let end = (self.parameter2 + 1) * group_size / self.parameter1.max(1);
                end.max(start + 1) - start
            }
            FilterType::StepAndOffset => {
                group_size / self.parameter2.max(1) - self.parameter1 / self.parameter2.max(1)
            }
            FilterType::Unknown(_) => group_size,
        }
    }

    #[allow(deprecated)]
    /// Returns the light chunk ID relative to the [filtered count](Self::count_filtered).
    /// # Unknown
    /// If the [`FilterType`] is `Unknown` then the result will be the same as `light_id`.
    /// # Panics
    /// Will panic if the light ID is greater than or equal to the group size.
    #[must_use]
    #[inline]
    #[deprecated(
        note = "Experimental. Does not consider chunks, random, or limit in calculations."
    )]
    pub fn get_relative_index(&self, mut light_id: i32, mut group_size: i32) -> i32 {
        assert!(light_id < group_size);

        if self.reverse.is_true() {
            light_id = group_size - light_id;
        }

        if let Some(chunks) = self.chunks
            && chunks > 0
        {
            light_id /= group_size / chunks;
            group_size /= group_size / chunks;
        }

        match self.filter_type {
            FilterType::Division => {
                let start = self.parameter2 * group_size / self.parameter1.max(1);
                light_id - start
            }
            FilterType::StepAndOffset => {
                let offset_light_id = light_id - self.parameter1;
                offset_light_id / self.parameter2.max(1)
            }
            FilterType::Unknown(_) => group_size,
        }
    }
}

loose_enum! {
    /// Controls how a [`Filter`]'s [`parameter1`](Filter::parameter1)
    /// and [`parameter2`](Filter::parameter2) values are used.
    #[derive(Default, Copy)]
    FilterType: i32 {
        /// Splits the group up into equal sections and selects one.
        /// - [`parameter1`](Filter::parameter1) determines the number of sections.
        ///   It will be rounded up to the nearest multiple of the group size.
        /// - [`parameter2`](Filter::parameter2) determines the section to select, starting at 0.
        #[default]
        Division = 1,
        /// Alternates selecting and not selecting lights.
        /// - [`parameter1`](Filter::parameter1) is the index of the first light that will be selected, starting at 0.
        /// - [`parameter2`](Filter::parameter2) determines the number of IDs to move forward before selecting another light.
        StepAndOffset = 2,
    }
}

loose_enum!(
    #[derive(Default, Copy)]
    RandomBehaviour: i32 {
        #[default]
        None = 0,
        KeepOrder = 1,
        RandomElements = 2,
    }
);

loose_enum!(
    #[derive(Default, Copy)]
    LimitBehaviour: i32 {
        #[default]
        None = 0,
        Duration = 1,
        Distribution = 2,
    }
);

#[allow(deprecated)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn division_first_half() {
        let filter = Filter {
            filter_type: FilterType::Division,
            parameter1: 2,
            parameter2: 0,
            ..Default::default()
        };

        assert!((0..6).all(|i| filter.is_in_filter(i, 12)));
        assert!((6..12).all(|i| !filter.is_in_filter(i, 12)));
        assert_eq!(filter.count_filtered(12), 6);
        assert!((0..6).all(|i| filter.get_relative_index(i, 12) == i));
    }

    #[test]
    fn division_second_half() {
        let filter = Filter {
            filter_type: FilterType::Division,
            parameter1: 2,
            parameter2: 1,
            ..Default::default()
        };

        assert!((0..6).all(|i| !filter.is_in_filter(i, 12)));
        assert!((6..12).all(|i| filter.is_in_filter(i, 12)));
        assert_eq!(filter.count_filtered(12), 6);
        assert!((6..12).all(|i| filter.get_relative_index(i, 12) == i - 6));
    }

    #[test]
    fn division_first_half_rev() {
        let filter = Filter {
            filter_type: FilterType::Division,
            parameter1: 2,
            parameter2: 0,
            reverse: LooseBool::True,
            ..Default::default()
        };

        assert!((0..6).all(|i| !filter.is_in_filter(i, 12)));
        assert!((6..12).all(|i| filter.is_in_filter(i, 12)));
        assert_eq!(filter.count_filtered(12), 6);
        assert!((6..12).all(|i| filter.get_relative_index(i, 12) == 12 - i));
    }

    #[test]
    fn division_second_half_rev() {
        let filter = Filter {
            filter_type: FilterType::Division,
            parameter1: 2,
            parameter2: 1,
            reverse: LooseBool::True,
            ..Default::default()
        };

        assert!((0..6).all(|i| filter.is_in_filter(i, 12)));
        assert!((6..12).all(|i| !filter.is_in_filter(i, 12)));
        assert_eq!(filter.count_filtered(12), 6);
        assert!((0..6).all(|i| filter.get_relative_index(i, 12) == 6 - i));
    }

    #[test]
    fn division_select_all() {
        let filter = Filter {
            filter_type: FilterType::Division,
            parameter1: 1,
            parameter2: 0,
            ..Default::default()
        };

        assert!((0..12).all(|i| filter.is_in_filter(i, 12)));
        assert_eq!(filter.count_filtered(12), 12);
        assert!((0..12).all(|i| filter.get_relative_index(i, 12) == i));
    }

    #[test]
    fn division_larger_than_group_size() {
        for i in 0..12 {
            let filter = Filter {
                filter_type: FilterType::Division,
                parameter1: 12,
                parameter2: i,
                ..Default::default()
            };

            let expected_id = match i {
                0 => 0,
                1 => 0,
                2 => 1,
                3 => 2,
                4 => 2,
                5 => 3,
                6 => 4,
                7 => 4,
                8 => 5,
                9 => 6,
                10 => 6,
                11 => 7,
                _ => unreachable!(),
            };

            assert!(filter.is_in_filter(expected_id, 8));
            assert!(
                (0..8)
                    .filter(|x| *x != expected_id)
                    .all(|i| !filter.is_in_filter(i, 8))
            );
            assert_eq!(filter.count_filtered(8), 1);
            assert_eq!(filter.get_relative_index(expected_id, 8), 0);
        }
    }

    #[test]
    fn step_select_all() {
        let filter = Filter {
            filter_type: FilterType::StepAndOffset,
            parameter1: 0,
            parameter2: 1,
            ..Default::default()
        };

        assert!((0..12).all(|i| filter.is_in_filter(i, 12)));
        assert_eq!(filter.count_filtered(12), 12);
        assert!((0..12).all(|i| filter.get_relative_index(i, 12) == i));
    }

    #[test]
    fn step_start_index() {
        for outer in 0..12 {
            let filter = Filter {
                filter_type: FilterType::StepAndOffset,
                parameter1: outer,
                parameter2: 1,
                ..Default::default()
            };

            assert!((0..outer).all(|i| !filter.is_in_filter(i, 12)));
            assert!((outer..12).all(|i| filter.is_in_filter(i, 12)));
            assert_eq!(filter.count_filtered(12), 12 - outer);
            assert!((outer..12).all(|i| filter.get_relative_index(i, 12) == i - outer));
        }
    }

    #[test]
    fn step_every_other() {
        let filter = Filter {
            filter_type: FilterType::StepAndOffset,
            parameter1: 0,
            parameter2: 2,
            ..Default::default()
        };

        for i in 0..12 {
            assert_eq!(filter.is_in_filter(i, 12), i % 2 == 0);

            if i % 2 == 0 {
                assert_eq!(filter.get_relative_index(i, 12), i / 2);
            }
        }
        assert_eq!(filter.count_filtered(12), 6);
    }

    #[test]
    fn step_every_other_offset() {
        let filter = Filter {
            filter_type: FilterType::StepAndOffset,
            parameter1: 1,
            parameter2: 2,
            ..Default::default()
        };

        for i in 0..12 {
            assert_eq!(filter.is_in_filter(i, 12), i % 2 != 0);

            if i % 2 != 0 {
                assert_eq!(filter.get_relative_index(i, 12), i / 2);
            }
        }
        assert_eq!(filter.count_filtered(12), 6);
    }

    #[test]
    fn chunks_of_two() {
        let filter = Filter {
            chunks: Some(6),
            ..Default::default()
        };

        assert!((0..12).all(|i| filter.is_in_filter(i, 12)));
        assert_eq!(filter.count_filtered(12), 6);
        assert!((0..6).all(|i| {
            filter.get_relative_index(i * 2, 12) == i
                && filter.get_relative_index(i * 2 + 1, 12) == i
        }));
    }

    #[test]
    fn chunks_of_six() {
        let filter = Filter {
            chunks: Some(2),
            ..Default::default()
        };

        assert!((0..12).all(|i| filter.is_in_filter(i, 12)));
        assert_eq!(filter.count_filtered(12), 2);
        assert!((0..6).all(|i| filter.get_relative_index(i, 12) == 0));
        assert!((0..6).all(|i| filter.get_relative_index(i + 6, 12) == 1));
    }
}
