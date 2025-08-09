//! Events that control animations unique to each environment.
//!
//! Unlike the other V3 group event types, FX events use a template-like JSON syntax.
//! In order to have standardized structure across all V3 events, custom serialization has been written in [`FxEventContainer`].
//! Because of this, neither [`FxEventBox`] nor [`FxEventGroup`] implement [`Serialize`] nor [`Deserialize`] directly.

use crate::difficulty::lightshow::DistributionType;
use crate::difficulty::lightshow::easing::Easing;
use crate::difficulty::lightshow::filter::Filter;
use crate::utils::LooseBool;
use crate::{TransitionType, impl_event_box, impl_event_data, impl_event_group, impl_timed};
use indexmap::IndexSet;
use ordered_float::OrderedFloat;
use serde::ser::SerializeStruct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::ops::{Deref, DerefMut};

/// Contains a list of [`FxEventBox`] as well as the [`Serialize`] and [`Deserialize`] implementations for FX events.
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(
    feature = "bevy_reflect",
    derive(bevy_reflect::Reflect),
    reflect(Debug, Clone, PartialEq)
)]
pub struct FxEventContainer {
    pub event_boxes: Vec<FxEventBox>,
}

impl Deref for FxEventContainer {
    type Target = Vec<FxEventBox>;

    fn deref(&self) -> &Self::Target {
        &self.event_boxes
    }
}

impl DerefMut for FxEventContainer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.event_boxes
    }
}

/// The format that is actually stored in JSON.
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
                    .map(|raw_group| {
                        let data = raw_group
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
                            .collect::<Result<Vec<FxEventData>, _>>()?;

                        Ok(FxEventGroup {
                            filter: raw_group.filter,
                            beat_dist_type: raw_group.beat_dist_type,
                            beat_dist_value: raw_group.beat_dist_value,
                            fx_dist_type: raw_group.fx_dist_type,
                            fx_dist_value: raw_group.fx_dist_value,
                            fx_dist_effect_first: raw_group.fx_dist_effect_first,
                            fx_dist_easing: raw_group.fx_dist_easing,
                            data,
                        })
                    })
                    .collect::<Result<Vec<FxEventGroup>, _>>()?;

                Ok(FxEventBox {
                    beat: raw_box.beat,
                    group_id: raw_box.group_id,
                    groups,
                })
            })
            .collect::<Result<Vec<FxEventBox>, _>>()?;

        Ok(FxEventContainer { event_boxes })
    }
}

// Todo avoid allocations.
impl Serialize for FxEventContainer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut event_data: IndexSet<FxEventDataKey> = IndexSet::new();

        // Todo Deduplicate.
        let raw_boxes = self
            .event_boxes
            .iter()
            .map(|event_box| {
                let groups_raw = event_box
                    .groups
                    .iter()
                    .map(|event_group| {
                        let mut ids = Vec::new();

                        for data in &event_group.data {
                            let data_key: FxEventDataKey = data.into();

                            let index = event_data.get_index_of(&data_key).unwrap_or_else(|| {
                                let index = event_data.len();
                                event_data.insert(data_key);
                                index
                            });

                            ids.push(index);
                        }

                        FxEventGroupRaw {
                            filter: event_group.filter.clone(),
                            beat_dist_type: event_group.beat_dist_type.clone(),
                            beat_dist_value: event_group.beat_dist_value,
                            fx_dist_type: event_group.fx_dist_type.clone(),
                            fx_dist_value: event_group.fx_dist_value,
                            fx_dist_effect_first: event_group.fx_dist_effect_first,
                            fx_dist_easing: event_group.fx_dist_easing.clone(),
                            data_ids: ids,
                        }
                    })
                    .collect::<Vec<FxEventGroupRaw>>();

                FxEventBoxRaw {
                    beat: event_box.beat,
                    group_id: event_box.group_id,
                    groups: groups_raw,
                }
            })
            .collect::<Vec<FxEventBoxRaw>>();

        let mut state = serializer.serialize_struct("FxEventContainer", 2)?;
        state.serialize_field("vfxEventBoxGroups", &raw_boxes)?;
        state.serialize_field(
            "_fxEventsCollection",
            &FxEventArrays {
                event_data: event_data.into_iter().map(FxEventData::from).collect(),
            },
        )?;
        state.end()
    }
}

/// A collection of [`FxEventGroup`]s that share the same group ID and beat.
///
/// Does not implement [`Serialize`] nor [`Deserialize`]. For more info, see the [module docs](super::fx).
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

/// The raw JSON structure that uses [data IDs](FxEventGroupRaw::data_ids) rather than actual [event data](FxEventData).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct FxEventBoxRaw {
    #[serde(rename = "b")]
    beat: f32,
    #[serde(rename = "g")]
    group_id: i32,
    #[serde(rename = "e")]
    groups: Vec<FxEventGroupRaw>,
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
///
/// Does not implement [`Serialize`] nor [`Deserialize`]. For more info, see the [module docs](super::fx).
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
    pub fx_dist_type: DistributionType,
    /// The strength of the brightness distribution. Dependent on the [distribution type](Self::fx_dist_type).
    ///
    /// A value of zero will have no effect.
    pub fx_dist_value: f32,
    /// Whether the first [`FxEventData`] of the group will be effected by brightness distribution.
    pub fx_dist_effect_first: LooseBool,
    pub fx_dist_easing: Option<Easing>,
    /// In the actual JSON structure, this is a list of indexes to a separate list of event data.
    /// For consistency, this is merged during parsing.
    pub data: Vec<FxEventData>,
}

/// The raw JSON structure that uses [data IDs](self::data_ids) rather than actual [event data](FxEventData).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct FxEventGroupRaw {
    #[serde(rename = "f")]
    filter: Filter,
    #[serde(rename = "d")]
    beat_dist_type: DistributionType,
    #[serde(rename = "w")]
    beat_dist_value: f32,
    #[serde(rename = "t")]
    fx_dist_type: DistributionType,
    #[serde(rename = "s")]
    fx_dist_value: f32,
    #[serde(rename = "b")]
    fx_dist_effect_first: LooseBool,
    #[serde(rename = "i")]
    fx_dist_easing: Option<Easing>,
    #[serde(rename = "l")]
    data_ids: Vec<usize>,
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

/// The lowest-level group event type, which determines the base value of the event.
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

/// A `PartialEq` and `Hash` version of [`FxEventData`], allowing for deduplication.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct FxEventDataKey {
    beat_offset: OrderedFloat<f32>,
    transition_type: TransitionType,
    easing: Easing,
    value: OrderedFloat<f32>,
}

impl From<&FxEventData> for FxEventDataKey {
    fn from(value: &FxEventData) -> Self {
        Self {
            beat_offset: value.beat_offset.into(),
            transition_type: value.transition_type,
            easing: value.easing,
            value: value.value.into(),
        }
    }
}

impl From<FxEventDataKey> for FxEventData {
    fn from(value: FxEventDataKey) -> Self {
        Self {
            beat_offset: value.beat_offset.into(),
            transition_type: value.transition_type,
            easing: value.easing,
            value: value.value.into(),
        }
    }
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
    use serde_json::{Value, json};

    fn get_test_container() -> FxEventContainer {
        FxEventContainer {
            event_boxes: vec![FxEventBox {
                beat: 2.0,
                group_id: 0,
                groups: vec![get_test_group(), get_test_group()],
            }],
        }
    }

    fn get_test_group() -> FxEventGroup {
        FxEventGroup {
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
        }
    }

    fn get_test_json() -> Value {
        json!(
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
                            },
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
                    ]
                }
            }
        )
    }

    #[test]
    fn test_deserialize() {
        let container: FxEventContainer = serde_json::from_value(get_test_json()).unwrap();

        assert_eq!(container, get_test_container());
    }

    #[test]
    fn test_serialize() {
        let out_json = serde_json::to_value(&get_test_container()).unwrap();

        assert_eq!(out_json, get_test_json());
    }

    #[test]
    fn test_round_trip() {
        let container: FxEventContainer = serde_json::from_value(get_test_json()).unwrap();

        let out_json = serde_json::to_string_pretty(&container).unwrap();

        let round_trip: FxEventContainer = serde_json::from_str(&out_json).unwrap();

        assert_eq!(container, round_trip);
    }
}
