//! Events that effect gameplay and aren't purely visual.

use crate::impl_timed;
use loose_enum::loose_enum;
use serde::{Deserialize, Serialize};

/// Controls the rotation that interactable objects spawn in 90/360 degree difficulties.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(
    feature = "bevy_reflect",
    derive(bevy_reflect::Reflect),
    reflect(Debug, Clone, PartialEq)
)]
pub struct LaneRotationEvent {
    /// The time the event takes place.
    #[serde(rename = "b")]
    pub beat: f32,
    #[serde(rename = "e")]
    pub execution_time: ExecutionTime,
    /// The number of degrees to rotate objects around the player.
    #[serde(rename = "r")]
    pub degrees: f32,
}

impl_timed!(LaneRotationEvent::beat);

loose_enum!(
    /// Determines when a [`LaneRotationEvent`] will be applied to objects.
    #[derive(Default, Debug, Clone, Copy, Eq, PartialEq, Hash)]
    #[cfg_attr(
        feature = "bevy_reflect",
        derive(bevy_reflect::Reflect),
        reflect(Debug, Clone, PartialEq)
    )]
    pub enum ExecutionTime: i32 {
        /// The [`LaneRotationEvent`] will affect objects with a beat *greater than or equal to* the event's beat.
        #[default]
        Early = 0,
        /// The [`LaneRotationEvent`] will affect objects with a beat *greater than* the event's beat.
        Late = 1,
    }
);

/// Changes the BPM of the map at a specific beat.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(
    feature = "bevy_reflect",
    derive(bevy_reflect::Reflect),
    reflect(Debug, Clone, PartialEq)
)]
pub struct BpmEvent {
    /// The time the event takes place.
    #[serde(rename = "b")]
    pub beat: f32,
    /// The BPM to change the map too.
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
