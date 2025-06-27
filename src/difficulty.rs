pub mod lightshow;
pub mod playfield;

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
    pub rotation_events: Vec<Value>,
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
    pub waypoints: Vec<Value>,
    pub basic_beatmap_events: Vec<Value>,
    pub color_boost_beatmap_events: Vec<ColorBoostEvent>,
    #[serde(rename = "lightColorEventBoxGroups")]
    pub color_event_boxes: Vec<ColorEventBox>,
    #[serde(rename = "lightRotationEventBoxGroups")]
    pub rotation_event_boxes: Vec<RotationEventBoxGroup>,
    #[serde(rename = "lightTranslationEventBoxGroups")]
    pub translation_event_boxes: Vec<Value>,
    pub basic_event_types_with_keywords: BasicEventTypesWithKeywords,
    pub use_normal_events_as_compatible_events: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BpmEvent {
    #[serde(rename = "b")]
    pub beat: f32,
    #[serde(rename = "m")]
    pub bpm: i32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColorBoostEvent {
    #[serde(rename = "b")]
    pub beat: f32,
    pub o: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BasicEventTypesWithKeywords {
    pub d: Vec<Value>,
}
