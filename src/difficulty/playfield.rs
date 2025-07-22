//! The interactable objects of a difficulty.

use crate::{impl_duration, impl_timed, loose_enum};
use serde::{Deserialize, Serialize};

/// The standard block/note.
#[doc(alias = "Block")]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(
    feature = "bevy_reflect",
    derive(bevy_reflect::Reflect),
    reflect(Debug, Clone, PartialEq)
)]
pub struct Note {
    /// The position of the object in time.
    #[serde(rename = "b")]
    pub beat: f32,
    /// A value representing the vertical position of the object.
    /// In the range 0..2 inclusive, with zero being the bottom and two being the top row.
    #[serde(rename = "y")]
    pub row: i32,
    /// A value representing the horizontal position of the object.
    /// In the range 0..3 inclusive, with zero being the far left and three being the far right column.
    #[serde(rename = "x")]
    pub col: i32,
    #[serde(rename = "c")]
    pub color: NoteColor,
    #[serde(rename = "d")]
    pub direction: CutDirection,
    /// The number of degrees counter-clockwise to offset the object by.
    #[serde(rename = "a")]
    pub angle_offset: f32,
}

impl_timed!(Note::beat);

loose_enum! {
    /// The color of a note, which determines which saber should be used to cut it.
    #[derive(Default, Copy)]
    NoteColor: i32 {
        #[default]
        Left = 0,
        Right = 1,
    }
}

loose_enum! {
    /// The direction a note should be cut.
    #[derive(Default, Copy)]
    CutDirection: i32 {
        #[default]
        Up = 0,
        Down = 1,
        Left = 2,
        Right = 3,
        UpLeft = 4,
        UpRight = 5,
        DownLeft = 6,
        DownRight = 7,
        Any = 8,
    }
}

impl CutDirection {
    /// Returns the number of degrees a note is rotated, with zero degrees being a downward note.
    ///
    /// Returns zero if the cut direction is unknown/any.
    pub fn get_degrees(&self) -> f32 {
        match self {
            CutDirection::Up => 180.0,
            CutDirection::Down => 0.0,
            CutDirection::Left => -90.0,
            CutDirection::Right => 90.0,
            CutDirection::UpLeft => -135.0,
            CutDirection::UpRight => 135.0,
            CutDirection::DownLeft => -45.0,
            CutDirection::DownRight => 45.0,
            CutDirection::Any => 0.0,
            CutDirection::Unknown(_) => 0.0,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(
    feature = "bevy_reflect",
    derive(bevy_reflect::Reflect),
    reflect(Debug, Clone, PartialEq)
)]
pub struct Bomb {
    /// The position of the object in time.
    #[serde(rename = "b")]
    pub beat: f32,
    /// A value representing the vertical position of the object.
    /// In the range 0..2 inclusive, with zero being the bottom and two being the top row.
    #[serde(rename = "y")]
    pub row: i32,
    /// A value representing the horizontal position of the object.
    /// In the range 0..3 inclusive, with zero being the far left and three being the far right column.
    #[serde(rename = "x")]
    pub col: i32,
}

impl_timed!(Bomb::beat);

#[doc(alias = "Obstacle")]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(
    feature = "bevy_reflect",
    derive(bevy_reflect::Reflect),
    reflect(Debug, Clone, PartialEq)
)]
pub struct Wall {
    /// The start position of the object in time.
    #[serde(rename = "b")]
    pub beat: f32,
    /// The length (in beats) that an object takes place.
    #[serde(rename = "d")]
    pub duration: f32,
    /// A value representing the vertical position of the object.
    /// In the range 0..2 inclusive, with zero being the bottom and two being the top row.
    #[serde(rename = "y")]
    pub row: i32,
    /// A value representing the horizontal position of the object.
    /// In the range 0..3 inclusive, with zero being the far left and three being the far right column.
    #[serde(rename = "x")]
    pub col: i32,
    #[serde(rename = "w")]
    pub width: i32,
    #[serde(rename = "h")]
    pub height: i32,
}

impl Default for Wall {
    fn default() -> Self {
        Self {
            beat: 0.0,
            duration: 1.0,
            row: 0,
            col: 0,
            width: 1,
            height: 5,
        }
    }
}

impl_duration!(Wall::beat, duration: duration);

#[doc(alias = "Slider")]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(
    feature = "bevy_reflect",
    derive(bevy_reflect::Reflect),
    reflect(Debug, Clone, PartialEq)
)]
pub struct Arc {
    /// The start position of the object in time.
    #[serde(rename = "b")]
    pub beat: f32,
    /// A value representing the vertical starting position of the object.
    /// In the range 0..2 inclusive, with zero being the bottom and two being the top row.
    #[serde(rename = "y")]
    pub row: i32,
    /// A value representing the horizontal starting position of the object.
    /// In the range 0..3 inclusive, with zero being the far left and three being the far right column.
    #[serde(rename = "x")]
    pub col: i32,
    #[serde(rename = "c")]
    pub color: NoteColor,
    #[serde(rename = "d")]
    pub direction: CutDirection,
    /// Controls how far away the starting bezier control point is in [cut direction](Self::direction).
    #[serde(rename = "mu")]
    pub control_point: f32,

    /// The end position of the object in time.
    #[serde(rename = "tb")]
    pub tail_beat: f32,
    /// A value representing the vertical ending position of the object.
    /// In the range 0..2 inclusive, with zero being the bottom and two being the top row.
    #[serde(rename = "ty")]
    pub tail_row: i32,
    /// A value representing the horizontal ending position of the object.
    /// In the range 0..3 inclusive, with zero being the far left and three being the far right column.
    #[serde(rename = "tx")]
    pub tail_col: i32,
    #[serde(rename = "tc")]
    pub tail_direction: CutDirection,
    /// Controls how far away the ending bezier control point is in [tail cut direction](Self::tail_direction).
    #[serde(rename = "tmu")]
    pub tail_control_point: f32,

    #[serde(rename = "m")]
    pub mid_anchor_mode: MidAnchorMode,
}

impl Default for Arc {
    fn default() -> Self {
        Self {
            beat: 0.0,
            row: 0,
            col: 0,
            color: Default::default(),
            direction: Default::default(),
            control_point: 1.0,
            tail_beat: 1.0,
            tail_row: 0,
            tail_col: 0,
            tail_direction: Default::default(),
            tail_control_point: 1.0,
            mid_anchor_mode: Default::default(),
        }
    }
}

impl_duration!(Arc::beat, end: tail_beat);

loose_enum! {
    #[derive(Default, Copy)]
    MidAnchorMode: i32 {
        #[default]
        Straight = 0,
        Clockwise = 1,
        CounterClockwise = 2,
    }
}

#[doc(alias = "BurstSlider")]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(
    feature = "bevy_reflect",
    derive(bevy_reflect::Reflect),
    reflect(Debug, Clone, PartialEq)
)]
pub struct Chain {
    /// The start position of the object in time.
    #[serde(rename = "b")]
    pub beat: f32,
    /// A value representing the vertical starting position of the object.
    /// In the range 0..2 inclusive, with zero being the bottom and two being the top row.
    #[serde(rename = "y")]
    pub row: i32,
    /// A value representing the horizontal starting position of the object.
    /// In the range 0..3 inclusive, with zero being the far left and three being the far right column.
    #[serde(rename = "x")]
    pub col: i32,
    #[serde(rename = "c")]
    pub color: NoteColor,
    #[serde(rename = "d")]
    pub direction: CutDirection,

    /// The end position of the object in time.
    #[serde(rename = "tb")]
    pub tail_beat: f32,
    /// A value representing the vertical ending position of the object.
    /// In the range 0..2 inclusive, with zero being the bottom and two being the top row.
    #[serde(rename = "ty")]
    pub tail_row: i32,
    /// A value representing the horizontal ending position of the object.
    /// In the range 0..3 inclusive, with zero being the far left and three being the far right column.
    #[serde(rename = "tx")]
    pub tail_col: i32,

    /// The number of links the chain has, including the connected [`Note`].
    #[serde(rename = "sc")]
    pub link_count: i32,
    #[serde(rename = "s")]
    pub link_squish: f32,
}

impl Default for Chain {
    fn default() -> Self {
        Self {
            beat: 0.0,
            row: 1,
            col: 0,
            color: Default::default(),
            direction: Default::default(),
            tail_beat: 0.0,
            tail_row: 0,
            tail_col: 0,
            link_count: 3,
            link_squish: 1.0,
        }
    }
}

impl_duration!(Chain::beat, end: tail_beat);
