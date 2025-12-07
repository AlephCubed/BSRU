//! Events that control the translation/position of objects.

use crate::difficulty::lightshow::easing::Easing;
use crate::difficulty::lightshow::filter::Filter;
use crate::difficulty::lightshow::group::EventData;
use crate::difficulty::lightshow::{DistributionType, EventAxis, TransitionType};
use crate::{impl_event_box, impl_event_group, impl_timed};
use loose_enum::LooseBool;
use serde::{Deserialize, Serialize};

/// A collection of [`TranslationEventGroup`]s that share the same group ID and beat.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(
    feature = "bevy_reflect",
    derive(bevy_reflect::Reflect),
    reflect(Debug, Clone, PartialEq)
)]
pub struct TranslationEventBox {
    /// The time the event takes place.
    #[serde(rename = "b")]
    pub beat: f32,
    /// The ID of the collection of objects that this event effects.
    #[serde(rename = "g")]
    pub group_id: i32,
    #[serde(rename = "e")]
    pub groups: Vec<TranslationEventGroup>,
}

impl Default for TranslationEventBox {
    fn default() -> Self {
        Self {
            beat: 0.0,
            group_id: 0,
            groups: vec![TranslationEventGroup::default()],
        }
    }
}

impl_timed!(TranslationEventBox::beat);
impl_event_box!(
    TranslationEventBox,
    TranslationEventGroup,
    TranslationEventData
);

/// A collection of [`TranslationEventData`] that share the same [`EventAxis`], [`Filter`], and distribution.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
    /// The strength of the beat distribution. Dependent on the [beat distribution type](Self::beat_dist_type).
    ///
    /// A value of zero will have no effect.
    #[serde(rename = "w")]
    pub beat_dist_value: f32,
    #[serde(rename = "t")]
    pub translation_dist_type: DistributionType,
    /// The strength of the translation distribution. Dependent on the [distribution type](Self::translation_dist_type).
    ///
    /// A value of zero will have no effect.
    #[serde(rename = "s")]
    pub translation_dist_value: f32,
    /// Whether the first [`TranslationEventData`] of the group will be effected by translation distribution.
    #[serde(rename = "b")]
    pub translation_dist_effect_first: LooseBool<i32>,
    #[serde(rename = "i")]
    pub translation_dist_easing: Easing,
    #[serde(rename = "a")]
    pub axis: EventAxis,
    /// If true, the translation will be mirrored.
    #[serde(rename = "r")]
    pub invert_axis: LooseBool<i32>,
    #[serde(rename = "l")]
    pub data: Vec<TranslationEventData>,
}

impl Default for TranslationEventGroup {
    fn default() -> Self {
        Self {
            filter: Default::default(),
            beat_dist_type: Default::default(),
            beat_dist_value: 0.0,
            translation_dist_type: Default::default(),
            translation_dist_value: 0.0,
            translation_dist_effect_first: Default::default(),
            translation_dist_easing: Easing::Linear,
            axis: Default::default(),
            invert_axis: Default::default(),
            data: vec![TranslationEventData::default()],
        }
    }
}

impl_event_group!(
    TranslationEventGroup::get_translation_offset,
    TranslationEventData
);

impl TranslationEventGroup {
    /// Returns the number of units that the event will be offset for a given light ID.
    /// # Panics
    /// Will panic if the light ID is greater than or equal to the group size.
    #[deprecated(note = "Experimental. Does not consider random in filter calculations.")]
    #[allow(deprecated)]
    pub fn get_translation_offset(&self, light_id: i32, group_size: i32) -> f32 {
        self.translation_dist_type.compute_value_offset(
            light_id,
            group_size,
            &self.filter,
            self.translation_dist_value,
            self.data.last().map(|data| data.beat_offset),
            Some(self.translation_dist_easing),
        )
    }
}

/// The lowest-level group event type, which determines the base position of the event.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(
    feature = "bevy_reflect",
    derive(bevy_reflect::Reflect),
    reflect(Debug, Clone, PartialEq)
)]
pub struct TranslationEventData {
    /// The number of beats the event will be offset from the [`TranslationEventBox`]'s beat.
    #[serde(rename = "b")]
    pub beat_offset: f32,
    #[serde(rename = "p")]
    pub transition_type: TransitionType,
    #[serde(rename = "e")]
    pub easing: Easing,
    /// The base number of units the event will offset objects by.
    #[serde(rename = "t")]
    pub value: f32,
}

impl EventData for TranslationEventData {
    fn get_beat_offset(&self) -> f32 {
        self.beat_offset
    }
}
