use crate::difficulty::playfield::CutDirection;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Waypoints {
    #[serde(rename = "b")]
    pub beat: f32,
    #[serde(rename = "x")]
    pub col: u8,
    #[serde(rename = "y")]
    pub row: u8,
    #[serde(rename = "d")]
    pub direction: CutDirection,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ColorBoostEvent {
    #[serde(rename = "b")]
    pub beat: f32,
    #[serde(rename = "o")]
    pub boost: bool,
}

/// An event containing an array of Special Event Keywords.
/// More info [here](https://bsmg.wiki/mapping/map-format/lightshow.html#special-event-keywords).
#[doc(alias = "KeywordEvent")]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpecialEvent {
    #[serde(rename = "d")]
    pub keywords: Option<Vec<Keyword>>,
}

/// Allows basic event lanes to be overridden with environment-specific behaviour, using secret keys.
/// More info [here](https://bsmg.wiki/mapping/map-format/lightshow.html#special-event-keywords).
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Keyword {
    #[serde(rename = "k")]
    pub keyword: String,
    #[serde(rename = "e")]
    pub event_types: Vec<i32>,
}
