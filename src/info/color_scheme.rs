//! Describes the colors of objects and lights for an environment/map.

pub mod presets;

#[allow(unused_imports)]
#[doc(hidden)]
pub use presets::*;

use serde::{Deserialize, Serialize};
use std::string::ToString;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(
    feature = "bevy_reflect",
    derive(bevy_reflect::Reflect),
    reflect(Debug, Clone, PartialEq)
)]
#[serde(rename_all = "camelCase")]
pub struct ColorSchemeOverride {
    pub use_override: bool,
    pub color_scheme: ColorScheme,
}

/// The colors of objects and lights for an environment/map.
///
/// This does *not* currently support while light color overrides.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(
    feature = "bevy_reflect",
    derive(bevy_reflect::Reflect),
    reflect(Debug, Clone, PartialEq)
)]
#[serde(rename_all = "camelCase")]
pub struct ColorScheme {
    /// The name of the color scheme.
    #[serde(rename = "colorSchemeId")]
    pub id: String,
    /// The color for the left saber/notes. Default is red.
    #[doc(alias = "saber_left")]
    #[serde(rename = "saberAColor")]
    pub note_left: Color,
    /// The color for the right saber/notes. Default is blue.
    #[doc(alias = "saber_right")]
    #[serde(rename = "saberBColor")]
    pub note_right: Color,

    /// The color of walls/obstacles.
    #[doc(alias = "obstacle")]
    #[serde(rename = "obstaclesColor")]
    pub wall: Color,

    /// The primary light color, often matching the [left note color](Self::note_left).
    #[doc(alias = "environment0")]
    #[serde(rename = "environmentColor0")]
    pub light_primary: Color,
    /// The secondary light color, often matching the [right note color](Self::note_right).
    #[doc(alias = "environment1")]
    #[serde(rename = "environmentColor1")]
    pub light_secondary: Color,

    /// The primary light color when [boost colors](crate::ColorBoostEvent) are enabled.
    #[doc(alias = "environment_boost_0")]
    #[serde(rename = "environmentColor0Boost")]
    pub boost_light_primary: Color,
    /// The secondary light color when [boost colors](crate::ColorBoostEvent) are enabled.
    #[doc(alias = "environment_boost_1")]
    #[serde(rename = "environmentColor1Boost")]
    pub boost_light_secondary: Color,
}

impl Default for ColorScheme {
    fn default() -> Self {
        ColorScheme {
            id: "Default".to_string(),
            note_left: Color {
                red: 0.7843137,
                green: 0.07843138,
                blue: 0.07843138,
                alpha: 1.0,
            },
            note_right: Color {
                red: 0.1568627,
                green: 0.5568627,
                blue: 0.8235294,
                alpha: 1.0,
            },
            wall: Color {
                red: 1.0,
                green: 0.1882353,
                blue: 0.1882353,
                alpha: 1.0,
            },
            light_primary: Color {
                red: 0.85,
                green: 0.08499997,
                blue: 0.08499997,
                alpha: 1.0,
            },
            light_secondary: Color {
                red: 0.1882353,
                green: 0.675294,
                blue: 1.0,
                alpha: 1.0,
            },
            boost_light_primary: Default::default(),
            boost_light_secondary: Default::default(),
        }
    }
}

/// The color of an object/light.
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(
    feature = "bevy_reflect",
    derive(bevy_reflect::Reflect),
    reflect(Debug, Clone, PartialEq)
)]
pub struct Color {
    #[serde(rename = "r")]
    red: f32,
    #[serde(rename = "g")]
    green: f32,
    #[serde(rename = "b")]
    blue: f32,
    #[serde(rename = "a")]
    alpha: f32,
}

impl Default for Color {
    fn default() -> Self {
        Self {
            red: 1.0,
            green: 1.0,
            blue: 1.0,
            alpha: 1.0,
        }
    }
}

#[cfg(feature = "bevy_color")]
mod color_conversions {
    use crate::info::color_scheme::Color;
    use bevy_color::Srgba;

    impl From<Srgba> for Color {
        fn from(value: Srgba) -> Self {
            Self {
                red: value.red,
                green: value.green,
                blue: value.blue,
                alpha: value.alpha,
            }
        }
    }

    impl From<Color> for Srgba {
        fn from(value: Color) -> Self {
            Self {
                red: value.red,
                green: value.green,
                blue: value.blue,
                alpha: value.alpha,
            }
        }
    }
}
