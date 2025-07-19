use crate::{impl_timed, loose_enum};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(
    feature = "bevy_reflect",
    derive(bevy_reflect::Reflect),
    reflect(Debug, Clone, PartialEq)
)]
pub struct LaneRotationEvent {
    #[serde(rename = "b")]
    pub beat: f32,
    #[serde(rename = "e")]
    pub execution_time: ExecutionTime,
    #[serde(rename = "r")]
    pub degrees: f32,
}

impl_timed!(LaneRotationEvent::beat);

loose_enum!(
    /// Determines when a [`LaneRotationEvent`] will be applied to objects placed on the same beat as this event.
    #[derive(Default, Copy)]
    ExecutionTime: i32 {
        /// The [`LaneRotationEvent`] will affect objects with a beat greater than or equal to the event's beat.
        #[default]
        Early = 0,
        /// The [`LaneRotationEvent`] will affect objects with a beat greater than the event's beat.
        Late = 1,
    }
);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(
    feature = "bevy_reflect",
    derive(bevy_reflect::Reflect),
    reflect(Debug, Clone, PartialEq)
)]
pub struct BpmEvent {
    #[serde(rename = "b")]
    pub beat: f32,
    #[serde(rename = "m")]
    pub bpm: f32,
}

impl Default for BpmEvent {
    fn default() -> Self {
        Self {
            beat: 0.0,
            bpm: 100.0,
        }
    }
}

impl_timed!(BpmEvent::beat);
