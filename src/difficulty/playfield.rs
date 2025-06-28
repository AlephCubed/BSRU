use crate::{impl_duration, impl_timed, loose_enum};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(
    feature = "bevy_reflect",
    derive(bevy_reflect::Reflect),
    reflect(Debug, Clone, PartialEq)
)]
pub struct Note {
    #[serde(rename = "b")]
    pub beat: f32,
    #[serde(rename = "y")]
    pub row: i32,
    #[serde(rename = "x")]
    pub col: i32,
    #[serde(rename = "c")]
    pub color: NoteColor,
    #[serde(rename = "d")]
    pub direction: CutDirection,
    #[serde(rename = "a")]
    pub angle_offset: f32,
}

impl_timed!(Note::beat);

loose_enum! {
    #[derive(Default, Copy)]
    NoteColor: i32 {
        #[default]
        Left = 0,
        Right = 1,
    }
}

loose_enum! {
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
    /// Returns the number of degrees a note is rotated, with zero degrees being downward note.
    /// Returns zero if the cut direction is unknown.
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
    #[serde(rename = "b")]
    pub beat: f32,
    #[serde(rename = "y")]
    pub row: i32,
    #[serde(rename = "x")]
    pub col: i32,
}

impl_timed!(Bomb::beat);

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(
    feature = "bevy_reflect",
    derive(bevy_reflect::Reflect),
    reflect(Debug, Clone, PartialEq)
)]
pub struct Wall {
    #[serde(rename = "b")]
    pub beat: f32,
    #[serde(rename = "d")]
    pub duration: f32,
    #[serde(rename = "y")]
    pub row: i32,
    #[serde(rename = "x")]
    pub col: i32,
    #[serde(rename = "w")]
    pub width: i32,
    #[serde(rename = "h")]
    pub height: i32,
}

impl_duration!(Wall::beat, duration: duration);

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(
    feature = "bevy_reflect",
    derive(bevy_reflect::Reflect),
    reflect(Debug, Clone, PartialEq)
)]
pub struct Arc {
    #[serde(rename = "b")]
    pub beat: f32,
    #[serde(rename = "y")]
    pub row: i32,
    #[serde(rename = "x")]
    pub col: i32,
    #[serde(rename = "c")]
    pub color: NoteColor,
    #[serde(rename = "d")]
    pub direction: CutDirection,
    #[serde(rename = "mu")]
    pub control_point: f32,

    #[serde(rename = "tb")]
    pub tail_beat: f32,
    #[serde(rename = "ty")]
    pub tail_row: i32,
    #[serde(rename = "tx")]
    pub tail_col: i32,
    #[serde(rename = "tc")]
    pub tail_direction: CutDirection,
    #[serde(rename = "tmu")]
    pub tail_control_point: f32,

    #[serde(rename = "m")]
    pub mid_anchor_mode: MidAnchorMode,
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(
    feature = "bevy_reflect",
    derive(bevy_reflect::Reflect),
    reflect(Debug, Clone, PartialEq)
)]
pub struct Chain {
    #[serde(rename = "b")]
    pub beat: f32,
    #[serde(rename = "y")]
    pub row: i32,
    #[serde(rename = "x")]
    pub col: i32,
    #[serde(rename = "c")]
    pub color: NoteColor,
    #[serde(rename = "d")]
    pub direction: CutDirection,

    #[serde(rename = "tb")]
    pub tail_beat: f32,
    #[serde(rename = "ty")]
    pub tail_row: i32,
    #[serde(rename = "tx")]
    pub tail_col: i32,

    #[serde(rename = "sc")]
    pub link_count: i32,
    #[serde(rename = "s")]
    pub link_squish: f32,
}

impl_duration!(Chain::beat, end: tail_beat);
