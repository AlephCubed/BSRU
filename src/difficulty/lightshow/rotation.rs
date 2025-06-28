use crate::difficulty::lightshow::easing::Easing;
use crate::difficulty::lightshow::filter::Filter;
use crate::difficulty::lightshow::{Axis, DistributionType, TransitionType};
use crate::utils::LooseBool;
use crate::{impl_get_beat_offset, impl_timed, loose_enum};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
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

impl_timed!(RotationEventBox::beat);

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
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
    /// Only present in difficulty file V3.2 or higher.
    #[serde(rename = "i")]
    pub rotation_dist_easing: Option<Easing>,
    #[serde(rename = "a")]
    pub axis: Axis,
    #[serde(rename = "r")]
    pub invert_axis: LooseBool,
    #[serde(rename = "l")]
    pub data: Vec<RotationEventData>,
}

impl_get_beat_offset!(RotationEventGroup);

impl RotationEventGroup {
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
