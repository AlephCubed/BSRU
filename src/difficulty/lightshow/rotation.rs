use crate::difficulty::lightshow::Filter;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RotationEventBoxGroup {
    #[serde(rename = "b")]
    pub beat: f32,
    #[serde(rename = "g")]
    pub group_id: i32,
    #[serde(rename = "e")]
    pub groups: Vec<RotationEventGroup>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RotationEventGroup {
    pub f: Filter,
    pub w: f32,
    pub d: i32,
    pub s: i32,
    pub t: i32,
    #[serde(rename = "b")]
    pub beat: i32,
    pub a: i32,
    pub r: i32,
    pub i: i32,
    #[serde(rename = "l")]
    pub data: Vec<RotationEventData>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RotationEventData {
    #[serde(rename = "b")]
    pub beat: f32,
    pub p: i32,
    pub e: i32,
    pub l: i32,
    pub r: i32,
    pub o: i32,
}
