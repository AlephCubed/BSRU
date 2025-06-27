use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InfoV2 {
    #[serde(rename = "_version")]
    pub version: String,
    #[serde(rename = "_songName")]
    pub song_name: String,
    #[serde(rename = "_songSubName")]
    pub song_sub_name: String,
    #[serde(rename = "_songAuthorName")]
    pub song_author_name: String,
    #[serde(rename = "_levelAuthorName")]
    pub level_author_name: String,
    #[serde(rename = "_beatsPerMinute")]
    pub beats_per_minute: i32,
    #[serde(rename = "_songTimeOffset")]
    pub song_time_offset: i32,
    #[serde(rename = "_shuffle")]
    pub shuffle: i32,
    #[serde(rename = "_shufflePeriod")]
    pub shuffle_period: i32,
    #[serde(rename = "_previewStartTime")]
    pub preview_start_time: f32,
    #[serde(rename = "_previewDuration")]
    pub preview_duration: i32,
    #[serde(rename = "_songFilename")]
    pub song_filename: String,
    #[serde(rename = "_coverImageFilename")]
    pub cover_image_filename: String,
    #[serde(rename = "_environmentName")]
    pub environment_name: String,
    #[serde(rename = "_allDirectionsEnvironmentName")]
    pub all_directions_environment_name: String,
    #[serde(rename = "_environmentNames")]
    pub environment_names: Vec<Value>,
    #[serde(rename = "_colorSchemes")]
    pub color_schemes: Vec<Value>,
    #[serde(rename = "_difficultyBeatmapSets")]
    pub difficulty_beatmap_sets: Vec<DifficultyBeatmapSet>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DifficultyBeatmapSet {
    #[serde(rename = "_beatmapCharacteristicName")]
    pub beatmap_characteristic_name: String,
    #[serde(rename = "_difficultyBeatmaps")]
    pub difficulty_beatmaps: Vec<DifficultyBeatmap>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DifficultyBeatmap {
    #[serde(rename = "_difficulty")]
    pub difficulty: String,
    #[serde(rename = "_difficultyRank")]
    pub difficulty_rank: i32,
    #[serde(rename = "_beatmapFilename")]
    pub beatmap_filename: String,
    #[serde(rename = "_noteJumpMovementSpeed")]
    pub note_jump_movement_speed: i32,
    #[serde(rename = "_noteJumpStartBeatOffset")]
    pub note_jump_start_beat_offset: f32,
}
