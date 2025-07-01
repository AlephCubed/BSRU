pub mod presets;

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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(
    feature = "bevy_reflect",
    derive(bevy_reflect::Reflect),
    reflect(Debug, Clone, PartialEq)
)]
#[serde(rename_all = "camelCase")]
pub struct ColorScheme {
    #[serde(rename = "colorSchemeId")]
    pub id: String,
    #[doc(alias = "saber_left")]
    #[serde(rename = "saberAColor")]
    pub note_left: Color,
    #[doc(alias = "saber_right")]
    #[serde(rename = "saberBColor")]
    pub note_right: Color,

    #[doc(alias = "obstacle")]
    #[serde(rename = "obstaclesColor")]
    pub wall: Color,

    #[doc(alias = "environment0")]
    #[serde(rename = "environmentColor0")]
    pub light_primary: Color,
    #[doc(alias = "environment1")]
    #[serde(rename = "environmentColor1")]
    pub light_secondary: Color,

    #[doc(alias = "environment_boost_0")]
    #[serde(rename = "environmentColor0Boost")]
    pub boost_light_primary: Color,
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
