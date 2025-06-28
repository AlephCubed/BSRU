use crate::impl_timed;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LaneRotationEvent {
    #[serde(rename = "b")]
    pub beat: f32,
    #[serde(rename = "e")]
    pub execution_time: i32,
    #[serde(rename = "r")]
    pub degrees: f32,
}

impl_timed!(LaneRotationEvent::beat);

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BpmEvent {
    #[serde(rename = "b")]
    pub beat: f32,
    #[serde(rename = "m")]
    pub bpm: f32,
}

impl_timed!(BpmEvent::beat);
