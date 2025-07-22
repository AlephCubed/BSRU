use crate::difficulty::playfield::CutDirection;
use crate::impl_timed;
use serde::{Deserialize, Serialize};

/// The basic V2 event type, which is still used for some elements of V3 environments (for example, the player platform).
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(
    feature = "bevy_reflect",
    derive(bevy_reflect::Reflect),
    reflect(Debug, Clone, PartialEq)
)]
pub struct BasicEvent {
    /// The time the event takes place.
    #[serde(rename = "b")]
    pub beat: f32,
    /// Determines the behaviour of the event. The exact behaviour differs depending on the environment.
    ///
    /// More info [here](https://bsmg.wiki/mapping/map-format/lightshow.html#basic-events-type).
    #[serde(rename = "et")]
    pub event_type: i32,
    /// Determines which effect the event will produce, based on its [type](Self::event_type).
    #[serde(rename = "i")]
    pub value: i32,
    /// Modifies the effect.
    #[serde(rename = "f")]
    pub float: f32,
}

impl_timed!(BasicEvent::beat);

/// Controls the TinyTAN figures on the [BTS environment](crate::info::Environment::BTS).
///
/// More info [here](https://bsmg.wiki/mapping/map-format/lightshow.html#waypoints).
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(
    feature = "bevy_reflect",
    derive(bevy_reflect::Reflect),
    reflect(Debug, Clone, PartialEq)
)]
pub struct Waypoint {
    /// The time the event takes place.
    #[serde(rename = "b")]
    pub beat: f32,
    /// A value representing the vertical position of the event.
    /// In the range 0..2 inclusive, with zero being the bottom and two being the top row.
    #[serde(rename = "y")]
    pub row: u8,
    /// A value representing the horizontal position of the event.
    /// In the range 0..3 inclusive, with zero being the far left and three being the far right column.
    #[serde(rename = "x")]
    pub col: u8,
    #[serde(rename = "d")]
    pub direction: CutDirection,
}

impl_timed!(Waypoint::beat);

/// Controls which lighting colors are used, based on a map or environment's [color scheme](crate::info::color_scheme::ColorScheme).
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(
    feature = "bevy_reflect",
    derive(bevy_reflect::Reflect),
    reflect(Debug, Clone, PartialEq)
)]
pub struct ColorBoostEvent {
    /// The time the event takes place.
    #[serde(rename = "b")]
    pub beat: f32,
    /// Whether to enable or disable boost colors.
    #[serde(rename = "o")]
    pub boost: bool,
}

impl_timed!(ColorBoostEvent::beat);

/// An event containing an array of Special Event Keywords.
///
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
///
/// More info [here](https://bsmg.wiki/mapping/map-format/lightshow.html#special-event-keywords).
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(
    feature = "bevy_reflect",
    derive(bevy_reflect::Reflect),
    reflect(Debug, Clone, PartialEq)
)]
pub struct Keyword {
    /// The secret key of the effect.
    #[serde(rename = "k")]
    pub keyword: String,
    /// A list of [event types](BasicEvent::event_type) to effect with the keyword.
    #[serde(rename = "e")]
    pub event_types: Vec<i32>,
}
