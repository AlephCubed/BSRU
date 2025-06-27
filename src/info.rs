use crate::loose_enum;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InfoV2 {
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
    pub bpm: i32,
    #[serde(rename = "_songTimeOffset")]
    pub time_offset: i32,
    #[serde(rename = "_shuffle")]
    pub shuffle: i32,
    #[serde(rename = "_shufflePeriod")]
    pub shuffle_period: i32,
    #[serde(rename = "_previewStartTime")]
    pub preview_start_time: f32,
    #[serde(rename = "_previewDuration")]
    pub preview_duration: i32,
    #[serde(rename = "_songFilename")]
    pub audio_file: String,
    #[serde(rename = "_coverImageFilename")]
    pub cover_image_file: String,
    #[serde(rename = "_environmentName")]
    pub environment: Environment,
    #[serde(rename = "_allDirectionsEnvironmentName")]
    pub all_directions_environment: AllDirectionEnvironment,
    #[serde(rename = "_environmentNames")]
    pub environments: Vec<Environment>,
    #[serde(rename = "_colorSchemes")]
    pub color_schemes: Vec<Value>, // Todo
    #[serde(rename = "_difficultyBeatmapSets")]
    pub difficulty_sets: Vec<DifficultySet>,
}

// Todo: Serde rename is not supported by macro.
loose_enum! {
    #[derive(Default)]
    Environment {
        #[doc(alias = "TheFirst")]
        #[default]
        DefaultEnvironment = 0,

        TriangleEnvironment = 1,
        NiceEnvironment = 2,
        BigMirrorEnvironment = 3,
        KDAEnvironment = 4,
        MonstercatEnvironment = 5,
        CrabRaveEnvironment = 6,
        DragonsEnvironment = 7,
        OriginsEnvironment = 8,
        PanicEnvironment = 9,
        RocketEnvironment = 10,
        GreenDayEnvironment = 11,
        GreenDayGrenadeEnvironment = 12,
        TimbalandEnvironment = 13,
        FitBeatEnvironment = 14,
        LinkinParkEnvironment = 15,
        BTSEnvironment = 16,
        KaleidoscopeEnvironment = 17,
        InterscopeEnvironment = 18,
        SkrillexEnvironment = 19,
        #[doc(alias = "BillieEilish")]
        BillieEnvironment = 20,
        #[doc(alias = "Spooky")]
        HalloweenEnvironment = 21,
        #[doc(alias = "LadyGaga")]
        GagaEnvironment = 22,
        // V3:
        WeaveEnvironment = 23,
        #[doc(alias = "FallOutBoy")]
        PyroEnvironment = 24,
        EDMEnvironment = 25,
        TheSecondEnvironment = 26,
        LizzoEnvironment = 27,
        TheWeekndEnvironment = 28,
        RockMixtapeEnvironment = 29,
        Dragons2Environment = 30,
        Panic2Environment = 31,
        QueenEnvironment = 32,
        // Todo Add more.
    }
}

loose_enum! {
    #[derive(Default)]
    AllDirectionEnvironment {
        #[default]
        GlassDesertEnvironment = 0,
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DifficultySet {
    #[serde(rename = "_beatmapCharacteristicName")]
    pub characteristic: String,
    #[serde(rename = "_difficultyBeatmaps")]
    pub difficulties: Vec<DifficultyInfo>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub enum Characteristic {
    #[default]
    Standard,
    NoArrows,
    OneSaber,
    Rotate360,
    Rotate90,
    Legacy,
    //Custom types.
    Lawless,
    Lightshow,
    Unknown(String),
}

// Todo Replace if macro gets expanded to support more types.
impl<'de> serde::Deserialize<'de> for Characteristic {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let val = String::deserialize(deserializer)?;
        Ok(match val.as_str() {
            "Standard" => Characteristic::Standard,
            "NoArrows" => Characteristic::NoArrows,
            "OneSaber" => Characteristic::OneSaber,
            "360Degree" => Characteristic::Rotate360,
            "90Degree" => Characteristic::Rotate90,
            "Legacy" => Characteristic::Legacy,
            "Lawless" => Characteristic::Lawless,
            "Lightshow" => Characteristic::Lightshow,
            s => Characteristic::Unknown(s.to_string()),
        })
    }
}

impl serde::Serialize for Characteristic {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Characteristic::Standard => serializer.serialize_str("Standard"),
            Characteristic::NoArrows => serializer.serialize_str("NoArrows"),
            Characteristic::OneSaber => serializer.serialize_str("OneSaber"),
            Characteristic::Rotate360 => serializer.serialize_str("360Degree"),
            Characteristic::Rotate90 => serializer.serialize_str("90Degree"),
            Characteristic::Legacy => serializer.serialize_str("Legacy"),
            Characteristic::Lawless => serializer.serialize_str("Legacy"),
            Characteristic::Lightshow => serializer.serialize_str("Lawless"),
            Characteristic::Unknown(s) => serializer.serialize_str(s),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DifficultyInfo {
    #[serde(rename = "_difficulty")]
    pub name: String,
    #[serde(rename = "_difficultyRank")]
    pub rank: i32,
    #[serde(rename = "_beatmapFilename")]
    pub file: String,
    #[doc(alias = "node_jump_speed")]
    #[serde(rename = "_noteJumpMovementSpeed")]
    pub njs: i32,
    #[doc(alias = "node_jump_distance")]
    #[serde(rename = "_noteJumpStartBeatOffset")]
    pub njd: f32,
}
