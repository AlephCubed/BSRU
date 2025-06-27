use crate::loose_enum;
use crate::macros::LooseBool;
use serde::{Deserialize, Serialize};

/// Controls which light indices are affected by event boxes.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Filter {
    // V3
    #[serde(rename = "f")]
    pub filter_type: FilterType,
    /// Dependent on the [`FilterType`]
    #[serde(rename = "p")]
    pub parameter1: i32,
    /// Dependent on the [`FilterType`]
    #[serde(rename = "t")]
    pub parameter2: i32,
    #[serde(rename = "r")]
    pub reverse: LooseBool,
    // V3.1
    #[serde(rename = "c")]
    pub chunks: i32,
    #[serde(rename = "n")]
    pub random_behaviour: i32,
    #[serde(rename = "s")]
    pub random_seed: i32,
    #[serde(rename = "d")]
    pub limit_behaviour: i32,
    #[serde(rename = "l")]
    pub limit_percent: i32,
}

loose_enum! {
    /// The parameters of a [Filter] do different things depending on the type.
    ///
    /// ### [Division](https://bsmg.wiki/mapping/map-format/lightshow.html#index-filters-type-1):
    /// Splits the group up into equal sections and selects one.
    /// - Parameter 1 determines the number of sections.
    ///   It will be rounded up to the nearest multiple of the group size.
    /// - Parameter 2 determines the section to select, starting at 0.
    ///
    /// ### [Step and Offset](https://bsmg.wiki/mapping/map-format/lightshow.html#index-filters-type-2):
    /// Alternates selecting and not selecting lights.
    /// - Parameter 1 is the index of the first light that will be selected, starting at 0.
    /// - Parameter 2 determines the number of lights that will be skipped between selections.
    #[derive(Default)]
    FilterType {
        #[default]
        //Todo Doesn't match wiki
        Division = 1,
        StepAndOffset = 2,
    }
}
