pub mod color_scheme;

use crate::info::color_scheme::ColorSchemeOverride;
use crate::loose_enum;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(
    feature = "bevy_reflect",
    derive(bevy_reflect::Reflect),
    reflect(Debug, Clone, PartialEq)
)]
pub struct Beatmap {
    #[serde(rename = "_version")]
    pub version: String,
    #[serde(rename = "_songName")]
    pub name: String,
    #[serde(rename = "_songSubName")]
    pub sub_name: String,
    #[serde(rename = "_songAuthorName")]
    pub artist: String,
    #[serde(rename = "_levelAuthorName")]
    pub mapper: String,
    #[serde(rename = "_beatsPerMinute")]
    pub bpm: f32,
    #[serde(rename = "_songTimeOffset")]
    pub time_offset: f32,
    #[serde(rename = "_shuffle")]
    pub shuffle: f32,
    #[serde(rename = "_shufflePeriod")]
    pub shuffle_period: f32,
    #[serde(rename = "_previewStartTime")]
    pub preview_start_time: f32,
    #[serde(rename = "_previewDuration")]
    pub preview_duration: f32,
    #[serde(rename = "_songFilename")]
    pub audio_file: String,
    #[serde(rename = "_coverImageFilename")]
    pub cover_image_file: String,
    #[serde(rename = "_environmentName")]
    pub environment: Environment,
    #[serde(rename = "_allDirectionsEnvironmentName")]
    pub all_directions_environment: AllDirectionEnvironment,
    /// > Only present in info file V2.1 or higher.
    #[serde(rename = "_environmentNames")]
    pub environments: Option<Vec<Environment>>,
    /// > Only present in info file V2.1 or higher.
    #[serde(rename = "_colorSchemes")]
    pub color_schemes: Option<Vec<ColorSchemeOverride>>,
    #[serde(rename = "_difficultyBeatmapSets")]
    pub difficulty_sets: Vec<DifficultySet>,
}

loose_enum! {
    #[derive(Default)]
    Environment: String {
        #[default]
        #[doc(alias = "Default")]
        TheFirst = "DefaultEnvironment",

        Triangle = "TriangleEnvironment",
        Nice = "NiceEnvironment",
        BigMirror = "BigMirrorEnvironment",
        KDA = "KDAEnvironment",
        Monstercat = "MonstercatEnvironment",
        CrabRave = "CrabRaveEnvironment",
        ImagineDragons = "DragonsEnvironment",
        Origins = "OriginsEnvironment",
        PanicAtTheDisco = "PanicEnvironment",
        RocketLeague = "RocketEnvironment",
        GreenDay = "GreenDayEnvironment",
        GreenDayGrenade = "GreenDayGrenadeEnvironment",
        Timbaland = "TimbalandEnvironment",
        FitBeat = "FitBeatEnvironment",
        LinkinPark = "LinkinParkEnvironment",
        BTS = "BTSEnvironment",
        Kaleidoscope = "KaleidoscopeEnvironment",
        Interscope = "InterscopeEnvironment",
        Skrillex = "SkrillexEnvironment",
        BillieEilish = "BillieEnvironment",
        #[doc(alias = "Halloween")]
        Spooky = "HalloweenEnvironment",
        LadyGaga = "GagaEnvironment",
        // V3:
        Weave = "WeaveEnvironment",
        #[doc(alias = "Pyro")]
        FallOutBoy = "PyroEnvironment",
        EDM = "EDMEnvironment",
        TheSecond = "TheSecondEnvironment",
        Lizzo = "LizzoEnvironment",
        TheWeeknd = "TheWeekndEnvironment",
        RockMixtape = "RockMixtapeEnvironment",
        Dragons2 = "Dragons2Environment",
        PanicAtTheDisco2 = "Panic2Environment",
        Queen = "QueenEnvironment",
        LinkinPark2 = "LinkinPark2Environment",
        TheRollingStones = "TheRollingStonesEnvironment",
        Lattice = "LatticeEnvironment",
        DaftPunk = "DaftPunkEnvironment",
        HipHop = "HipHopEnvironment",
        Collider = "ColliderEnvironment",
        BritneySpears = "BritneyEnvironment",
        Monstercat2 = "Monstercat2Environment",
        Metallica = "MetallicaEnvironment",
    }
}

loose_enum! {
    #[derive(Default)]
    AllDirectionEnvironment: String {
        #[default]
        GlassDesert = "GlassDesertEnvironment",
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(
    feature = "bevy_reflect",
    derive(bevy_reflect::Reflect),
    reflect(Debug, Clone, PartialEq)
)]
pub struct DifficultySet {
    #[serde(rename = "_beatmapCharacteristicName")]
    pub characteristic: Characteristic,
    #[serde(rename = "_difficultyBeatmaps")]
    pub difficulties: Vec<DifficultyInfo>,
}

loose_enum! {
    #[derive(Default)]
    Characteristic: String {
        #[default]
        Standard = "Standard",
        NoArrows = "NoArrows",
        OneSaber = "OneSaber",
        Rotate360 = "360Degree",
        Rotate90 = "90Degree",
        Legacy = "Legacy",
        //Custom types.
        Lawless = "Lawless",
        Lightshow = "Lightshow",
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(
    feature = "bevy_reflect",
    derive(bevy_reflect::Reflect),
    reflect(Debug, Clone, PartialEq)
)]
pub struct DifficultyInfo {
    #[serde(rename = "_difficulty")]
    pub name: String,
    #[serde(rename = "_difficultyRank")]
    pub rank: DifficultyRank,
    #[serde(rename = "_beatmapFilename")]
    pub file: String,
    #[doc(alias = "node_jump_speed")]
    #[serde(rename = "_noteJumpMovementSpeed")]
    pub njs: f32,
    #[doc(alias = "node_jump_distance")]
    #[serde(rename = "_noteJumpStartBeatOffset")]
    pub njd: f32,
    /// > Only present in info file V2.1 or higher.
    #[serde(rename = "_beatmapColorSchemeIdx")]
    pub color_scheme_index: Option<i32>,
}

loose_enum! {
    #[derive(Default, Copy)]
    DifficultyRank: i32 {
        Easy = 1,
        Normal = 3,
        Hard = 5,
        #[default]
        Expert = 7,
        ExpertPlus = 9,
    }
}
