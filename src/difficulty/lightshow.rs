use serde::{Deserialize, Serialize};

pub mod color;
pub mod rotation;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Filter {
    pub f: i32,
    pub p: i32,
    pub t: i32,
    pub r: i32,
    #[serde(rename = "c")]
    pub color: i32,
    pub n: i32,
    pub s: i32,
    pub l: i32,
    pub d: i32,
}
