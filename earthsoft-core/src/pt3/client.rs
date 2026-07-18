use crate::pt3::{Config, DeviceContext, TunerContext};
use crate::{Error, Tuner, TunerInfo};
use async_trait::async_trait;
use earthsoft_sdk::pt3;
use std::path::Path;
use std::sync::{Arc, Mutex};

// =============================================================================
// Client
// =============================================================================

#[derive(Debug, Default)]
pub struct Client {
    tuner_context: Vec<TunerContext>,
}

impl Client {
    pub async fn new(config_path: impl AsRef<Path>) -> Result<Self, Error> {
        let raw_toml = std::fs::read_to_string(config_path).map_err(|e| {
            eprintln!("std::fs::read_to_string() に失敗しました。: {:?}", e);
            Error::Failure
        })?;

        let config: Config = toml::from_str(&raw_toml).map_err(|e| {
            eprintln!("toml::from_str() に失敗しました。: {:?}", e);
            Error::Failure
        })?;

        let pt3 = pt3::Pt3::new().map_err(|e| {
            eprintln!("earthsoft_sdk::pt3::Pt3::new() に失敗しました。: {:?}", e);
            Error::Failure
        })?;

        let bus = pt3.create_bus().map_err(|e| {
            eprintln!(
                "earthsoft_sdk::pt3::Pt3::create_bus() に失敗しました。: {:?}",
                e
            );
            Error::Failure
        })?;

        let device_infos = bus.scan_device_info(9).map_err(|e| {
            eprintln!(
                "earthsoft_sdk::pt3::Pt3::scan_device_info() に失敗しました。: {:?}",
                e
            );
            Error::Failure
        })?;

        let device_context = device_infos
            .iter()
            .map(|device_info| {
                let context = DeviceContext::new(bus.clone(), device_info.clone());

                Ok(Arc::new(Mutex::new(context)))
            })
            .collect::<Result<Vec<_>, Error>>()?;

        let mut tuner_context = Vec::new();
        for (device_index, context) in device_context.iter().enumerate() {
            for isdb in pt3::Isdb::ALL {
                for tuner in 0..2 {
                    let context =
                        TunerContext::new(context.clone(), device_index, isdb, tuner, &config);

                    tuner_context.push(context);
                }
            }
        }

        Ok(Self { tuner_context })
    }
}

#[async_trait]
impl crate::Client for Client {
    async fn get_tuner_info(&self) -> Result<Vec<TunerInfo>, Error> {
        let tuner_info = self
            .tuner_context
            .iter()
            .map(|context| &context.tuner_info)
            .cloned()
            .collect();

        Ok(tuner_info)
    }

    async fn get_tuner(&self, name: &str) -> Result<Box<dyn Tuner>, Error> {
        let tuner_context = self
            .tuner_context
            .iter()
            .find(|context| context.tuner_info.tuner_name == name)
            .ok_or_else(|| {
                eprintln!("指定された名前のチューナーが見つかりません: {}", name);
                Error::Failure
            })?;

        let tuner = tuner_context.create_tuner()?;

        Ok(Box::new(tuner.clone()))
    }
}
