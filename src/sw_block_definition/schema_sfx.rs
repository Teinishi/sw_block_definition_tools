use super::Of32;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub struct SfxDatas {
    #[serde(default)]
    pub sfx_data: Vec<SfxData>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub struct SfxData {
    #[serde(rename = "@sfx_name")]
    pub sfx_name: Option<String>,
    #[serde(rename = "@sfx_range_inner")]
    pub sfx_range_inner: Option<Of32>,
    #[serde(rename = "@sfx_range_outer")]
    pub sfx_range_outer: Option<Of32>,
    #[serde(rename = "@sfx_priority")]
    pub sfx_priority: Option<Of32>,
    #[serde(rename = "@sfx_is_underwater_affected")]
    pub sfx_is_underwater_affected: Option<bool>,

    pub sfx_layers: Vec<SfxLayers>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub struct SfxLayers {
    #[serde(default)]
    pub sfx_layer: Vec<SfxLayer>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub struct SfxLayer {
    #[serde(rename = "@sfx_filename_start")]
    pub sfx_filename_start: Option<String>,
    #[serde(rename = "@sfx_filename_loop")]
    pub sfx_filename_loop: Option<String>,
    #[serde(rename = "@sfx_filename_end")]
    pub sfx_filename_end: Option<String>,
    #[serde(rename = "@sfx_gain")]
    pub sfx_gain: Option<Of32>,
    #[serde(rename = "@sfx_loop_start_time")]
    pub sfx_loop_start_time: Option<Of32>,
    #[serde(rename = "@sfx_loop_blend_duration")]
    pub sfx_loop_blend_duration: Option<Of32>,
    #[serde(rename = "@sfx_volume_fade_speed")]
    pub sfx_volume_fade_speed: Option<Of32>,
    #[serde(rename = "@sfx_pitch_fade_speed")]
    pub sfx_pitch_fade_speed: Option<Of32>,
}
