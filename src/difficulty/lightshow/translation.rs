use crate::difficulty::lightshow::easing::Easing;
use crate::difficulty::lightshow::filter::Filter;
use crate::difficulty::lightshow::{Axis, DistributionType, TransitionType};
use crate::macros::LooseBool;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TranslationEventBox {
    #[serde(rename = "b")]
    pub beat: f32,
    #[serde(rename = "g")]
    pub group_id: i32,
    #[serde(rename = "e")]
    pub groups: Vec<TranslationEventGroup>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TranslationEventGroup {
    #[serde(rename = "f")]
    pub filter: Filter,
    #[serde(rename = "d")]
    pub beat_dist_type: DistributionType,
    #[serde(rename = "w")]
    pub beat_dist_value: f32,
    #[serde(rename = "t")]
    pub translation_dist_type: DistributionType,
    #[serde(rename = "s")]
    pub translation_dist_value: f32,
    #[serde(rename = "b")]
    pub translation_dist_effect_first: LooseBool,
    #[serde(rename = "i")]
    pub translation_dist_easing: Easing,
    #[serde(rename = "a")]
    pub axis: Axis,
    #[serde(rename = "r")]
    pub invert_axis: LooseBool,
    #[serde(rename = "l")]
    pub data: Vec<TranslationEventData>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TranslationEventData {
    #[serde(rename = "b")]
    pub beat_offset: f32,
    #[serde(rename = "p")]
    pub transition_type: TransitionType,
    #[serde(rename = "e")]
    pub easing: Easing,
    #[serde(rename = "t")]
    pub value: f32,
}
