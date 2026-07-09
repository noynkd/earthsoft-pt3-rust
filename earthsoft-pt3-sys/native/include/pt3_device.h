#pragma once

#include <stdint.h>

#ifdef  __cplusplus
extern "C" {
#endif

enum Pt3Isdb {
    Pt3Satellite,
    Pt3Terrestrial,
};

enum {
    Pt3IsdbCount = 2,
};

enum Pt3LnbPower {
    Pt3LnbPowerOff,
    Pt3LnbPower15V,
    Pt3LnbPower11V,
};

enum Pt3RamPinsMode {
    Pt3RamPinsModeNormal,
    Pt3RamPinsModeLow,
    Pt3RamPinsModeHigh,
};

enum Pt3SatelliteLayerIndex {
    Pt3SatelliteLayerIndexLow  = 0,
    Pt3SatelliteLayerIndexHigh = 1,
};

enum {
    Pt3SatelliteLayerCount = 2,
};

enum Pt3SatelliteLayerMask {
    Pt3SatelliteLayerMaskNone = 0,
    Pt3SatelliteLayerMaskLow  = 1 << (uint32_t)Pt3SatelliteLayerIndexLow,
    Pt3SatelliteLayerMaskHigh = 1 << (uint32_t)Pt3SatelliteLayerIndexHigh,
};

enum Pt3TerrestrialLayerIndex {
    Pt3TerrestrialLayerIndexA = 0,
    Pt3TerrestrialLayerIndexB = 1,
    Pt3TerrestrialLayerIndexC = 2,
};

enum {
    Pt3TerrestrialLayerCount = 3,
};

enum Pt3TerrestrialLayerMask {
    Pt3TerrestrialLayerMaskNone = 0,
    Pt3TerrestrialLayerMaskA = 1 << (uint32_t)Pt3TerrestrialLayerIndexA,
    Pt3TerrestrialLayerMaskB = 1 << (uint32_t)Pt3TerrestrialLayerIndexB,
    Pt3TerrestrialLayerMaskC = 1 << (uint32_t)Pt3TerrestrialLayerIndexC,
};

enum Pt3TransferDirection {
    Pt3TransferDirectionWrite = 1 << 0,
    Pt3TransferDirectionRead  = 1 << 1,
    Pt3TransferDirectionWriteRead = Pt3TransferDirectionWrite | Pt3TransferDirectionRead,
};

enum Pt3TsPinMode {
    Pt3TsPinModeNormal,
    Pt3TsPinModeLow,
    Pt3TsPinModeHigh,
};

typedef struct Pt3BufferInfo {
    uint64_t Address;
    uint32_t Size;
} Pt3BufferInfo;

typedef struct Pt3ConstantInfo {
    uint8_t PtVersion;
    uint8_t RegisterMapVersion;
    uint8_t FpgaVersion;
    uint8_t IsTsSupported;
    uint32_t PageDescriptorSizeBits;
} Pt3ConstantInfo;

typedef struct Pt3ErrorRate {
    uint32_t Numerator;
    uint32_t Denominator;
} Pt3ErrorRate;

typedef struct Pt3SatelliteTmcc {
    uint32_t Indicator;
    uint32_t Mode[4];
    uint32_t Slot[4];
    uint32_t Id[8];
    uint32_t Emergency;
    uint32_t UpLink;
    uint32_t ExtFlag;
    uint32_t ExtData[2];
} Pt3SatelliteTmcc;

typedef struct Pt3SatelliteLayer {
    uint32_t Mode[Pt3SatelliteLayerCount];
    uint32_t Count[Pt3SatelliteLayerCount];
} Pt3SatelliteLayer;

typedef struct Pt3TerrestrialTmcc {
    uint32_t System;
    uint32_t Indicator;
    uint32_t Emergency;
    uint32_t Partial;
    uint32_t Mode[Pt3TerrestrialLayerCount];
    uint32_t Rate[Pt3TerrestrialLayerCount];
    uint32_t Interleave[Pt3TerrestrialLayerCount];
    uint32_t Segment[Pt3TerrestrialLayerCount];
    uint32_t Phase;
    uint32_t Reserved;
} Pt3TerrestrialTmcc;

typedef struct Pt3TransferInfo {
    uint8_t Busy;
    uint32_t Status;
    uint8_t InternalFifoAOverflow;
    uint8_t InternalFifoAUnderflow;
    uint8_t ExternalFifoOverflow;
    uint32_t ExternalFifoMaxUsedBytes;
    uint8_t InternalFifoBOverflow;
    uint8_t InternalFifoBUnderflow;
} Pt3TransferInfo;

typedef struct Pt3TsPinsLevel {
    uint8_t Clock;
    uint8_t Data;
    uint8_t Byte;
    uint8_t Valid;
} Pt3TsPinsLevel;

typedef struct Pt3TsPinsMode {
    uint32_t ClockData;
    uint32_t Byte;
    uint32_t Valid;
} Pt3TsPinsMode;

typedef struct Pt3Device Pt3Device;

int32_t DeletePt3Device(Pt3Device *device);
int32_t OpenPt3Device(Pt3Device *device);
int32_t ClosePt3Device(Pt3Device *device);
int32_t GetPt3ConstantInfo(Pt3Device *device, Pt3ConstantInfo *constantInfo);
int32_t SetPt3LnbPower(Pt3Device *device, /* enum Pt3LnbPower */ uint32_t power);
int32_t GetPt3LnbPower(Pt3Device *device, /* enum Pt3LnbPower */ uint32_t *power);
int32_t SetPt3LnbPowerWhenClose(Pt3Device *device, /* enum Pt3LnbPower */ uint32_t power);
int32_t GetPt3LnbPowerWhenClose(Pt3Device *device, /* enum Pt3LnbPower */ uint32_t *power);
int32_t InitPt3Tuner(Pt3Device *device);
int32_t SetPt3TunerSleep(Pt3Device *device, /* enum Pt3Isdb */ uint32_t isdb, uint32_t tuner, uint8_t sleep);
int32_t GetPt3TunerSleep(Pt3Device *device, /* enum Pt3Isdb */ uint32_t isdb, uint32_t tuner, uint8_t *sleep);
int32_t SetPt3Frequency(Pt3Device *device, /* enum Pt3Isdb */ uint32_t isdb, uint32_t tuner, uint32_t channel, int32_t offset);
int32_t GetPt3Frequency(Pt3Device *device, /* enum Pt3Isdb */ uint32_t isdb, uint32_t tuner, uint32_t *channel, int32_t *offset);
int32_t GetPt3FrequencyOffset(Pt3Device *device, /* enum Isdb */ uint32_t isdb, uint32_t tuner, int32_t *clock, int32_t *offset);
int32_t GetPt3CnAgc(Pt3Device *device, /* enum Pt3Isdb */ uint32_t isdb, uint32_t tuner, uint32_t *cn100, uint32_t *currentAgc, uint32_t *maxAgc);
int32_t GetPt3RfLevel(Pt3Device *device, uint32_t tuner, float *level);
int32_t SetPt3SatelliteId(Pt3Device *device, uint32_t tuner, uint32_t id);
int32_t GetPt3SatelliteId(Pt3Device *device, uint32_t tuner, uint32_t *id);
int32_t SetPt3InnerErrorRateLayer(Pt3Device *device, /* enum Pt3Isdb */ uint32_t isdb, uint32_t tuner, /* enum Pt3(Satellite|Terrestrial)LayerIndex */ uint32_t layer);
int32_t GetPt3InnerErrorRate(Pt3Device *device, /* enum Pt3Isdb */ uint32_t isdb, uint32_t tuner, Pt3ErrorRate *errorRate);
int32_t GetPt3CorrectedErrorRate(Pt3Device *device, /* enum Pt3Isdb */ uint32_t isdb, uint32_t tuner, /* enum Pt3(Satellite|Terrestrial)LayerIndex */ uint32_t layer, Pt3ErrorRate *errorRate);
int32_t ResetPt3CorrectedErrorCount(Pt3Device *device, /* enum Pt3Isdb */ uint32_t isdb, uint32_t tuner);
int32_t GetPt3ErrorCount(Pt3Device *device, /* enum Pt3Isdb */ uint32_t isdb, uint32_t tuner, uint32_t *count);
int32_t GetPt3SatelliteTmcc(Pt3Device *device, uint32_t tuner, Pt3SatelliteTmcc *tmcc);
int32_t GetPt3SatelliteLayer(Pt3Device *device, uint32_t tuner, Pt3SatelliteLayer *layer);
int32_t GetPt3TerrestrialTmcc(Pt3Device *device, uint32_t tuner, Pt3TerrestrialTmcc *tmcc);
int32_t SetPt3AmpPower(Pt3Device *device, uint8_t power);
int32_t SetPt3LayerEnable(Pt3Device *device, /* enum Pt3Isdb */ uint32_t isdb, uint32_t tuner, /* enum Pt3(Satellite|Terrestrial)LayerMask */ uint32_t layerMask);
int32_t GetPt3LayerEnable(Pt3Device *device, /* enum Pt3Isdb */ uint32_t isdb, uint32_t tuner, /* enum Pt3(Satellite|Terrestrial)LayerMask */ uint32_t *layerMask);
int32_t SetPt3TsPinsMode(Pt3Device *device, /* enum Pt3Isdb */ uint32_t isdb, uint32_t tuner, const Pt3TsPinsMode *mode);
int32_t GetPt3TsPinsLevel(Pt3Device *device, /* enum Pt3Isdb */ uint32_t isdb, uint32_t tuner, Pt3TsPinsLevel *level);
int32_t GetPt3TsSyncByte(Pt3Device *device, /* enum Pt3Isdb */ uint32_t isdb, uint32_t tuner, uint8_t *syncByte);
int32_t SetPt3RamPinsMode(Pt3Device *device, /* enum Pt3RamPinsMode */ uint32_t mode);
// int32_t __LockPt3Buffer__Obsolete(Pt3Device *device, void *ptr, uint32_t size, void **handle);
int32_t UnlockPt3Buffer(Pt3Device *device, void *handle);
int32_t GetPt3BufferInfo(Pt3Device *device, void *handle, const Pt3BufferInfo **infoTable, uint32_t *infoCount);
int32_t SetPt3TransferPageDescriptorAddress(Pt3Device *device, /* enum Pt3Isdb */ uint32_t isdb, uint32_t tunerIndex, uint64_t pageDescriptorAddress);
int32_t SetPt3TransferEnabled(Pt3Device *device, /* enum Pt3Isdb */ uint32_t isdb, uint32_t tuner, uint8_t enabled);
int32_t GetPt3TransferEnabled(Pt3Device *device, /* enum Pt3Isdb */ uint32_t isdb, uint32_t tuner, uint8_t *enabled);
int32_t SetPt3TransferTestMode(Pt3Device *device, /* enum Pt3Isdb */ uint32_t isdb, uint32_t tuner, uint8_t testMode, uint16_t initial, uint8_t notOp);
int32_t GetPt3TransferInfo(Pt3Device *device, /* enum Isdb */ uint32_t isdb, uint32_t tuner, Pt3TransferInfo *transferInfo);
int32_t LockPt3Buffer(Pt3Device *device, void *ptr, uint32_t size, /* enum Pt3TransferDirection */ uint32_t direction, void **handle);
int32_t SyncPt3BufferCpu(Pt3Device *device, void *handle);
int32_t SyncPt3BufferIo(Pt3Device *device, void *handle);

#ifdef  __cplusplus
}
#endif
