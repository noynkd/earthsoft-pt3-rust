#pragma once

#include <cstdint>

namespace Earthsoft::Pt3 {
    enum class Status : std::int32_t {
        Ok                              = 0x000,

        Unspecified                     = 0x100,
        NotImplemented                  = 0x101,
        InvalidParameter                = 0x102,
        OutOfMemory                     = 0x103,
        InternalError                   = 0x104,

        WdApiLoadError                  = 0x200,
        RemainingDevices                = 0x201,

        PciBusError                     = 0x300,
        InvalidConfigRevision           = 0x301,
        InvalidFpgaVersion              = 0x302,
        InvalidPciBaseAddress           = 0x303,
        FlashMemoryFailed               = 0x304,
        DcmLockTimeout                  = 0x305,
        DcmShiftTimeout                 = 0x306,
        PowerResetFailed                = 0x307,
        I2cError                        = 0x308,
        TunerInSleep                    = 0x309,
        PllOutOfRange                   = 0x30A,
        PllLockTimeout                  = 0x30B,
        VirtualAllocFailed              = 0x30C,
        InvalidDmaAddress               = 0x30D,
        BufferAlreadyAllocated          = 0x30E,
        DeviceAlreadyOpen               = 0x30F,
        DeviceNotOpen                   = 0x310,
        BufferInUse                     = 0x311,
        BufferNotAllocated              = 0x312,
        DeviceNotClosed                 = 0x313,

        WdDriverNameInvalid             = 0x400,
        WdOpenFailed                    = 0x401,
        WdCloseFailed                   = 0x402,
        WdVersionInvalid                = 0x403,
        WdLicenseInvalid                = 0x404,
        WdPciScanCardsFailed            = 0x405,
        WdPciConfigDumpFailed           = 0x406,
        WdPciGetCardInfoFailed          = 0x407,
        WdPciGetCardInfoBusFailed       = 0x408,
        WdPciGetCardInfoMemoryFailed    = 0x409,
        WdCardRegisterFailed            = 0x40A,
        WdCardUnregisterFailed          = 0x40B,
        WdCardCleanupSetupFailed        = 0x40C,
        WdDmaLockFailed                 = 0x40D,
        WdDmaUnlockFailed               = 0x40E,
        WdDmaSyncCpuFailed              = 0x40F,
        WdDmaSyncIoFailed               = 0x410,

        RomError                        = 0x500,
        RomTimeout                      = 0x501,
    };
}
