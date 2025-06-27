pub mod gameplay_event;
pub mod lightshow;
pub mod playfield;

use crate::difficulty::gameplay_event::{BpmEvent, LaneRotationEvent};
use crate::difficulty::lightshow::basic::{BasicEvent, ColorBoostEvent, SpecialEvent, Waypoints};
use crate::difficulty::lightshow::color::ColorEventBox;
use crate::difficulty::lightshow::rotation::RotationEventBoxGroup;
use crate::difficulty::playfield::{Arc, Bomb, Chain, Note, Wall};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DifficultyV3_2 {
    pub version: String,
    pub bpm_events: Vec<BpmEvent>,
    #[serde(rename = "rotationEvents")]
    pub lane_rotation_events: Vec<LaneRotationEvent>,
    #[serde(rename = "colorNotes")]
    pub notes: Vec<Note>,
    #[serde(rename = "bombNotes")]
    pub bombs: Vec<Bomb>,
    #[serde(rename = "obstacles")]
    pub walls: Vec<Wall>,
    #[serde(rename = "sliders")]
    pub arcs: Vec<Arc>,
    #[serde(rename = "burstSliders")]
    pub chains: Vec<Chain>,
    pub waypoints: Vec<Waypoints>,
    #[serde(rename = "basicBeatmapEvents")]
    pub basic_events: Vec<BasicEvent>,
    #[serde(rename = "colorBoostBeatmapEvents")]
    pub color_boost_events: Vec<ColorBoostEvent>,
    #[serde(rename = "lightColorEventBoxGroups")]
    pub color_event_boxes: Vec<ColorEventBox>,
    #[serde(rename = "lightRotationEventBoxGroups")]
    pub rotation_event_boxes: Vec<RotationEventBoxGroup>,
    #[serde(rename = "lightTranslationEventBoxGroups")]
    pub translation_event_boxes: Vec<Value>, // Todo
    #[doc(alias = "keyword_events")]
    #[serde(rename = "basicEventTypesWithKeywords")]
    pub special_events: SpecialEvent,
    pub use_normal_events_as_compatible_events: bool,
}
