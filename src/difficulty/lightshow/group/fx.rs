//! Events that animations unique to each environment.

use crate::difficulty::lightshow::DistributionType;
use crate::difficulty::lightshow::easing::Easing;
use crate::difficulty::lightshow::filter::Filter;
use crate::utils::LooseBool;
use crate::{TransitionType, impl_event_box, impl_event_data, impl_event_group, impl_timed};
use serde::{Deserialize, Serialize};

/// A collection of [`FxEventGroup`]s that share the same group ID and beat.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(
    feature = "bevy_reflect",
    derive(bevy_reflect::Reflect),
    reflect(Debug, Clone, PartialEq)
)]
pub struct FxEventBox {
    /// The time the event takes place.
    #[serde(rename = "b")]
    pub beat: f32,
    /// The ID of the collection of objects that this event effects.
    #[serde(rename = "g")]
    pub group_id: i32,
    #[serde(rename = "e")]
    pub groups: Vec<FxEventGroup>,
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(
    feature = "bevy_reflect",
    derive(bevy_reflect::Reflect),
    reflect(Debug, Clone, PartialEq)
)]
pub struct FxEventGroup {
    #[serde(rename = "f")]
    pub filter: Filter,
    #[serde(rename = "d")]
    pub beat_dist_type: DistributionType,
    /// The strength of the beat distribution. Dependent on the [distribution type](Self::beat_dist_type).
    ///
    /// A value of zero will have no effect.
    #[serde(rename = "w")]
    pub beat_dist_value: f32,
    /// The strength of the brightness distribution. Dependent on the [distribution type](Self::bright_dist_type).
    ///
    /// A value of zero will have no effect.
    #[serde(rename = "t")]
    pub fx_dist_type: DistributionType,
    #[serde(rename = "r")]
    pub fx_dist_value: f32,
    /// Whether the first [`FxEventData`] of the group will be effected by brightness distribution.
    #[serde(rename = "b")]
    pub fx_dist_effect_first: LooseBool,
    #[serde(rename = "i")]
    pub fx_dist_easing: Option<Easing>,
    #[serde(rename = "e")]
    pub data: Vec<FxEventData>,
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
