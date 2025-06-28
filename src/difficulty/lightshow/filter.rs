use crate::loose_enum;
use crate::utils::LooseBool;
use serde::{Deserialize, Serialize};

/// Controls which light indices are affected by event boxes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Filter {
    // V3.0:
    #[serde(rename = "f")]
    pub filter_type: FilterType,
    /// Dependent on the [`FilterType`]
    #[serde(rename = "p")]
    pub parameter1: i32,
    /// Dependent on the [`FilterType`]
    #[serde(rename = "t")]
    pub parameter2: i32,
    #[serde(rename = "r")]
    pub reverse: LooseBool,
    // V3.1:
    /// Only present in difficulty file V3.1 or higher.
    #[serde(rename = "c")]
    pub chunks: Option<i32>,
    /// Only present in difficulty file V3.1 or higher.
    #[serde(rename = "n")]
    pub random_behaviour: Option<i32>,
    /// Only present in difficulty file V3.1 or higher.
    #[serde(rename = "s")]
    pub random_seed: Option<i32>,
    /// Only present in difficulty file V3.1 or higher.
    #[serde(rename = "d")]
    pub limit_behaviour: Option<i32>,
    /// Only present in difficulty file V3.1 or higher.
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
            chunks: None,
            random_behaviour: None,
            random_seed: None,
            limit_behaviour: None,
            limit_percent: None,
        }
    }
}

impl Filter {
    /// Returns true if the light ID is in the filter.
    /// # Unknown
    /// If the [`FilterType`] is `Unknown` then the result will be `true`.
    /// # Panics
    /// Will panic if the light id is greater than or equal to the group size.
    pub fn is_in_filter(&self, light_id: i32, group_size: i32) -> bool {
        assert!(light_id < group_size);
        match self.filter_type {
            FilterType::Division => {
                let start = self.parameter2 * group_size / self.parameter1.max(1);
                let end = (self.parameter2 + 1) * group_size / self.parameter1.max(1);
                light_id >= start && light_id < end
            }
            FilterType::StepAndOffset => {
                let offset_light_id = light_id - self.parameter1;
                offset_light_id % self.parameter2.max(1) == 0
            }
            FilterType::Unknown(_) => true,
        }
    }
}

loose_enum! {
    /// The parameters of a [Filter] do different things depending on the type.
    ///
    /// ### [Division](https://bsmg.wiki/mapping/map-format/lightshow.html#index-filters-type-1):
    /// Splits the group up into equal sections and selects one.
    /// - Parameter 1 determines the number of sections.
    ///   It will be rounded up to the nearest multiple of the group size.
    /// - Parameter 2 determines the section to select, starting at 0.
    ///
    /// ### [Step and Offset](https://bsmg.wiki/mapping/map-format/lightshow.html#index-filters-type-2):
    /// Alternates selecting and not selecting lights.
    /// - Parameter 1 is the index of the first light that will be selected, starting at 0.
    /// - Parameter 2 determines the number of IDs to move forward before selecting another light.
    #[derive(Default, Copy)]
    FilterType: i32 {
        #[default]
        //Todo Doesn't match wiki
        Division = 1,
        StepAndOffset = 2,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_in_filter_division_first_half() {
        let filter = Filter {
            filter_type: FilterType::Division,
            parameter1: 2,
            parameter2: 0,
            ..Default::default()
        };

        assert!((0..6).all(|i| filter.is_in_filter(i, 12)));
        assert!((6..12).all(|i| !filter.is_in_filter(i, 12)));
    }

    #[test]
    fn is_in_filter_division_second_half() {
        let filter = Filter {
            filter_type: FilterType::Division,
            parameter1: 2,
            parameter2: 1,
            ..Default::default()
        };

        assert!((0..6).all(|i| !filter.is_in_filter(i, 12)));
        assert!((6..12).all(|i| filter.is_in_filter(i, 12)));
    }

    #[test]
    fn is_in_filter_division_all() {
        let filter = Filter {
            filter_type: FilterType::Division,
            parameter1: 1,
            parameter2: 0,
            ..Default::default()
        };

        for i in 0..12 {
            assert_eq!(filter.is_in_filter(i, 12), true,);
        }
    }

    #[test]
    fn is_in_filter_step_all() {
        let filter = Filter {
            filter_type: FilterType::StepAndOffset,
            parameter1: 0,
            parameter2: 1,
            ..Default::default()
        };

        for i in 0..12 {
            assert_eq!(filter.is_in_filter(i, 12), true);
        }
    }

    #[test]
    fn is_in_filter_step_start_index() {
        for i in 0..12 {
            let filter = Filter {
                filter_type: FilterType::StepAndOffset,
                parameter1: i,
                parameter2: 1,
                ..Default::default()
            };
            assert!((i..12).all(|i| filter.is_in_filter(i, 12)));
        }
    }

    #[test]
    fn is_in_filter_step_every_other() {
        let filter = Filter {
            filter_type: FilterType::StepAndOffset,
            parameter1: 0,
            parameter2: 2,
            ..Default::default()
        };

        for i in 0..12 {
            assert_eq!(filter.is_in_filter(i, 12), i % 2 == 0);
        }
    }

    #[test]
    fn is_in_filter_step_every_other_offset() {
        let filter = Filter {
            filter_type: FilterType::StepAndOffset,
            parameter1: 1,
            parameter2: 2,
            ..Default::default()
        };

        for i in 0..12 {
            assert_eq!(filter.is_in_filter(i, 12), i % 2 != 0);
        }
    }
}
