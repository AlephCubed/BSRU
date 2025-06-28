use crate::difficulty::lightshow::DistributionType;
use crate::difficulty::lightshow::easing::Easing;
use crate::difficulty::lightshow::filter::Filter;
use crate::loose_enum;
use crate::macros::LooseBool;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColorEventBox {
    #[serde(rename = "b")]
    pub beat: f32,
    #[serde(rename = "g")]
    pub group_id: i32,
    #[serde(rename = "e")]
    pub groups: Vec<ColorEventGroup>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
