use crate::loose_enum;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    #[serde(rename = "b")]
    pub beat: f32,
    #[serde(rename = "x")]
    pub row: i32,
    #[serde(rename = "y")]
    pub col: i32,
    #[serde(rename = "c")]
    pub color: NoteColor,
    #[serde(rename = "d")]
    pub direction: i32,
    #[serde(rename = "a")]
    pub angle_offset: i32,
}

loose_enum! {
    #[derive(Default)]
    NoteColor {
        #[default]
        Left = 0,
        Right = 1,
    }
}

loose_enum! {
    #[derive(Default)]
    CutDirection {
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
    /// Returns `None` if the cut direction is unknown.
    pub fn get_degrees(&self) -> Option<f32> {
        match self {
            CutDirection::Up => Some(180.0),
            CutDirection::Down => Some(0.0),
            CutDirection::Left => Some(-90.0),
            CutDirection::Right => Some(90.0),
            CutDirection::UpLeft => Some(-135.0),
            CutDirection::UpRight => Some(135.0),
            CutDirection::DownLeft => Some(-45.0),
            CutDirection::DownRight => Some(45.0),
            CutDirection::Any => Some(0.0),
            CutDirection::Unknown(_) => None,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bomb {
    #[serde(rename = "b")]
    pub beat: f32,
    #[serde(rename = "x")]
    pub row: i32,
    #[serde(rename = "y")]
    pub col: i32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Wall {
    #[serde(rename = "b")]
    pub beat: f32,
    #[serde(rename = "d")]
    pub duration: f32,
    #[serde(rename = "x")]
    pub row: i32,
    #[serde(rename = "y")]
    pub col: i32,
    #[serde(rename = "w")]
    pub width: i32,
    #[serde(rename = "h")]
    pub height: i32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Arc {
    #[serde(rename = "b")]
    pub beat: f32,
    #[serde(rename = "c")]
    pub color: i32,
    #[serde(rename = "x")]
    pub row: i32,
    #[serde(rename = "y")]
    pub col: i32,
    pub d: i32,
    pub tb: f32,
    pub tx: i32,
    pub ty: i32,
    pub mu: i32,
    pub tmu: i32,
    pub tc: i32,
    pub m: i32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Chain {
    #[serde(rename = "b")]
    pub beat: f32,
    #[serde(rename = "c")]
    pub color: i32,
    #[serde(rename = "x")]
    pub row: i32,
    #[serde(rename = "y")]
    pub col: i32,
    pub d: i32,
    pub tb: f32,
    pub tx: i32,
    pub ty: i32,
    pub sc: i32,
    pub s: i32,
}
