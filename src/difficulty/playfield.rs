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
    pub color: i32,
    #[serde(rename = "d")]
    pub direction: i32,
    #[serde(rename = "a")]
    pub angle_offset: i32,
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
