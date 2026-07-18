use crate::Channel;
use serde::Deserialize;

// =============================================================================
// Config
// =============================================================================

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Config {
    pub tuners: Vec<TunerSetting>,
    pub spaces: Vec<String>,
    pub channels: Vec<ChannelSetting>,
}

// =============================================================================
// TunerSetting
// =============================================================================

#[derive(Debug, Clone, Default, Deserialize)]
pub struct TunerSetting {
    pub name: String,
    pub path: String,
}

// =============================================================================
// ChannelSetting
// =============================================================================

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ChannelSetting {
    pub name: String,
    pub space: u32,
    pub index: u32,
    pub channel: u32,
    pub tsid: u32,
}

impl From<ChannelSetting> for Channel {
    fn from(value: ChannelSetting) -> Self {
        (&value).into()
    }
}

impl From<&ChannelSetting> for Channel {
    fn from(value: &ChannelSetting) -> Self {
        Self {
            space: value.space,
            index: value.index,
            name: value.name.clone(),
        }
    }
}
