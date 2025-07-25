//! Events that control the rotation of objects.

use crate::difficulty::lightshow::easing::Easing;
use crate::difficulty::lightshow::filter::Filter;
use crate::difficulty::lightshow::group::EventData;
use crate::difficulty::lightshow::{DistributionType, EventAxis, TransitionType};
use crate::utils::LooseBool;
use crate::{impl_event_box, impl_event_group, impl_timed, loose_enum};
use serde::{Deserialize, Serialize};

/// A collection of [`RotationEventGroup`]s that share the same group ID and beat.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(
    feature = "bevy_reflect",
    derive(bevy_reflect::Reflect),
    reflect(Debug, Clone, PartialEq)
)]
pub struct RotationEventBox {
    /// The time the event takes place.
    #[serde(rename = "b")]
    pub beat: f32,
    /// The ID of the collection of objects that this event effects.
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

/// A collection of [`RotationEventData`] that share the same [`EventAxis`], [`Filter`], and distribution.
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
    /// The strength of the beat distribution. Dependent on the [distribution type](Self::beat_dist_type).
    ///
    /// A value of zero will have no effect.
    #[serde(rename = "w")]
    pub beat_dist_value: f32,
    #[serde(rename = "t")]
    pub rotation_dist_type: DistributionType,
    /// The strength of the rotation distribution. Dependent on the [distribution type](Self::rotation_dist_type).
    ///
    /// A value of zero will have no effect.
    #[serde(rename = "s")]
    pub rotation_dist_value: f32,
    /// Whether the first [`RotationEventData`] of the group will be effected by rotation distribution.
    #[serde(rename = "b")]
    pub rotation_dist_effect_first: LooseBool,
    /// > Only present in difficulty file V3.2 or higher.
    #[serde(rename = "i")]
    pub rotation_dist_easing: Option<Easing>,
    #[serde(rename = "a")]
    pub axis: EventAxis,
    /// If true, the rotation will be mirrored.
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
            rotation_dist_easing: Some(Easing::Linear),
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
    #[deprecated(note = "Experimental. Does not consider random in filter calculations.")]
    #[allow(deprecated)]
    pub fn get_rotation_offset(&self, light_id: i32, group_size: i32) -> f32 {
        self.rotation_dist_type.compute_value_offset(
            light_id,
            group_size,
            &self.filter,
            self.rotation_dist_value,
            self.data.last().map(|data| data.beat_offset),
            self.rotation_dist_easing,
        )
    }
}

/// The lowest-level group event type, which determines the base rotation of the event.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(
    feature = "bevy_reflect",
    derive(bevy_reflect::Reflect),
    reflect(Debug, Clone, PartialEq)
)]
pub struct RotationEventData {
    /// The number of beats the event will be offset from the [`RotationEventBox`]'s beat.
    #[serde(rename = "b")]
    pub beat_offset: f32,
    #[serde(rename = "p")]
    pub transition_type: TransitionType,
    #[serde(rename = "e")]
    pub easing: Easing,
    /// The base number of degrees the event will rotate objects by.
    #[serde(rename = "r")]
    pub degrees: f32,
    #[serde(rename = "o")]
    pub direction: RotationDirection,
    /// Extends the rotation by 360 degrees in the [`RotationDirection`].
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
