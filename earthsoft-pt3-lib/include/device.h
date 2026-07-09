#pragma once

#include <array>
#include <cstddef>
#include <cstdint>

namespace Earthsoft::Pt3 {
    class Bus;

    enum class Isdb : std::uint32_t {
        Satellite,
        Terrestrial,
    };

    constexpr std::size_t IsdbCount = 2;

    enum class LnbPower : std::uint32_t {
        PowerOff,
        Power15V,
        Power11V,
    };

    enum class RamPinsMode : std::uint32_t {
        Normal,
        Low,
        High,
    };

    enum class TransferDirection : std::uint32_t {
        Write = 1 << 0,
        Read  = 1 << 1,
        WriteRead = Write | Read,
    };

    enum class TsPinMode : std::uint32_t {
        Normal,
        Low,
        High,
    };

    struct BufferInfo {
        std::uint64_t Address;
        std::uint32_t Size;
    };

    struct ConstantInfo {
        std::uint8_t PtVersion;
        std::uint8_t RegisterMapVersion;
        std::uint8_t FpgaVersion;
        bool IsTsSupported;
        std::uint32_t PageDescriptorSizeBits;
    };

    struct ErrorRate {
        std::uint32_t Numerator;
        std::uint32_t Denominator;
    };

    struct TransferInfo {
        bool Busy;
        std::uint32_t Status;
        bool InternalFifoAOverflow;
        bool InternalFifoAUnderflow;
        bool ExternalFifoOverflow;
        std::uint32_t ExternalFifoMaxUsedBytes;
        bool InternalFifoBOverflow;
        bool InternalFifoBUnderflow;
    };

    struct TsPinsLevel {
        bool Clock;
        bool Data;
        bool Byte;
        bool Valid;
    };

    struct TsPinsMode {
        TsPinMode ClockData;
        TsPinMode Byte;
        TsPinMode Valid;
    };

    namespace Satellite {
        enum class LayerIndex : std::uint32_t {
            Low  = 0,
            High = 1,
        };

        constexpr std::size_t LayerCount = 2;

        enum class LayerMask : std::uint32_t {
            None = 0,
            Low  = 1 << static_cast<std::uint32_t>(LayerIndex::Low),
            High = 1 << static_cast<std::uint32_t>(LayerIndex::High),
        };

        struct Layer {
            std::array<std::uint32_t, LayerCount> Mode;
            std::array<std::uint32_t, LayerCount> Count;
        };

        struct Tmcc {
            std::uint32_t Indicator;
            std::array<std::uint32_t, 4> Mode;
            std::array<std::uint32_t, 4> Slot;
            std::array<std::uint32_t, 8> Id;
            std::uint32_t Emergency;
            std::uint32_t UpLink;
            std::uint32_t ExtFlag;
            std::array<std::uint32_t, 2> ExtData;
        };
    }

    namespace Terrestrial {
        enum class LayerIndex : std::uint32_t {
            A = 0,
            B = 1,
            C = 2,
        };

        constexpr std::size_t LayerCount = 3;

        enum class LayerMask : std::uint32_t {
            None = 0,
            A = 1 << static_cast<std::uint32_t>(LayerIndex::A),
            B = 1 << static_cast<std::uint32_t>(LayerIndex::B),
            C = 1 << static_cast<std::uint32_t>(LayerIndex::C),
        };

        struct Tmcc {
            std::uint32_t System;
            std::uint32_t Indicator;
            std::uint32_t Emergency;
            std::uint32_t Partial;
            std::array<std::uint32_t, LayerCount> Mode;
            std::array<std::uint32_t, LayerCount> Rate;
            std::array<std::uint32_t, LayerCount> Interleave;
            std::array<std::uint32_t, LayerCount> Segment;
            std::uint32_t Phase;
            std::uint32_t Reserved;
        };
    }

    class Device {
    public:
        virtual std::int32_t Delete() = 0;
        virtual std::int32_t Open() = 0;
        virtual std::int32_t Close() = 0;
        virtual std::int32_t GetConstantInfo(ConstantInfo *constantInfo) const = 0;
        virtual std::int32_t SetLnbPower(LnbPower power) = 0;
        virtual std::int32_t GetLnbPower(LnbPower *power) const = 0;
        virtual std::int32_t SetLnbPowerWhenClose(LnbPower power) = 0;
        virtual std::int32_t GetLnbPowerWhenClose(LnbPower *power) const = 0;
        virtual std::int32_t InitTuner() = 0;
        virtual std::int32_t SetTunerSleep(Isdb isdb, std::uint32_t tuner, bool sleep) = 0;
        virtual std::int32_t GetTunerSleep(Isdb isdb, std::uint32_t tuner, bool *sleep) const = 0;
        virtual std::int32_t SetFrequency(Isdb isdb, std::uint32_t tuner, std::uint32_t channel, std::int32_t offset = 0) = 0;
        virtual std::int32_t GetFrequency(Isdb isdb, std::uint32_t tuner, std::uint32_t *channel, std::int32_t *offset = nullptr) const = 0;
        virtual std::int32_t GetFrequencyOffset(Isdb isdb, std::uint32_t tuner, std::int32_t *clock, std::int32_t *offset) = 0;
        virtual std::int32_t GetCnAgc(Isdb isdb, std::uint32_t tuner, std::uint32_t *cn100, std::uint32_t *currentAgc, std::uint32_t *maxAgc) = 0;
        virtual std::int32_t GetRfLevel(std::uint32_t tuner, float *level) = 0;
        virtual std::int32_t SetSatelliteId(std::uint32_t tuner, std::uint32_t id) = 0;
        virtual std::int32_t GetSatelliteId(std::uint32_t tuner, std::uint32_t *id) = 0;
        virtual std::int32_t SetInnerErrorRateLayer(Isdb isdb, std::uint32_t tuner, std::uint32_t layer) = 0;
        virtual std::int32_t GetInnerErrorRate(Isdb isdb, std::uint32_t tuner, ErrorRate *errorRate) = 0;
        virtual std::int32_t GetCorrectedErrorRate(Isdb isdb, std::uint32_t tuner, std::uint32_t layer, ErrorRate *errorRate) = 0;
        virtual std::int32_t ResetCorrectedErrorCount(Isdb isdb, std::uint32_t tuner) = 0;
        virtual std::int32_t GetErrorCount(Isdb isdb, std::uint32_t tuner, std::uint32_t *count) = 0;
        virtual std::int32_t GetSatelliteTmcc(std::uint32_t tuner, Satellite::Tmcc *tmcc) = 0;
        virtual std::int32_t GetSatelliteLayer(std::uint32_t tuner, Satellite::Layer *layer) = 0;
        virtual std::int32_t GetTerrestrialTmcc(std::uint32_t tuner, Terrestrial::Tmcc *tmcc) = 0;
        virtual std::int32_t SetAmpPower(bool power) = 0;
        virtual std::int32_t SetLayerEnable(Isdb isdb, std::uint32_t tuner, std::uint32_t layerMask) = 0;
        virtual std::int32_t GetLayerEnable(Isdb isdb, std::uint32_t tuner, std::uint32_t *layerMask) const = 0;
        virtual std::int32_t SetTsPinsMode(Isdb isdb, std::uint32_t tuner, const TsPinsMode *mode) = 0;
        virtual std::int32_t GetTsPinsLevel(Isdb isdb, std::uint32_t tuner, TsPinsLevel *level) = 0;
        virtual std::int32_t GetTsSyncByte(Isdb isdb, std::uint32_t tuner, std::uint8_t *syncByte) = 0;
        virtual std::int32_t SetRamPinsMode(RamPinsMode mode) = 0;
        virtual std::int32_t __LockBuffer__Obsolete(void *ptr, std::uint32_t size, void **handle) = 0;
        virtual std::int32_t UnlockBuffer(void *handle) = 0;
        virtual std::int32_t GetBufferInfo(void *handle, const BufferInfo **infoTable, std::uint32_t *infoCount) = 0;
        virtual std::int32_t SetTransferPageDescriptorAddress(Isdb isdb, std::uint32_t tuner, std::uint64_t pageDescriptorAddress) = 0;
        virtual std::int32_t SetTransferEnabled(Isdb isdb, std::uint32_t tuner, bool enabled) = 0;
        virtual std::int32_t GetTransferEnabled(Isdb isdb, std::uint32_t tuner, bool *enabled) const = 0;
        virtual std::int32_t SetTransferTestMode(Isdb isdb, std::uint32_t tuner, bool testMode = false, std::uint16_t initial = 0, bool notOp = false) = 0;
        virtual std::int32_t GetTransferInfo(Isdb isdb, std::uint32_t tuner, TransferInfo *transferInfo) = 0;
        virtual std::int32_t LockBuffer(void *ptr, std::uint32_t size, TransferDirection direction, void **handle) = 0;
        virtual std::int32_t SyncBufferCpu(void *handle) = 0;
        virtual std::int32_t SyncBufferIo(void *handle) = 0;

    protected:
        virtual ~Device() {}
    };
}
