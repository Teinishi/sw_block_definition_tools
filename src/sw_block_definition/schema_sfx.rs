use super::{
    attribute_specifier::AttributeProperty, AttributeEnum, AttributeSpecifier, AttributeValue,
    Definition, Of32,
};
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

#[derive(
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    strum::Display,
    strum::VariantArray,
    Clone,
    Copy,
)]
#[strum(serialize_all = "snake_case", prefix = "sfx_")]
pub enum SfxDataAttribute {
    Name,
    RangeInner,
    RangeOuter,
    Priority,
    IsUnderwaterAffected,
}

impl AttributeEnum<SfxData> for SfxDataAttribute {
    fn get_value(&self, d: &SfxData) -> Option<AttributeValue> {
        match self {
            Self::Name => Some(d.sfx_name.clone()?.into()),
            Self::RangeInner => Some(d.sfx_range_inner?.into()),
            Self::RangeOuter => Some(d.sfx_range_outer?.into()),
            Self::Priority => Some(d.sfx_priority?.into()),
            Self::IsUnderwaterAffected => Some(d.sfx_is_underwater_affected?.into()),
        }
    }

    fn get_value_root(&self, d: &Definition) -> Vec<AttributeValue> {
        if let Some(datas) = d.sfx_datas.last() {
            datas
                .sfx_data
                .iter()
                .filter_map(|item| self.get_value(item))
                .collect()
        } else {
            vec![]
        }
    }
}

impl From<SfxDataAttribute> for AttributeSpecifier {
    fn from(value: SfxDataAttribute) -> Self {
        Self::SfxData(value)
    }
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

#[derive(
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    strum::Display,
    strum::VariantArray,
    Clone,
    Copy,
)]
#[strum(serialize_all = "snake_case", prefix = "sfx_")]
pub enum SfxLayerAttribute {
    FilenameStart,
    FilenameLoop,
    FilenameEnd,
    Gain,
    LoopStartTime,
    LoopBlendDuration,
    VolumeFadeSpeed,
    PitchFadeSpeed,
}

impl AttributeEnum<SfxLayer> for SfxLayerAttribute {
    fn get_value(&self, d: &SfxLayer) -> Option<AttributeValue> {
        match self {
            Self::FilenameStart => Some(d.sfx_filename_start.clone()?.into()),
            Self::FilenameLoop => Some(d.sfx_filename_loop.clone()?.into()),
            Self::FilenameEnd => Some(d.sfx_filename_end.clone()?.into()),
            Self::Gain => Some(d.sfx_gain?.into()),
            Self::LoopStartTime => Some(d.sfx_loop_start_time?.into()),
            Self::LoopBlendDuration => Some(d.sfx_loop_blend_duration?.into()),
            Self::VolumeFadeSpeed => Some(d.sfx_volume_fade_speed?.into()),
            Self::PitchFadeSpeed => Some(d.sfx_pitch_fade_speed?.into()),
        }
    }

    fn get_value_root(&self, d: &Definition) -> Vec<AttributeValue> {
        if let Some(datas) = d.sfx_datas.last() {
            datas
                .sfx_data
                .iter()
                .flat_map(|data| {
                    data.sfx_layers
                        .last()
                        .iter()
                        .flat_map(|layers| {
                            layers
                                .sfx_layer
                                .iter()
                                .filter_map(|layer| self.get_value(layer))
                        })
                        .collect::<Vec<_>>()
                })
                .collect()
        } else {
            vec![]
        }
    }

    fn property(&self) -> AttributeProperty {
        AttributeProperty {
            is_audio_file: matches!(
                self,
                Self::FilenameStart | Self::FilenameLoop | Self::FilenameEnd
            ),
        }
    }
}

impl From<SfxLayerAttribute> for AttributeSpecifier {
    fn from(value: SfxLayerAttribute) -> Self {
        Self::SfxLayer(value)
    }
}
