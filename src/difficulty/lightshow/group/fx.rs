//! Events that animations unique to each environment.

use crate::difficulty::lightshow::DistributionType;
use crate::difficulty::lightshow::easing::Easing;
use crate::difficulty::lightshow::filter::Filter;
use crate::utils::LooseBool;
use crate::{TransitionType, impl_event_box, impl_event_data, impl_event_group, impl_timed};
use serde::ser::SerializeStruct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(
    feature = "bevy_reflect",
    derive(bevy_reflect::Reflect),
    reflect(Debug, Clone, PartialEq)
)]
pub struct FxEventContainer {
    pub event_boxes: Vec<FxEventBox>,
}

#[derive(Deserialize)]
struct FxEventInput {
    #[serde(rename = "vfxEventBoxGroups")]
    event_boxes: Vec<FxEventBoxRaw>,
    #[serde(rename = "_fxEventsCollection")]
    arrays: FxEventArrays,
}

#[derive(Deserialize, Serialize)]
struct FxEventArrays {
    #[serde(rename = "_fl")]
    event_data: Vec<FxEventData>,
}

impl<'de> Deserialize<'de> for FxEventContainer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let FxEventInput {
            event_boxes,
            arrays,
        } = FxEventInput::deserialize(deserializer)?;

        let event_boxes = event_boxes
            .into_iter()
            .map(|raw_box| {
                let groups = raw_box
                    .groups
                    .into_iter()
                    .map(|g| {
                        let data = g
                            .data_ids
                            .into_iter()
                            .map(|id| {
                                arrays.event_data.get(id).cloned().ok_or_else(|| {
                                    serde::de::Error::custom(format!(
                                        "Missing FxEventData with id {}",
                                        id
                                    ))
                                })
                            })
                            .collect::<Result<Vec<_>, _>>()?;

                        Ok(FxEventGroup {
                            filter: g.filter,
                            beat_dist_type: g.beat_dist_type,
                            beat_dist_value: g.beat_dist_value,
                            fx_dist_type: g.fx_dist_type,
                            fx_dist_value: g.fx_dist_value,
                            fx_dist_effect_first: g.fx_dist_effect_first,
                            fx_dist_easing: g.fx_dist_easing,
                            data,
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()?;

                Ok(FxEventBox {
                    beat: raw_box.beat,
                    group_id: raw_box.group_id,
                    groups,
                })
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(FxEventContainer { event_boxes })
    }
}

// Todo avoid allocations.
impl Serialize for FxEventContainer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut event_data: Vec<FxEventData> = Vec::new();

        // Todo Deduplicate.
        let event_boxes_raw: Vec<_> = self
            .event_boxes
            .iter()
            .map(|b| {
                let groups_raw: Vec<_> = b
                    .groups
                    .iter()
                    .map(|g| {
                        let mut ids = Vec::new();
                        for data in &g.data {
                            ids.push(event_data.len());
                            event_data.push(data.clone());
                        }

                        FxEventGroupRaw {
                            filter: g.filter.clone(),
                            beat_dist_type: g.beat_dist_type.clone(),
                            beat_dist_value: g.beat_dist_value,
                            fx_dist_type: g.fx_dist_type.clone(),
                            fx_dist_value: g.fx_dist_value,
                            fx_dist_effect_first: g.fx_dist_effect_first,
                            fx_dist_easing: g.fx_dist_easing.clone(),
                            data_ids: ids,
                        }
                    })
                    .collect();

                FxEventBoxRaw {
                    beat: b.beat,
                    group_id: b.group_id,
                    groups: groups_raw,
                }
            })
            .collect();

        // Phase 2: Serialize both arrays as a struct
        let mut state = serializer.serialize_struct("FxEventContainer", 2)?;
        state.serialize_field("vfxEventBoxGroups", &event_boxes_raw)?;
        state.serialize_field("_fxEventsCollection", &FxEventArrays { event_data })?;
        state.end()
    }
}

/// A collection of [`FxEventGroup`]s that share the same group ID and beat.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "bevy_reflect",
    derive(bevy_reflect::Reflect),
    reflect(Debug, Clone, PartialEq)
)]
pub struct FxEventBox {
    /// The time the event takes place.
    pub beat: f32,
    /// The ID of the collection of objects that this event effects.
    pub group_id: i32,
    pub groups: Vec<FxEventGroup>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FxEventBoxRaw {
    #[serde(rename = "b")]
    pub beat: f32,
    #[serde(rename = "g")]
    pub group_id: i32,
    #[serde(rename = "e")]
    pub groups: Vec<FxEventGroupRaw>,
}

impl Default for FxEventBox {
    fn default() -> Self {
        Self {
            beat: 0.0,
            group_id: 0,
            groups: vec![FxEventGroup::default()],
        }
    }
}

impl_timed!(FxEventBox::beat);
impl_event_box!(FxEventBox, FxEventGroup, FxEventData);

/// A collection of [`FxEventData`] that share the same [`Filter`] and distribution.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "bevy_reflect",
    derive(bevy_reflect::Reflect),
    reflect(Debug, Clone, PartialEq)
)]
pub struct FxEventGroup {
    pub filter: Filter,
    pub beat_dist_type: DistributionType,
    /// The strength of the beat distribution. Dependent on the [distribution type](Self::beat_dist_type).
    ///
    /// A value of zero will have no effect.
    pub beat_dist_value: f32,
    /// The strength of the brightness distribution. Dependent on the [distribution type](Self::bright_dist_type).
    ///
    /// A value of zero will have no effect.
    pub fx_dist_type: DistributionType,
    pub fx_dist_value: f32,
    /// Whether the first [`FxEventData`] of the group will be effected by brightness distribution.
    pub fx_dist_effect_first: LooseBool,
    pub fx_dist_easing: Option<Easing>,
    pub data: Vec<FxEventData>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FxEventGroupRaw {
    #[serde(rename = "f")]
    pub filter: Filter,
    #[serde(rename = "d")]
    pub beat_dist_type: DistributionType,
    #[serde(rename = "w")]
    pub beat_dist_value: f32,
    #[serde(rename = "t")]
    pub fx_dist_type: DistributionType,
    #[serde(rename = "s")]
    pub fx_dist_value: f32,
    #[serde(rename = "b")]
    pub fx_dist_effect_first: LooseBool,
    #[serde(rename = "i")]
    pub fx_dist_easing: Option<Easing>,
    #[serde(rename = "l")]
    pub data_ids: Vec<usize>,
}

impl Default for FxEventGroup {
    fn default() -> Self {
        Self {
            filter: Default::default(),
            beat_dist_type: Default::default(),
            beat_dist_value: 0.0,
            fx_dist_type: Default::default(),
            fx_dist_value: 0.0,
            fx_dist_effect_first: Default::default(),
            fx_dist_easing: Some(Easing::Linear),
            data: vec![FxEventData::default()],
        }
    }
}

impl_event_group!(FxEventGroup::get_fx_offset, FxEventData);

impl FxEventGroup {
    /// Returns the FX value that the event will be offset for a given light ID.
    /// # Panics
    /// Will panic if the light ID is greater than or equal to the group size.
    #[deprecated(note = "Experimental. Does not consider random in filter calculations.")]
    #[allow(deprecated)]
    pub fn get_fx_offset(&self, light_id: i32, group_size: i32) -> f32 {
        self.fx_dist_type.compute_value_offset(
            light_id,
            group_size,
            &self.filter,
            self.fx_dist_value,
            self.data.last().map(|data| data.beat_offset),
            self.fx_dist_easing,
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
pub struct FxEventData {
    /// The number of beats the event will be offset from the [`FxEventBox`]'s beat.
    #[serde(rename = "b")]
    pub beat_offset: f32,
    #[serde(rename = "p")]
    pub transition_type: TransitionType,
    #[serde(rename = "i")]
    pub easing: Easing,
    /// The base value of the effect.
    #[serde(rename = "v")]
    pub value: f32,
}

impl Default for FxEventData {
    fn default() -> Self {
        Self {
            beat_offset: 0.0,
            transition_type: Default::default(),
            easing: Easing::default(),
            value: 1.0,
        }
    }
}

impl_event_data!(FxEventData);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize() {
        let json = r#"
            {
                "vfxEventBoxGroups":
                [
                    {
                        "b": 2.0,
                        "g": 0,
                        "e":
                        [
                            {
                                "f":
                                {
                                    "c": 0,
                                    "f": 1,
                                    "p": 1,
                                    "t": 0,
                                    "r": 0,
                                    "n": 0,
                                    "s": 0,
                                    "l": 1.0,
                                    "d": 0
                                },
                                "w": 1.0,
                                "d": 1,
                                "s": 1.0,
                                "t": 1,
                                "b": 1,
                                "i": -1,
                                "l":
                                [
                                    0
                                ]
                            }
                        ]
                    }
                ],
                "_fxEventsCollection":
                {
                    "_fl":
                    [
                        {
                            "b": 0.0,
                            "p": 0,
                            "i": 0,
                            "v": 100.0
                        }
                    ],
                    "_il":
                    []
                }
            }
        "#;

        let container: FxEventContainer =
            serde_json::from_str(json).expect("Failed to deserialize");

        assert_eq!(
            container,
            FxEventContainer {
                event_boxes: vec![FxEventBox {
                    beat: 2.0,
                    group_id: 0,
                    groups: vec![FxEventGroup {
                        filter: Default::default(),
                        beat_dist_type: DistributionType::Wave,
                        beat_dist_value: 1.0,
                        fx_dist_type: DistributionType::Wave,
                        fx_dist_value: 1.0,
                        fx_dist_effect_first: LooseBool::True,
                        fx_dist_easing: Some(Easing::None),
                        data: vec![FxEventData {
                            beat_offset: 0.0,
                            transition_type: TransitionType::Transition,
                            easing: Easing::Linear,
                            value: 100.0,
                        }],
                    }],
                }],
            }
        );
    }

    #[test]
    fn test_roundtrip() {
        let json = r#"
            {
                "vfxEventBoxGroups":
                [
                    {
                        "b": 2.0,
                        "g": 0,
                        "e":
                        [
                            {
                                "f":
                                {
                                    "c": 0,
                                    "f": 1,
                                    "p": 1,
                                    "t": 0,
                                    "r": 0,
                                    "n": 0,
                                    "s": 0,
                                    "l": 1.0,
                                    "d": 0
                                },
                                "w": 1.0,
                                "d": 1,
                                "s": 1.0,
                                "t": 1,
                                "b": 1,
                                "i": -1,
                                "l":
                                [
                                    0
                                ]
                            }
                        ]
                    }
                ],
                "_fxEventsCollection":
                {
                    "_fl":
                    [
                        {
                            "b": 0.0,
                            "p": 0,
                            "i": 0,
                            "v": 100.0
                        }
                    ],
                    "_il":
                    []
                }
            }
        "#;

        // Deserialize into your parsed container
        let container: FxEventContainer =
            serde_json::from_str(json).expect("Failed to deserialize");

        // Serialize it back
        let out_json = serde_json::to_string_pretty(&container).expect("Failed to serialize");

        // Re-deserialize and compare
        let roundtrip: FxEventContainer =
            serde_json::from_str(&out_json).expect("Re-deserialization failed");

        assert_eq!(container, roundtrip, "Round-trip did not match");

        println!("Round-trip serialization succeeded:\n{}", out_json);
    }
}
