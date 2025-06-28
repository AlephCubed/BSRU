use crate::difficulty::playfield::CutDirection;
use crate::impl_timed;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(
    feature = "bevy_reflect",
    derive(bevy_reflect::Reflect),
    reflect(Debug, Clone, PartialEq)
)]
pub struct BasicEvent {
    #[serde(rename = "b")]
    pub beat: f32,
    /// More info [here](https://bsmg.wiki/mapping/map-format/lightshow.html#basic-events-type).
    #[serde(rename = "et")]
    pub event_type: i32,
    #[serde(rename = "i")]
    pub value: i32,
    #[serde(rename = "f")]
    pub float: f32,
}

impl_timed!(BasicEvent::beat);

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(
    feature = "bevy_reflect",
    derive(bevy_reflect::Reflect),
    reflect(Debug, Clone, PartialEq)
)]
pub struct Waypoint {
    #[serde(rename = "b")]
    pub beat: f32,
    #[serde(rename = "y")]
    pub row: u8,
    #[serde(rename = "x")]
    pub col: u8,
    #[serde(rename = "d")]
    pub direction: CutDirection,
}

impl_timed!(Waypoint::beat);

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(
    feature = "bevy_reflect",
    derive(bevy_reflect::Reflect),
    reflect(Debug, Clone, PartialEq)
)]
pub struct ColorBoostEvent {
    #[serde(rename = "b")]
    pub beat: f32,
    #[serde(rename = "o")]
    pub boost: bool,
}

impl_timed!(ColorBoostEvent::beat);

/// An event containing an array of Special Event Keywords.
/// More info [here](https://bsmg.wiki/mapping/map-format/lightshow.html#special-event-keywords).
#[doc(alias = "KeywordEvent")]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(
    feature = "bevy_reflect",
    derive(bevy_reflect::Reflect),
    reflect(Debug, Clone, PartialEq)
)]
pub struct SpecialEvent {
    #[serde(rename = "d")]
    pub keywords: Option<Vec<Keyword>>,
}

/// Allows basic event lanes to be overridden with environment-specific behaviour, using secret keys.
/// More info [here](https://bsmg.wiki/mapping/map-format/lightshow.html#special-event-keywords).
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(
    feature = "bevy_reflect",
    derive(bevy_reflect::Reflect),
    reflect(Debug, Clone, PartialEq)
)]
pub struct Keyword {
    #[serde(rename = "k")]
    pub keyword: String,
    #[serde(rename = "e")]
    pub event_types: Vec<i32>,
}
