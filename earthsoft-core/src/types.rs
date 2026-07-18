use std::fmt::{Display, Formatter};
use std::pin::Pin;
use tokio_stream::Stream;

pub type Packet = Vec<u8>;
pub type PacketStream = Pin<Box<dyn Stream<Item = Result<Packet, Error>> + Send>>;

// =============================================================================
// Isdb
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Isdb {
    #[default]
    Satellite,
    Terrestrial,
}

// =============================================================================
// Channel
// =============================================================================

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Channel {
    pub space: u32,
    pub index: u32,
    pub name: String,
}

// =============================================================================
// Error
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Error {
    #[default]
    Success,
    // TODO: 実際のエラーステータスを設定する
    Failure,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Success => write!(f, "成功しました。"),
            // TODO: 実際のエラーステータスを設定する
            Error::Failure => write!(f, "失敗しました。"),
        }
    }
}

impl std::error::Error for Error {}

// =============================================================================
// TunerInfo
// =============================================================================

#[derive(Debug, Clone, PartialEq, Default)]
pub struct TunerInfo {
    pub tuner_name: String, // 読み込んだすべてのチューナーを識別できる一意の名前を返す
    pub device_name: String, // デバイス名
    pub isdb: Isdb,         // ISDB
    pub tuner_index: u32, // 上記デバイス/ISDB毎の連番、もしくはデバイス毎の連番 (デバイスによって変わる)
    pub signal_level: f32, // チューナーが取得できる信号強度
}
