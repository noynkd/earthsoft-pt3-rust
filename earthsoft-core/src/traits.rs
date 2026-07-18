use crate::{Channel, Error, PacketStream, TunerInfo};
use async_trait::async_trait;

// =============================================================================
// Client
// =============================================================================

#[async_trait]
pub trait Client: Sized + Send + Sync {
    async fn get_tuner_info(&self) -> Result<Vec<TunerInfo>, Error>;
    async fn get_tuner(&self, name: &str) -> Result<Box<dyn Tuner>, Error>;
}

// =============================================================================
// Tuner
// =============================================================================

#[async_trait]
pub trait Tuner: Send + Sync {
    async fn get_info(&self) -> Result<TunerInfo, Error>;
    async fn get_channels(&self) -> Result<Vec<Channel>, Error>;
    async fn get_current_channel(&self) -> Result<Channel, Error>;
    async fn set_channel(&self, channel: &Channel) -> Result<(), Error>;
    async fn start_stream(&self) -> Result<PacketStream, Error>;
    async fn stop_stream(&self) -> Result<(), Error>;
}
