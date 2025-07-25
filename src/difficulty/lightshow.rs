//! Events that have no effect on gameplay.

pub mod basic;
pub mod easing;
pub mod filter;
pub mod group;

pub use basic::*;
pub use easing::*;
pub use filter::*;
pub use group::*;

use crate::loose_enum;

loose_enum! {
    /// The way that the distribution value is used.
    #[derive(Default, Copy)]
    DistributionType: i32 {
        /// The distribution value represents the difference between *the last and first step*.
        #[default]
        Wave = 1,
        /// The distribution value represents the difference between *each step*.
        Step = 2,
    }
}

impl DistributionType {
    #[deprecated(note = "Experimental. Does not consider random or limit in filter calculations.")]
    #[allow(deprecated)]
    fn compute_beat_offset(
        &self,
        light_id: i32,
        group_size: i32,
        filter: &Filter,
        dist_value: f32,
        last_data_offset: Option<f32>,
        easing: Option<Easing>,
    ) -> f32 {
        let filtered_id = filter.get_relative_index(light_id, group_size);

        let filtered_size = if let Some(limit_behaviour) = filter.limit_behaviour
            && limit_behaviour.duration()
        {
            filter.count_filtered(group_size)
        } else {
            filter.count_filtered_without_limit(group_size)
        };

        self.compute_offset(
            filtered_id,
            filtered_size,
            dist_value,
            last_data_offset,
            easing,
        )
    }

    #[deprecated(note = "Experimental. Does not consider random or limit in filter calculations.")]
    #[allow(deprecated)]
    fn compute_value_offset(
        &self,
        light_id: i32,
        group_size: i32,
        filter: &Filter,
        dist_value: f32,
        last_data_offset: Option<f32>,
        easing: Option<Easing>,
    ) -> f32 {
        let filtered_id = filter.get_relative_index(light_id, group_size);

        let filtered_size = if let Some(limit_behaviour) = filter.limit_behaviour
            && limit_behaviour.distribution()
        {
            filter.count_filtered(group_size)
        } else {
            filter.count_filtered_without_limit(group_size)
        };

        self.compute_offset(
            filtered_id,
            filtered_size,
            dist_value,
            last_data_offset,
            easing,
        )
    }

    #[deprecated(note = "Experimental. Does not consider random or limit in filter calculations.")]
    #[allow(deprecated)]
    #[inline(always)]
    fn compute_offset(
        &self,
        filtered_id: i32,
        filtered_size: i32,
        dist_value: f32,
        last_data_offset: Option<f32>,
        easing: Option<Easing>,
    ) -> f32 {
        let filtered_id = filtered_id as f32;
        let filtered_size = filtered_size as f32;

        if dist_value == 0.0 {
            return 0.0;
        }

        match self {
            DistributionType::Wave => {
                let mut modified_value = dist_value;
                if let Some(offset) = last_data_offset {
                    modified_value -= offset;
                }

                let mut fraction = filtered_id / filtered_size;
                if let Some(easing) = easing {
                    fraction = easing.ease(fraction);
                }

                fraction * modified_value
            }
            DistributionType::Step => dist_value * filtered_id,
            DistributionType::Unknown(_) => 0.0,
        }
    }

    /// Tests that both [`self.compute_beat_offset`] and [`self.compute_value_offset`] have the same return value, returning that value.
    ///
    /// This only makes sense if the [`LimitBehaviour`] is either `None` or `Both`.
    #[cfg(test)]
    #[allow(deprecated)]
    fn compute_both(
        &self,
        light_id: i32,
        group_size: i32,
        filter: &Filter,
        dist_value: f32,
        last_data_offset: Option<f32>,
        easing: Option<Easing>,
    ) -> f32 {
        assert!(
            matches!(
                filter.limit_behaviour,
                None | Some(LimitBehaviour::None) | Some(LimitBehaviour::Both)
            ),
            "This test method only makes sense if LimitBehaviour is `None` or `Both`"
        );

        let beat = self.compute_beat_offset(
            light_id,
            group_size,
            filter,
            dist_value,
            last_data_offset,
            easing,
        );
        let value = self.compute_value_offset(
            light_id,
            group_size,
            filter,
            dist_value,
            last_data_offset,
            easing,
        );
        assert_eq!(beat, value);
        beat
    }
}

loose_enum! {
    /// Controls how the state is changed relative to the previous event.
    #[derive(Default, Copy)]
    TransitionType: i32 {
        /// The state will blend from the previous event's state, using the events [easing](Easing).
        #[default]
        Transition = 0,
        /// The event's state will be ignored, replaced with the state from the previous event.
        Extend = 1,
    }
}

loose_enum! {
    /// The axis that a rotation/translation event effects.
    #[derive(Default, Copy)]
    EventAxis: i32 {
        #[default]
        X = 0,
        Y = 1,
        Z = 2,
    }
}

// More readable concrete tests are available in the `color` module.
#[allow(deprecated)]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::difficulty::lightshow::filter::FilterType;
    use crate::utils::LooseBool;

    #[test]
    fn wave() {
        for i in 0..12 {
            assert_eq!(
                DistributionType::Wave.compute_both(i, 12, &Filter::default(), 12.0, None, None),
                i as f32
            );
        }
    }

    #[test]
    fn step() {
        for i in 0..12 {
            assert_eq!(
                DistributionType::Step.compute_both(i, 12, &Filter::default(), 1.0, None, None),
                i as f32
            );
        }
    }

    #[test]
    fn wave_negative() {
        for i in 0..12 {
            assert_eq!(
                DistributionType::Wave.compute_both(i, 12, &Filter::default(), -12.0, None, None),
                -i as f32
            );
        }
    }

    #[test]
    fn step_negative() {
        for i in 0..12 {
            assert_eq!(
                DistributionType::Step.compute_both(i, 12, &Filter::default(), -1.0, None, None),
                -i as f32
            );
        }
    }

    #[test]
    fn wave_zero() {
        for i in 0..12 {
            assert_eq!(
                DistributionType::Wave.compute_both(i, 12, &Filter::default(), 0.0, None, None),
                0.0
            );
        }
    }

    #[test]
    fn step_zero() {
        for i in 0..12 {
            assert_eq!(
                DistributionType::Step.compute_both(i, 12, &Filter::default(), 0.0, None, None),
                0.0
            );
        }
    }

    #[test]
    fn wave_with_division_filter() {
        for i in 0..6 {
            assert_eq!(
                DistributionType::Wave.compute_both(
                    i + 6,
                    12,
                    &Filter {
                        filter_type: FilterType::Division,
                        parameter1: 2,
                        parameter2: 1,
                        ..Default::default()
                    },
                    6.0,
                    None,
                    None
                ),
                i as f32
            );
        }
    }

    #[test]
    fn step_with_division_filter() {
        for i in 0..6 {
            assert_eq!(
                DistributionType::Step.compute_both(
                    i + 6,
                    12,
                    &Filter {
                        filter_type: FilterType::Division,
                        parameter1: 2,
                        parameter2: 1,
                        ..Default::default()
                    },
                    1.0,
                    None,
                    None
                ),
                i as f32
            );
        }
    }

    #[test]
    fn wave_with_step_filter() {
        for i in 0..6 {
            assert_eq!(
                DistributionType::Wave.compute_both(
                    i * 2,
                    12,
                    &Filter {
                        filter_type: FilterType::StepAndOffset,
                        parameter1: 0,
                        parameter2: 2,
                        ..Default::default()
                    },
                    6.0,
                    None,
                    None
                ),
                i as f32
            );
        }
    }

    #[test]
    fn step_with_step_filter() {
        for i in 0..6 {
            assert_eq!(
                DistributionType::Step.compute_both(
                    i * 2,
                    12,
                    &Filter {
                        filter_type: FilterType::StepAndOffset,
                        parameter1: 0,
                        parameter2: 2,
                        ..Default::default()
                    },
                    1.0,
                    None,
                    None
                ),
                i as f32
            );
        }
    }

    #[test]
    fn wave_with_reverse_filter() {
        for i in 0..12 {
            assert_eq!(
                DistributionType::Wave.compute_both(
                    i,
                    12,
                    &Filter {
                        reverse: LooseBool::True,
                        ..Default::default()
                    },
                    12.0,
                    None,
                    None
                ),
                12.0 - i as f32
            );
        }
    }

    #[test]
    fn step_with_reverse_filter() {
        for i in 0..12 {
            assert_eq!(
                DistributionType::Step.compute_both(
                    i,
                    12,
                    &Filter {
                        reverse: LooseBool::True,
                        ..Default::default()
                    },
                    1.0,
                    None,
                    None
                ),
                12.0 - i as f32
            );
        }
    }

    #[test]
    fn wave_with_chunks_of_size_two() {
        for i in 0..6 {
            let filter = Filter {
                chunks: Some(6),
                ..Default::default()
            };
            assert_eq!(
                DistributionType::Wave.compute_both(i * 2, 12, &filter, 6.0, None, None),
                i as f32
            );
            assert_eq!(
                DistributionType::Wave.compute_both(i * 2 + 1, 12, &filter, 6.0, None, None),
                i as f32
            );
        }
    }

    #[test]
    fn step_with_chunks_of_size_two() {
        for i in 0..6 {
            let filter = Filter {
                chunks: Some(6),
                ..Default::default()
            };
            assert_eq!(
                DistributionType::Step.compute_both(i * 2, 12, &filter, 1.0, None, None),
                i as f32
            );
            assert_eq!(
                DistributionType::Step.compute_both(i * 2 + 1, 12, &filter, 1.0, None, None),
                i as f32
            );
        }
    }

    #[test]
    fn wave_with_chunks_of_size_six() {
        for i in 0..6 {
            let filter = Filter {
                chunks: Some(2),
                ..Default::default()
            };
            assert_eq!(
                DistributionType::Wave.compute_both(i, 12, &filter, 2.0, None, None),
                0.0
            );
            assert_eq!(
                DistributionType::Wave.compute_both(i + 6, 12, &filter, 2.0, None, None),
                1.0
            );
        }
    }

    #[test]
    fn step_with_chunks_of_size_six() {
        for i in 0..6 {
            let filter = Filter {
                chunks: Some(2),
                ..Default::default()
            };
            assert_eq!(
                DistributionType::Step.compute_both(i, 12, &filter, 1.0, None, None),
                0.0
            );
            assert_eq!(
                DistributionType::Step.compute_both(i + 6, 12, &filter, 1.0, None, None),
                1.0
            );
        }
    }

    #[test]
    fn wave_with_chunks_out_of_bounds() {
        let filter = Filter {
            chunks: Some(24),
            ..Default::default()
        };

        for i in 0..12 {
            assert_eq!(
                DistributionType::Wave.compute_both(i, 12, &filter, 12.0, None, None),
                i as f32
            );
        }
    }

    #[test]
    fn step_with_chunks_out_of_bounds() {
        let filter = Filter {
            chunks: Some(24),
            ..Default::default()
        };

        for i in 0..12 {
            assert_eq!(
                DistributionType::Step.compute_both(i, 12, &filter, 1.0, None, None),
                i as f32
            );
        }
    }

    #[test]
    fn wave_with_limit() {
        let filter = Filter {
            limit_percent: Some(0.5),
            ..Default::default()
        };

        for i in 0..6 {
            assert_eq!(
                DistributionType::Wave.compute_both(i, 12, &filter, 12.0, None, None),
                i as f32
            );
        }
    }

    #[test]
    fn step_with_limit() {
        let filter = Filter {
            limit_percent: Some(0.5),
            ..Default::default()
        };

        for i in 0..6 {
            assert_eq!(
                DistributionType::Step.compute_both(i, 12, &filter, 1.0, None, None),
                i as f32
            );
        }
    }

    #[test]
    fn wave_with_enabled_limit() {
        let filter = Filter {
            limit_behaviour: Some(LimitBehaviour::Both),
            limit_percent: Some(0.5),
            ..Default::default()
        };

        for i in 0..6 {
            assert_eq!(
                DistributionType::Wave.compute_both(i, 12, &filter, 12.0, None, None),
                (i * 2) as f32
            );
        }
    }

    #[test]
    fn step_with_enabled_limit() {
        let filter = Filter {
            limit_behaviour: Some(LimitBehaviour::Both),
            limit_percent: Some(0.5),
            ..Default::default()
        };

        for i in 0..6 {
            assert_eq!(
                DistributionType::Step.compute_both(i, 12, &filter, 1.0, None, None),
                i as f32
            );
        }
    }
}
