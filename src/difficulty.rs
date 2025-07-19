pub mod gameplay_event;
pub mod lightshow;
pub mod playfield;

use crate::difficulty::gameplay_event::{BpmEvent, LaneRotationEvent};
use crate::difficulty::lightshow::basic::{BasicEvent, ColorBoostEvent, SpecialEvent, Waypoint};
use crate::difficulty::playfield::{Arc, Bomb, Chain, Note, Wall};
use lightshow::boxes::color::ColorEventBox;
use lightshow::boxes::rotation::RotationEventBox;
use lightshow::boxes::translation::TranslationEventBox;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "bevy_reflect",
    derive(bevy_reflect::Reflect),
    reflect(Debug, Clone, PartialEq)
)]
pub struct Difficulty {
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
    pub waypoints: Vec<Waypoint>,
    #[serde(rename = "basicBeatmapEvents")]
    pub basic_events: Vec<BasicEvent>,
    #[serde(rename = "colorBoostBeatmapEvents")]
    pub color_boost_events: Vec<ColorBoostEvent>,
    #[serde(rename = "lightColorEventBoxGroups")]
    pub color_event_boxes: Vec<ColorEventBox>,
    #[serde(rename = "lightRotationEventBoxGroups")]
    pub rotation_event_boxes: Vec<RotationEventBox>,
    /// > Only present in difficulty file V3.2 or higher.
    #[serde(rename = "lightTranslationEventBoxGroups")]
    pub translation_event_boxes: Option<Vec<TranslationEventBox>>,
    #[doc(alias = "keyword_events")]
    #[serde(rename = "basicEventTypesWithKeywords")]
    pub special_events: SpecialEvent,
    #[serde(rename = "useNormalEventsAsCompatibleEvents")]
    pub use_compatible_events: bool,
}
