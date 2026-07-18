use crate::pt3::{ChannelSetting, Config, Tuner};
use crate::{Error, Isdb, TunerInfo};
use earthsoft_sdk::pt3;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

// =============================================================================
// DeviceContext
// =============================================================================

#[derive(Debug)]
pub(crate) struct DeviceContext {
    pub(crate) bus: Arc<pt3::Bus>,
    pub(crate) device_info: pt3::DeviceInfo,
    pub(crate) device: Option<Arc<pt3::Device>>,
    pub(crate) opened_tuners: [[AtomicBool; 2]; 2],
}

impl DeviceContext {
    pub fn new(bus: Arc<pt3::Bus>, device_info: pt3::DeviceInfo) -> Self {
        Self {
            bus,
            device_info,
            device: None,
            opened_tuners: Default::default(),
        }
    }

    pub fn activate(&mut self, isdb: usize, tuner: usize) -> Result<Arc<pt3::Device>, Error> {
        match &self.device {
            Some(device) => {
                self.opened_tuners[isdb][tuner].store(true, Ordering::SeqCst);
                Ok(device.clone())
            }
            None => {
                let device = self.bus.create_device(&self.device_info).map_err(|e| {
                    eprintln!(
                        "earthsoft_pt3::pt3::::create_device() に失敗しました。: {:?}",
                        e
                    );
                    Error::Failure
                })?;

                device.open().map_err(|e| {
                    eprintln!("earthsoft_pt3::pt3::::open() に失敗しました。: {:?}", e);
                    Error::Failure
                })?;

                device.init_tuner().map_err(|e| {
                    eprintln!(
                        "earthsoft_pt3::pt3::::init_tuner() に失敗しました。: {:?}",
                        e
                    );
                    Error::Failure
                })?;

                // 全チューナーを一度省電力モードにする
                for isdb in pt3::Isdb::ALL {
                    for tuner in 0..2 {
                        device.set_tuner_sleep(isdb, tuner, true).map_err(|e| {
                            eprintln!(
                                "earthsoft_pt3::pt3::::set_tuner_sleep() に失敗しました。: {:?}",
                                e
                            );
                            Error::Failure
                        })?;
                    }
                }

                self.device = Some(device.clone());

                self.opened_tuners[isdb][tuner].store(true, Ordering::SeqCst);
                Ok(device)
            }
        }
    }

    pub fn deactivate(&mut self, isdb: usize, tuner: usize) {
        self.opened_tuners[isdb][tuner].store(false, Ordering::SeqCst);

        let any_active = self
            .opened_tuners
            .iter()
            .flat_map(|inner| inner.iter())
            .any(|atomic| atomic.load(Ordering::SeqCst));

        if !any_active {
            self.device = None;
        }
    }
}

// =============================================================================
// TunerContext
// =============================================================================

#[derive(Debug)]
pub(crate) struct TunerContext {
    pub(crate) device_context: Arc<Mutex<DeviceContext>>,
    pub(crate) tuner_info: TunerInfo,
    pub(crate) isdb: pt3::Isdb,
    pub(crate) tuner: u32,
    pub(crate) channels: Vec<ChannelSetting>,
}

impl TunerContext {
    pub fn new(
        device_context: Arc<Mutex<DeviceContext>>,
        device_index: usize,
        isdb: pt3::Isdb,
        tuner: u32,
        config: &Config,
    ) -> Self {
        let isdb_name = match isdb {
            pt3::Isdb::Satellite => "ISDB-S",
            pt3::Isdb::Terrestrial => "ISDB-T",
        };

        let bus_index = device_context.lock().unwrap().device_info.bus;

        let path = std::format!(
            "PT3/Bus:{}/Device:{}/{}/{}",
            bus_index,
            device_index,
            isdb_name,
            tuner,
        );

        let name = config
            .tuners
            .iter()
            .find(|tuner_setting| tuner_setting.path == path)
            .map_or_else(|| path, |tuner_setting| tuner_setting.name.clone());

        let tuner_info = TunerInfo {
            tuner_name: name,
            device_name: "PT3".into(),
            isdb: isdb.into(),
            tuner_index: tuner,
            signal_level: 0.0,
        };

        // 利用可能なチャンネルをチャンネル設定から取得する
        // いまのところ3波混合チューナーとかは考えない
        let spaces = match isdb {
            pt3::Isdb::Satellite => 0..=1,
            pt3::Isdb::Terrestrial => 2..=4,
        };
        let channels = config
            .channels
            .iter()
            .filter(|channel_setting| spaces.contains(&channel_setting.space))
            .cloned()
            .collect::<Vec<_>>();

        Self {
            device_context,
            tuner_info,
            isdb,
            tuner,
            channels,
        }
    }

    pub fn create_tuner(&self) -> Result<Tuner, Error> {
        let mut device_context = self.device_context.lock().unwrap();

        // デバイスを取得、もしくは作成する
        let device = device_context.activate(self.isdb as usize, self.tuner as usize)?;

        // 省電力モードから復帰する
        let sleep = device.get_tuner_sleep(self.isdb, self.tuner).map_err(|e| {
            eprintln!(
                "earthsoft_pt3::pt3::Device::get_tuner_sleep() に失敗しました。: {:?}",
                e
            );
            Error::Failure
        })?;
        if sleep {
            device
                .set_tuner_sleep(self.isdb, self.tuner, false)
                .map_err(|e| {
                    eprintln!(
                        "earthsoft_pt3::pt3::Device::set_tuner_sleep() に失敗しました。: {:?}",
                        e
                    );
                    Error::Failure
                })?;
        }

        Ok(Tuner::new(
            device,
            self.isdb,
            self.tuner,
            self.device_context.clone(),
            self.tuner_info.clone(),
            self.channels.clone(),
        ))
    }
}

impl From<pt3::Isdb> for Isdb {
    fn from(value: pt3::Isdb) -> Self {
        match value {
            pt3::Isdb::Satellite => Isdb::Satellite,
            pt3::Isdb::Terrestrial => Isdb::Terrestrial,
        }
    }
}
