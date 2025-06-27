use crate::loose_enum;
use serde::{Deserialize, Serialize};

pub mod color;
mod easing;
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

loose_enum! {
    /// The distribution value does different things depending on the type.
    ///
    /// # [Beat Distribution](https://bsmg.wiki/mapping/map-format/lightshow.html#light-color-event-boxes-beat-distribution):
    /// ### Wave:
    /// The value represents the total time for all steps to complete.
    /// ### Step:
    /// The value represents the time until the next step is completed.
    ///
    /// # [Brightness](https://bsmg.wiki/mapping/map-format/lightshow.html#light-color-event-boxes-effect-distribution) and [Rotation Distribution](https://bsmg.wiki/mapping/map-format/lightshow.html#light-rotation-event-boxes-effect-distribution):
    /// ### Wave:
    /// The value represents the total difference between the first and last step.
    /// ### Step:
    /// The value represents the different between the current and next step.
    #[derive(Default)]
    DistributionType {
        #[default]
        Wave = 1,
        Step = 2,
    }
}
