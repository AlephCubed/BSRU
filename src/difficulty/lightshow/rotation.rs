use crate::difficulty::lightshow::DistributionType;
use crate::difficulty::lightshow::easing::Easing;
use crate::difficulty::lightshow::filter::Filter;
use crate::loose_enum;
use crate::macros::LooseBool;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RotationEventBoxGroup {
    #[serde(rename = "b")]
    pub beat: f32,
    #[serde(rename = "g")]
    pub group_id: i32,
    #[serde(rename = "e")]
    pub groups: Vec<RotationEventGroup>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
    pub axis: i32,
    #[serde(rename = "r")]
    pub invert_axis: LooseBool,
    #[serde(rename = "l")]
    pub data: Vec<RotationEventData>,
}

loose_enum! {
    #[derive(Default, Copy)]
    Axis: i32 {
        #[default]
        X = 0,
        Y = 1,
        Z = 2,
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RotationEventData {
    #[serde(rename = "b")]
    pub beat: f32,
    #[serde(rename = "p")]
    pub transition_type: RotationTransitionType,
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
    /// Controls how the angle is changed from the previous event.
    /// - Transition: The angle will slowly move from the previous events angle, using the easing.
    /// - Extend: The events rotation will be ignored,
    /// and the values from the previous event will be used instead.
    ///
    /// More info [here](https://bsmg.wiki/mapping/map-format/lightshow.html#light-rotation-events-type).
    #[derive(Default, Copy)]
    RotationTransitionType: i32 {
        #[default]
        Transition = 0,
        Extend = 1,
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
