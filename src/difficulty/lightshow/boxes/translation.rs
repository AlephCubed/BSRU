use crate::difficulty::lightshow::boxes::EventData;
use crate::difficulty::lightshow::easing::Easing;
use crate::difficulty::lightshow::filter::Filter;
use crate::difficulty::lightshow::{DistributionType, EventAxis, TransitionType};
use crate::utils::LooseBool;
use crate::{impl_event_box, impl_event_group, impl_timed};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(
    feature = "bevy_reflect",
    derive(bevy_reflect::Reflect),
    reflect(Debug, Clone, PartialEq)
)]
pub struct TranslationEventBox {
    #[serde(rename = "b")]
    pub beat: f32,
    #[serde(rename = "g")]
    pub group_id: i32,
    #[serde(rename = "e")]
    pub groups: Vec<TranslationEventGroup>,
}

impl_timed!(TranslationEventBox::beat);
impl_event_box!(
    TranslationEventBox,
    TranslationEventGroup,
    TranslationEventData
);

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(
    feature = "bevy_reflect",
    derive(bevy_reflect::Reflect),
    reflect(Debug, Clone, PartialEq)
)]
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
    pub axis: EventAxis,
    #[serde(rename = "r")]
    pub invert_axis: LooseBool,
    #[serde(rename = "l")]
    pub data: Vec<TranslationEventData>,
}

impl_event_group!(
    TranslationEventGroup::get_translation_offset,
    TranslationEventData
);

impl TranslationEventGroup {
    pub fn get_translation_offset(&self, light_id: i32, group_size: i32) -> f32 {
        self.translation_dist_type.compute_offset(
            light_id,
            group_size,
            &self.filter,
            self.translation_dist_value,
            self.data.last().map(|data| data.beat_offset),
            Some(self.translation_dist_easing),
        )
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(
    feature = "bevy_reflect",
    derive(bevy_reflect::Reflect),
    reflect(Debug, Clone, PartialEq)
)]
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

impl EventData for TranslationEventData {
    fn get_beat_offset(&self) -> f32 {
        self.beat_offset
    }
}
