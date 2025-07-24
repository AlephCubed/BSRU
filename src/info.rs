//! Defines the structure of a map's `Info.dat` file.

pub mod color_scheme;

#[doc(hidden)]
pub use color_scheme::*;

use crate::loose_enum;
use serde::{Deserialize, Serialize};

/// A map's `Info.dat` file.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(
    feature = "bevy_reflect",
    derive(bevy_reflect::Reflect),
    reflect(Debug, Clone, PartialEq)
)]
pub struct Beatmap {
    /// The info file version, in the form of `2.1.0`.
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
    /// The path to the audio file, relative to the map's folder.
    #[serde(rename = "_songFilename")]
    pub audio_file: String,
    /// The path to the cover image file, relative to the map's folder.
    #[serde(rename = "_coverImageFilename")]
    pub cover_image_file: String,
    /// The environment that will be used for 90 and 360 degree difficulties.
    ///
    /// Starting in info file V2.1, individual difficulties can override this using [environment_index](DifficultyInfo::environment_index).
    #[serde(rename = "_environmentName")]
    pub environment: Environment,
    /// The environment that will be used for 90 and 360 degree difficulties.
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
    /// The world that surrounds the player and defines which lights are available.
    ///
    /// For 90/360 degree mode, see [`AllDirectionEnvironment`].
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
    /// The world that surrounds the player while playing 90/360 degree mode.
    ///
    /// For standard mode, see [`Environment`].
    #[derive(Default)]
    AllDirectionEnvironment: String {
        #[default]
        GlassDesert = "GlassDesertEnvironment",
    }
}

/// Describes a group of difficulties, all with the same [characteristic/mode](Characteristic).
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
    /// Describes the type/game mode of a difficulty.
    ///
    /// Note that [`Lawless`](Self::Lawless) and [`Lightshow`](Self::Lightshow) are modded characteristics,
    /// and may cause problems in un-modded versions of the game.
    #[derive(Default)]
    Characteristic: String {
        #[default]
        Standard = "Standard",
        NoArrows = "NoArrows",
        OneSaber = "OneSaber",
        Rotate360 = "360Degree",
        Rotate90 = "90Degree",
        Legacy = "Legacy",
        /// > Modded characteristic. May cause problems in un-modded versions of the game.
        Lawless = "Lawless",
        /// > Modded characteristic. May cause problems in un-modded versions of the game.
        Lightshow = "Lightshow",
    }
}

/// Describes the settings for a difficulty.
///
/// Note that a difficulties [characteristic](Characteristic) is defined by its [`DifficultySet`].
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
    /// The path to the difficulty file, relative to the map's folder.
    #[serde(rename = "_beatmapFilename")]
    pub file: String,
    #[doc(alias = "node_jump_speed")]
    #[serde(rename = "_noteJumpMovementSpeed")]
    pub njs: f32,
    #[doc(alias = "node_jump_distance")]
    #[serde(rename = "_noteJumpStartBeatOffset")]
    pub njd: f32,
    /// > Only present in info file V2.1 or higher.
    ///
    /// The ID of environment to use from the map's [environment list](Beatmap::environments).
    #[serde(rename = "_environmentNameIdx")]
    pub environment_index: Option<i32>,
    /// > Only present in info file V2.1 or higher.
    ///
    /// The ID of color scheme to use from the map's [color schemes list](Beatmap::color_schemes).
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
