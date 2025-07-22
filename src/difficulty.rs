//! Defines the structure of a map's difficulty file(s) (i.e. `ExpertStandard.dat`).

pub mod gameplay_event;
pub mod lightshow;
pub mod playfield;

pub use gameplay_event::*;
pub use lightshow::*;
pub use playfield::*;

use serde::{Deserialize, Serialize};

/// A map's difficulty file(s) (i.e. `ExpertStandard.dat`).
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "bevy_reflect",
    derive(bevy_reflect::Reflect),
    reflect(Debug, Clone, PartialEq)
)]
pub struct Difficulty {
    /// The difficulty file version, in the form of `3.2.0`.
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
    /// If false, overriding the environment in game will disable all lightshow events.
    #[serde(rename = "useNormalEventsAsCompatibleEvents")]
    pub use_compatible_events: bool,
}
