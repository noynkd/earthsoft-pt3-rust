// =============================================================================
// Error
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Error {
    #[default]
    Ok,                             // 0x000

    Unspecified,                    // 0x100
    NotImplemented,                 // 0x101
    InvalidParameter,               // 0x102
    OutOfMemory,                    // 0x103
    InternalError,                  // 0x104

    WdApiLoadError,                 // 0x200
    RemainingDevices,               // 0x201

    PciBusError,                    // 0x300
    InvalidConfigRevision,          // 0x301
    InvalidFpgaVersion,             // 0x302
    InvalidPciBaseAddress,          // 0x303
    FlashMemoryFailed,              // 0x304
    DcmLockTimeout,                 // 0x305
    DcmShiftTimeout,                // 0x306
    PowerResetFailed,               // 0x307
    I2cError,                       // 0x308
    TunerInSleep,                   // 0x309
    PllOutOfRange,                  // 0x30A
    PllLockTimeout,                 // 0x30B
    VirtualAllocFailed,             // 0x30C
    InvalidDmaAddress,              // 0x30D
    BufferAlreadyAllocated,         // 0x30E
    DeviceAlreadyOpen,              // 0x30F
    DeviceNotOpen,                  // 0x310
    BufferInUse,                    // 0x311
    BufferNotAllocated,             // 0x312
    DeviceNotClosed,                // 0x313

    WdDriverNameInvalid,            // 0x400
    WdOpenFailed,                   // 0x401
    WdCloseFailed,                  // 0x402
    WdVersionInvalid,               // 0x403
    WdLicenseInvalid,               // 0x404
    WdPciScanCardsFailed,           // 0x405
    WdPciConfigDumpFailed,          // 0x406
    WdPciGetCardInfoFailed,         // 0x407
    WdPciGetCardInfoBusFailed,      // 0x408
    WdPciGetCardInfoMemoryFailed,   // 0x409
    WdCardRegisterFailed,           // 0x40A
    WdCardUnregisterFailed,         // 0x40B
    WdCardCleanupSetupFailed,       // 0x40C
    WdDmaLockFailed,                // 0x40D
    WdDmaUnlockFailed,              // 0x40E
    WdDmaSyncCpuFailed,             // 0x40F
    WdDmaSyncIoFailed,              // 0x410

    RomError,                       // 0x500
    RomTimeout,                     // 0x501

    Unknown(i32),
}

impl Error {
    pub fn check_result(&self) -> Result<(), Error> {
        match self {
            Error::Ok => Ok(()),
            other => Err(*other),
        }
    }
}

impl From<i32> for Error {
    fn from(value: i32) -> Self {
        match value {
            0x000 => Error::Ok,
            0x100 => Error::Unspecified,
            0x101 => Error::NotImplemented,
            0x102 => Error::InvalidParameter,
            0x103 => Error::OutOfMemory,
            0x104 => Error::InternalError,
            0x200 => Error::WdApiLoadError,
            0x201 => Error::RemainingDevices,
            0x300 => Error::PciBusError,
            0x301 => Error::InvalidConfigRevision,
            0x302 => Error::InvalidFpgaVersion,
            0x303 => Error::InvalidPciBaseAddress,
            0x304 => Error::FlashMemoryFailed,
            0x305 => Error::DcmLockTimeout,
            0x306 => Error::DcmShiftTimeout,
            0x307 => Error::PowerResetFailed,
            0x308 => Error::I2cError,
            0x309 => Error::TunerInSleep,
            0x30A => Error::PllOutOfRange,
            0x30B => Error::PllLockTimeout,
            0x30C => Error::VirtualAllocFailed,
            0x30D => Error::InvalidDmaAddress,
            0x30E => Error::BufferAlreadyAllocated,
            0x30F => Error::DeviceAlreadyOpen,
            0x310 => Error::DeviceNotOpen,
            0x311 => Error::BufferInUse,
            0x312 => Error::BufferNotAllocated,
            0x313 => Error::DeviceNotClosed,
            0x400 => Error::WdDriverNameInvalid,
            0x401 => Error::WdOpenFailed,
            0x402 => Error::WdCloseFailed,
            0x403 => Error::WdVersionInvalid,
            0x404 => Error::WdLicenseInvalid,
            0x405 => Error::WdPciScanCardsFailed,
            0x406 => Error::WdPciConfigDumpFailed,
            0x407 => Error::WdPciGetCardInfoFailed,
            0x408 => Error::WdPciGetCardInfoBusFailed,
            0x409 => Error::WdPciGetCardInfoMemoryFailed,
            0x40A => Error::WdCardRegisterFailed,
            0x40B => Error::WdCardUnregisterFailed,
            0x40C => Error::WdCardCleanupSetupFailed,
            0x40D => Error::WdDmaLockFailed,
            0x40E => Error::WdDmaUnlockFailed,
            0x40F => Error::WdDmaSyncCpuFailed,
            0x410 => Error::WdDmaSyncIoFailed,
            0x500 => Error::RomError,
            0x501 => Error::RomTimeout,
            status => Error::Unknown(status),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Ok                           => write!(f, "正常終了しました。"),
            Error::Unspecified                  => write!(f, "不特定のエラーです。"),
            Error::NotImplemented               => write!(f, "処理が実装されていません。"),
            Error::InvalidParameter             => write!(f, "不正なパラメーターです。"),
            Error::OutOfMemory                  => write!(f, "メモリが足りません。"),
            Error::InternalError                => write!(f, "内部処理エラーです。"),
            Error::WdApiLoadError               => write!(f, "wdapi1100.dll のロードに失敗しました。"),
            Error::RemainingDevices             => write!(f, "デバイスが残っています。バスを削除できません。"),
            Error::PciBusError                  => write!(f, "PCIバスでエラーが発生しました。"),
            Error::InvalidConfigRevision        => write!(f, "リビジョンIDが不正です。"),
            Error::InvalidFpgaVersion           => write!(f, "FPGAバージョンが不正です。"),
            Error::InvalidPciBaseAddress        => write!(f, "PCIベースアドレスの割り当てに失敗しました。"),
            Error::FlashMemoryFailed            => write!(f, "フラッシュメモリの操作に失敗しました。"),
            Error::DcmLockTimeout               => write!(f, "DCMのロックでタイムアウトしました。"),
            Error::DcmShiftTimeout              => write!(f, "DCMのフェーズシフトでタイムアウトしました。"),
            Error::PowerResetFailed             => write!(f, "電源のリセットに失敗しました。"),
            Error::I2cError                     => write!(f, "I2C通信エラーが発生しました。"),
            Error::TunerInSleep                 => write!(f, "チューナーが省電力状態のため処理できません。"),
            Error::PllOutOfRange                => write!(f, "PLL周波数が範囲外です。"),
            Error::PllLockTimeout               => write!(f, "PLLのロックでタイムアウトしました。"),
            Error::VirtualAllocFailed           => write!(f, "仮想メモリの割り当てに失敗しました。"),
            Error::InvalidDmaAddress            => write!(f, "DMAアドレスが不正です。"),
            Error::BufferAlreadyAllocated       => write!(f, "DMAメモリ領域はすでに割り当てられています。"),
            Error::DeviceAlreadyOpen            => write!(f, "デバイスはすでに開始しています。"),
            Error::DeviceNotOpen                => write!(f, "デバイスが開始していないため処理できません。"),
            Error::BufferInUse                  => write!(f, "DMAメモリ領域は使用中です。"),
            Error::BufferNotAllocated           => write!(f, "DMAメモリ領域が割り当てられていません。"),
            Error::DeviceNotClosed              => write!(f, "デバイスが終了していません。"),
            Error::WdDriverNameInvalid          => write!(f, "WD_DriverName() の取得値が不正です。"),
            Error::WdOpenFailed                 => write!(f, "WD_Open() に失敗しました。"),
            Error::WdCloseFailed                => write!(f, "WD_Close() に失敗しました。"),
            Error::WdVersionInvalid             => write!(f, "WD_Open() に失敗しました。"),
            Error::WdLicenseInvalid             => write!(f, "WD_Open() に失敗しました。"),
            Error::WdPciScanCardsFailed         => write!(f, "WD_PciScanCard() に失敗しました。"),
            Error::WdPciConfigDumpFailed        => write!(f, "WD_ConfigDump() に失敗しました。"),
            Error::WdPciGetCardInfoFailed       => write!(f, "WD_PciGetCardInfo() に失敗しました。"),
            Error::WdPciGetCardInfoBusFailed    => write!(f, "WD_PciGetCardInfo() に失敗しました。PCIバス情報が不正です。"),
            Error::WdPciGetCardInfoMemoryFailed => write!(f, "WD_PciGetCardInfo() に失敗しました。メモリ情報が不正です。"),
            Error::WdCardRegisterFailed         => write!(f, "WD_CardRegister() に失敗しました。"),
            Error::WdCardUnregisterFailed       => write!(f, "WD_CardUnregister() に失敗しました。"),
            Error::WdCardCleanupSetupFailed     => write!(f, "WD_CardCleanupSetup() に失敗しました。"),
            Error::WdDmaLockFailed              => write!(f, "WD_DmaLock() に失敗しました。"),
            Error::WdDmaUnlockFailed            => write!(f, "WD_DmaUnlock() に失敗しました。"),
            Error::WdDmaSyncCpuFailed           => write!(f, "WD_DmaSyncCpu() に失敗しました。"),
            Error::WdDmaSyncIoFailed            => write!(f, "WD_DmaSyncIo() に失敗しました。"),
            Error::RomError                     => write!(f, "ボードのEEPROM/ROM関連でエラーが発生しました。"),
            Error::RomTimeout                   => write!(f, "ROMへの操作でタイムアウトしました。"),
            Error::Unknown(status)              => write!(f, "不明なステータス値を返しました: 0x{:03X}。", status),
        }
    }
}

impl std::error::Error for Error {}
