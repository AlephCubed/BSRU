use crate::difficulty::lightshow::Filter;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColorEventBox {
    #[serde(rename = "b")]
    pub beat: f32,
    #[serde(rename = "g")]
    pub group_id: i32,
    #[serde(rename = "e")]
    pub groups: Vec<ColorEventGroup>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColorEventGroup {
    #[serde(rename = "f")]
    pub filter: Filter,
    pub w: f32,
    pub d: i32,
    pub r: f32,
    pub t: i32,
    #[serde(rename = "b")]
    pub beat: i32,
    pub i: i32,
    #[serde(rename = "e")]
    pub data: Vec<ColorEventData>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColorEventData {
    #[serde(rename = "b")]
    pub beat: f32,
    pub i: i32,
    #[serde(rename = "c")]
    pub color: i32,
    pub s: f32,
    pub f: i32,
}
