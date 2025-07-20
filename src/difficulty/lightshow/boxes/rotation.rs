use crate::difficulty::lightshow::boxes::EventData;
use crate::difficulty::lightshow::easing::Easing;
use crate::difficulty::lightshow::filter::Filter;
use crate::difficulty::lightshow::{DistributionType, EventAxis, TransitionType};
use crate::utils::LooseBool;
use crate::{impl_event_box, impl_event_group, impl_timed, loose_enum};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(
    feature = "bevy_reflect",
    derive(bevy_reflect::Reflect),
    reflect(Debug, Clone, PartialEq)
)]
pub struct RotationEventBox {
    #[serde(rename = "b")]
    pub beat: f32,
    #[serde(rename = "g")]
    pub group_id: i32,
    #[serde(rename = "e")]
    pub groups: Vec<RotationEventGroup>,
}

impl Default for RotationEventBox {
    fn default() -> Self {
        Self {
            beat: 0.0,
            group_id: 0,
            groups: vec![RotationEventGroup::default()],
        }
    }
}

impl_timed!(RotationEventBox::beat);
impl_event_box!(RotationEventBox, RotationEventGroup, RotationEventData);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(
    feature = "bevy_reflect",
    derive(bevy_reflect::Reflect),
    reflect(Debug, Clone, PartialEq)
)]
pub struct RotationEventGroup {
    #[serde(rename = "f")]
    pub filter: Filter,
    #[serde(rename = "d")]
    pub beat_dist_type: DistributionType,
    #[serde(rename = "w")]
    pub beat_dist_value: f32,
    #[serde(rename = "t")]
    pub rotation_dist_type: DistributionType,
    #[serde(rename = "s")]
    pub rotation_dist_value: f32,
    #[serde(rename = "b")]
    pub rotation_dist_effect_first: LooseBool,
    /// > Only present in difficulty file V3.2 or higher.
    #[serde(rename = "i")]
    pub rotation_dist_easing: Option<Easing>,
    #[serde(rename = "a")]
    pub axis: EventAxis,
    #[serde(rename = "r")]
    pub invert_axis: LooseBool,
    #[serde(rename = "l")]
    pub data: Vec<RotationEventData>,
}

impl Default for RotationEventGroup {
    fn default() -> Self {
        Self {
            filter: Default::default(),
            beat_dist_type: Default::default(),
            beat_dist_value: 0.0,
            rotation_dist_type: Default::default(),
            rotation_dist_value: 0.0,
            rotation_dist_effect_first: LooseBool::True,
            rotation_dist_easing: None,
            axis: Default::default(),
            invert_axis: Default::default(),
            data: vec![RotationEventData::default()],
        }
    }
}

impl_event_group!(RotationEventGroup::get_rotation_offset, RotationEventData);

impl RotationEventGroup {
    /// Returns the number of degrees that the event will be offset for a given light ID.
    /// # Panics
    /// Will panic if the light ID is greater than or equal to the group size.
    #[deprecated(
        note = "Experimental. Does not consider chunks, random, or limit in filter calculations."
    )]
    #[allow(deprecated)]
    pub fn get_rotation_offset(&self, light_id: i32, group_size: i32) -> f32 {
        self.rotation_dist_type.compute_offset(
            light_id,
            group_size,
            &self.filter,
            self.rotation_dist_value,
            self.data.last().map(|data| data.beat_offset),
            self.rotation_dist_easing,
        )
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(
    feature = "bevy_reflect",
    derive(bevy_reflect::Reflect),
    reflect(Debug, Clone, PartialEq)
)]
pub struct RotationEventData {
    #[serde(rename = "b")]
    pub beat_offset: f32,
    #[serde(rename = "p")]
    pub transition_type: TransitionType,
    #[serde(rename = "e")]
    pub easing: Easing,
    #[serde(rename = "r")]
    pub degrees: f32,
    #[serde(rename = "o")]
    pub direction: RotationDirection,
    #[serde(rename = "l")]
    pub loops: i32,
}

impl EventData for RotationEventData {
    fn get_beat_offset(&self) -> f32 {
        self.beat_offset
    }
}

loose_enum! {
    /// Determines the direction that the rotation event will rotate.
    /// Automatic will choose the shortest distance.
    #[derive(Default, Copy)]
    RotationDirection: i32 {
        #[default]
        Automatic = 0,
        Clockwise = 1,
        CounterClockwise = 2,
    }
}
