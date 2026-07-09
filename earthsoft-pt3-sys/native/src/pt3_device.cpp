#include "internal.h"

#ifdef  __cplusplus
extern "C" {
#endif

int32_t DeletePt3Device(Pt3Device *device) {
    if (device == nullptr || device->impl == nullptr) {
        // return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
        return static_cast<int32_t>(Earthsoft::Pt3::Status::Ok);
    }

    std::int32_t status = device->impl->Delete();

    return static_cast<int32_t>(status);
}

int32_t OpenPt3Device(Pt3Device *device) {
    if (device == nullptr || device->impl == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }

    std::int32_t status = device->impl->Open();

    return static_cast<int32_t>(status);
}

int32_t ClosePt3Device(Pt3Device *device) {
    if (device == nullptr || device->impl == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }

    std::int32_t status = device->impl->Close();

    return static_cast<int32_t>(status);
}

int32_t GetPt3ConstantInfo(Pt3Device *device, Pt3ConstantInfo *constantInfo) {
    if (device == nullptr || device->impl == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }
    if (constantInfo == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }
    std::int32_t status = device->impl->GetConstantInfo(
        reinterpret_cast<Earthsoft::Pt3::ConstantInfo *>(constantInfo));

    return static_cast<int32_t>(status);
}

int32_t SetPt3LnbPower(Pt3Device *device, uint32_t power) {
    if (device == nullptr || device->impl == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }

    std::int32_t status = device->impl->SetLnbPower(
        static_cast<Earthsoft::Pt3::LnbPower>(power));

    return static_cast<int32_t>(status);
}

int32_t GetPt3LnbPower(Pt3Device *device, uint32_t *power) {
    if (device == nullptr || device->impl == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }
    if (power == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }

    std::int32_t status = device->impl->GetLnbPower(
        reinterpret_cast<Earthsoft::Pt3::LnbPower *>(power));

    return static_cast<int32_t>(status);    
}

int32_t SetPt3LnbPowerWhenClose(Pt3Device *device, uint32_t power) {
    if (device == nullptr || device->impl == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }

    std::int32_t status = device->impl->SetLnbPowerWhenClose(
        static_cast<Earthsoft::Pt3::LnbPower>(power));

    return static_cast<int32_t>(status);
}

int32_t GetPt3LnbPowerWhenClose(Pt3Device *device, uint32_t *power) {
    if (device == nullptr || device->impl == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }
    if (power == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }

    std::int32_t status = device->impl->GetLnbPowerWhenClose(
        reinterpret_cast<Earthsoft::Pt3::LnbPower *>(power));

    return static_cast<int32_t>(status);    
}

int32_t InitPt3Tuner(Pt3Device *device) {
    if (device == nullptr || device->impl == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }

    std::int32_t status = device->impl->InitTuner();

    return static_cast<int32_t>(status);
}

int32_t SetPt3TunerSleep(Pt3Device *device, uint32_t isdb, uint32_t tuner, uint8_t sleep) {
    if (device == nullptr || device->impl == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }

    std::int32_t status = device->impl->SetTunerSleep(
        static_cast<Earthsoft::Pt3::Isdb>(isdb),
        static_cast<std::int32_t>(tuner),
        static_cast<bool>(sleep));

    return static_cast<int32_t>(status);
}

int32_t GetPt3TunerSleep(Pt3Device *device, uint32_t isdb, uint32_t tuner, uint8_t *sleep) {
    if (device == nullptr || device->impl == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }
    if (sleep == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }

    bool rawSleep = false;

    std::int32_t status = device->impl->GetTunerSleep(
        static_cast<Earthsoft::Pt3::Isdb>(isdb),
        static_cast<std::uint32_t>(tuner),
        reinterpret_cast<bool *>(sleep));

    return static_cast<int32_t>(status);
}

int32_t SetPt3Frequency(Pt3Device *device, uint32_t isdb, uint32_t tuner, uint32_t channel, int32_t offset) {
    if (device == nullptr || device->impl == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }

    std::int32_t status = device->impl->SetFrequency(
        static_cast<Earthsoft::Pt3::Isdb>(isdb),
        static_cast<std::uint32_t>(tuner),
        static_cast<std::uint32_t>(channel),
        static_cast<std::int32_t>(offset));

    return static_cast<int32_t>(status);
}

int32_t GetPt3Frequency(Pt3Device *device, uint32_t isdb, uint32_t tuner, uint32_t *channel, int32_t *offset) {
    if (device == nullptr || device->impl == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }
    if (channel == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }
    // NOTE: offset は nullptr を許容する
    // if (offset == nullptr) {
    //     return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    // }

    std::int32_t status = device->impl->GetFrequency(
        static_cast<Earthsoft::Pt3::Isdb>(isdb),
        static_cast<std::uint32_t>(tuner),
        reinterpret_cast<std::uint32_t *>(channel),
        reinterpret_cast<std::int32_t *>(offset));

    return static_cast<int32_t>(status);
}

int32_t GetPt3FrequencyOffset(Pt3Device *device, uint32_t isdb, uint32_t tuner, int32_t *clock, int32_t *offset) {
    if (device == nullptr || device->impl == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }
    if (clock == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }
    if (offset == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }

    std::int32_t status = device->impl->GetFrequencyOffset(
        static_cast<Earthsoft::Pt3::Isdb>(isdb),
        static_cast<std::uint32_t>(tuner),
        reinterpret_cast<std::int32_t *>(clock),
        reinterpret_cast<std::int32_t *>(offset));

    return static_cast<int32_t>(status);
}

int32_t GetPt3CnAgc(Pt3Device *device, uint32_t isdb, uint32_t tuner, uint32_t *cn100, uint32_t *currentAgc, uint32_t *maxAgc) {
    if (device == nullptr || device->impl == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }
    if (cn100 == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }
    if (currentAgc == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }
    if (maxAgc == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }

    std::int32_t status = device->impl->GetCnAgc(
        static_cast<Earthsoft::Pt3::Isdb>(isdb),
        static_cast<std::uint32_t>(tuner),
        reinterpret_cast<std::uint32_t *>(cn100),
        reinterpret_cast<std::uint32_t *>(currentAgc),
        reinterpret_cast<std::uint32_t *>(maxAgc));

    return static_cast<int32_t>(status);
}

int32_t GetPt3RfLevel(Pt3Device *device, uint32_t tuner, float *level) {
    if (device == nullptr || device->impl == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }
    if (level == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }

    std::int32_t status = device->impl->GetRfLevel(
        static_cast<std::uint32_t>(tuner),
        level);

    return static_cast<int32_t>(status);
}

int32_t SetPt3SatelliteId(Pt3Device *device, uint32_t tuner, uint32_t id) {
    if (device == nullptr || device->impl == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }

    std::int32_t status = device->impl->SetSatelliteId(
        static_cast<std::uint32_t>(tuner),
        static_cast<std::uint32_t>(id));

    return static_cast<int32_t>(status);
}

int32_t GetPt3SatelliteId(Pt3Device *device, uint32_t tuner, uint32_t *id) {
    if (device == nullptr || device->impl == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }
    if (id == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }

    std::int32_t status = device->impl->GetSatelliteId(
        static_cast<std::uint32_t>(tuner),
        reinterpret_cast<std::uint32_t *>(id));

    return static_cast<int32_t>(status);
}

int32_t SetPt3InnerErrorRateLayer(Pt3Device *device, uint32_t isdb, uint32_t tuner, uint32_t layer) {
    if (device == nullptr || device->impl == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }

    std::int32_t status = device->impl->SetInnerErrorRateLayer(
        static_cast<Earthsoft::Pt3::Isdb>(isdb),
        static_cast<std::uint32_t>(tuner),
        static_cast<std::uint32_t>(layer));

    return static_cast<int32_t>(status);
}

int32_t GetPt3InnerErrorRate(Pt3Device *device, uint32_t isdb, uint32_t tuner, Pt3ErrorRate *errorRate) {
    if (device == nullptr || device->impl == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }
    if (errorRate == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }

    Earthsoft::Pt3::ErrorRate rawErrrorRate {};

    std::int32_t status = device->impl->GetInnerErrorRate(
        static_cast<Earthsoft::Pt3::Isdb>(isdb),
        static_cast<std::uint32_t>(tuner),
        reinterpret_cast<Earthsoft::Pt3::ErrorRate *>(errorRate));

    return static_cast<int32_t>(status);
}

int32_t GetPt3CorrectedErrorRate(Pt3Device *device, uint32_t isdb, uint32_t tuner, uint32_t layer, Pt3ErrorRate *errorRate) {
    if (device == nullptr || device->impl == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }
    if (errorRate == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }

    Earthsoft::Pt3::ErrorRate rawErrrorRate {};

    std::int32_t status = device->impl->GetCorrectedErrorRate(
        static_cast<Earthsoft::Pt3::Isdb>(isdb),
        static_cast<std::uint32_t>(tuner),
        static_cast<std::uint32_t>(layer),
        reinterpret_cast<Earthsoft::Pt3::ErrorRate *>(errorRate));

    return static_cast<int32_t>(status);
}

int32_t ResetPt3CorrectedErrorCount(Pt3Device *device, uint32_t isdb, uint32_t tuner) {
    if (device == nullptr || device->impl == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }

    std::int32_t status = device->impl->ResetCorrectedErrorCount(
        static_cast<Earthsoft::Pt3::Isdb>(isdb),
        static_cast<std::uint32_t>(tuner));

    return static_cast<int32_t>(status);
}

int32_t GetPt3ErrorCount(Pt3Device *device, uint32_t isdb, uint32_t tuner, uint32_t *count) {
    if (device == nullptr || device->impl == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }
    if (count == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }

    std::int32_t status = device->impl->GetErrorCount(
        static_cast<Earthsoft::Pt3::Isdb>(isdb),
        static_cast<std::uint32_t>(tuner),
        reinterpret_cast<std::uint32_t *>(count));

    return static_cast<int32_t>(status);
}

int32_t GetPt3SatelliteTmcc(Pt3Device *device, uint32_t tuner, Pt3SatelliteTmcc *tmcc) {
    if (device == nullptr || device->impl == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }
    if (tmcc == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }

    std::int32_t status = device->impl->GetSatelliteTmcc(
        static_cast<std::uint32_t>(tuner),
        reinterpret_cast<Earthsoft::Pt3::Satellite::Tmcc *>(tmcc));

    return static_cast<int32_t>(status);
}

int32_t GetPt3SatelliteLayer(Pt3Device *device, uint32_t tuner, Pt3SatelliteLayer *layer) {
    if (device == nullptr || device->impl == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }
    if (layer == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }

    std::int32_t status = device->impl->GetSatelliteLayer(
        static_cast<std::uint32_t>(tuner),
        reinterpret_cast<Earthsoft::Pt3::Satellite::Layer *>(layer));

    return static_cast<int32_t>(status);
}

int32_t GetPt3TerrestrialTmcc(Pt3Device *device, uint32_t tuner, Pt3TerrestrialTmcc *tmcc) {
    if (device == nullptr || device->impl == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }
    if (tmcc == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }

    std::int32_t status = device->impl->GetTerrestrialTmcc(
        static_cast<std::uint32_t>(tuner),
        reinterpret_cast<Earthsoft::Pt3::Terrestrial::Tmcc *>(tmcc));

    return static_cast<int32_t>(status);
}

int32_t SetPt3AmpPower(Pt3Device *device, uint8_t power) {
    if (device == nullptr || device->impl == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }

    std::int32_t status = device->impl->SetAmpPower(
        static_cast<std::uint8_t>(power));

    return static_cast<int32_t>(status);
}

int32_t SetPt3LayerEnable(Pt3Device *device, uint32_t isdb, uint32_t tuner, uint32_t layerMask) {
    if (device == nullptr || device->impl == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }

    std::int32_t status = device->impl->SetLayerEnable(
        static_cast<Earthsoft::Pt3::Isdb>(isdb),
        static_cast<std::uint32_t>(tuner),
        static_cast<std::uint32_t>(layerMask));

    return static_cast<int32_t>(status);
}

int32_t GetPt3LayerEnable(Pt3Device *device, uint32_t isdb, uint32_t tuner, uint32_t *layerMask) {
    if (device == nullptr || device->impl == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }
    if (layerMask == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }

    std::int32_t status = device->impl->GetLayerEnable(
        static_cast<Earthsoft::Pt3::Isdb>(isdb),
        static_cast<std::uint32_t>(tuner),
        reinterpret_cast<std::uint32_t *>(layerMask));

    return static_cast<int32_t>(status);
}

int32_t SetPt3TsPinsMode(Pt3Device *device, uint32_t isdb, uint32_t tuner, const Pt3TsPinsMode *mode) {
    if (device == nullptr || device->impl == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }
    if (mode == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }

    std::int32_t status = device->impl->SetTsPinsMode(
        static_cast<Earthsoft::Pt3::Isdb>(isdb),
        static_cast<std::uint32_t>(tuner),
        reinterpret_cast<const Earthsoft::Pt3::TsPinsMode *>(mode));

    return static_cast<int32_t>(status);
}

int32_t GetPt3TsPinsLevel(Pt3Device *device, uint32_t isdb, uint32_t tuner, Pt3TsPinsLevel *level) {
    if (device == nullptr || device->impl == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }
    if (level == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }

    std::int32_t status = device->impl->GetTsPinsLevel(
        static_cast<Earthsoft::Pt3::Isdb>(isdb),
        static_cast<std::uint32_t>(tuner),
        reinterpret_cast<Earthsoft::Pt3::TsPinsLevel *>(level));

    return static_cast<int32_t>(status);
}

int32_t GetPt3TsSyncByte(Pt3Device *device, uint32_t isdb, uint32_t tuner, uint8_t *syncByte) {
    if (device == nullptr || device->impl == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }
    if (syncByte == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }

    std::int32_t status = device->impl->GetTsSyncByte(
        static_cast<Earthsoft::Pt3::Isdb>(isdb),
        static_cast<std::uint32_t>(tuner),
        reinterpret_cast<std::uint8_t *>(syncByte));

    return static_cast<int32_t>(status);
}

int32_t SetPt3RamPinsMode(Pt3Device *device, uint32_t mode) {
    if (device == nullptr || device->impl == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }

    std::int32_t status = device->impl->SetRamPinsMode(
        static_cast<Earthsoft::Pt3::RamPinsMode>(mode)
    );

    return static_cast<int32_t>(status);
}

int32_t UnlockPt3Buffer(Pt3Device *device, void *handle) {
    if (device == nullptr || device->impl == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }
    if (handle == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }

    std::int32_t status = device->impl->UnlockBuffer(handle);

    return static_cast<int32_t>(status);
}

int32_t GetPt3BufferInfo(Pt3Device *device, void *handle, const Pt3BufferInfo **infoTable, uint32_t *infoCount) {
    if (device == nullptr || device->impl == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }
    if (handle == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }
    if (infoTable == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }
    if (infoCount == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }

    std::int32_t status = device->impl->GetBufferInfo(
        handle,
        reinterpret_cast<const Earthsoft::Pt3::BufferInfo **>(infoTable),
        reinterpret_cast<std::uint32_t *>(infoCount));

    return static_cast<int32_t>(status);
}

int32_t SetPt3TransferPageDescriptorAddress(Pt3Device *device, uint32_t isdb, uint32_t tuner, uint64_t pageDescriptorAddress) {
    if (device == nullptr || device->impl == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }

    std::int32_t status = device->impl->SetTransferPageDescriptorAddress(
        static_cast<Earthsoft::Pt3::Isdb>(isdb),
        static_cast<std::uint32_t>(tuner),
        static_cast<std::uint64_t>(pageDescriptorAddress));

    return static_cast<int32_t>(status);
}

int32_t SetPt3TransferEnabled(Pt3Device *device, uint32_t isdb, uint32_t tuner, uint8_t enabled) {
    if (device == nullptr || device->impl == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }

    std::int32_t status = device->impl->SetTransferEnabled(
        static_cast<Earthsoft::Pt3::Isdb>(isdb),
        static_cast<std::uint32_t>(tuner),
        static_cast<bool>(enabled));

    return static_cast<int32_t>(status);
}

int32_t GetPt3TransferEnabled(Pt3Device *device, uint32_t isdb, uint32_t tuner, uint8_t *enabled) {
    if (device == nullptr || device->impl == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }
    if (enabled == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }

    std::int32_t status = device->impl->GetTransferEnabled(
        static_cast<Earthsoft::Pt3::Isdb>(isdb),
        static_cast<std::uint32_t>(tuner),
        reinterpret_cast<bool *>(enabled));

    return static_cast<int32_t>(status);
}

int32_t SetPt3TransferTestMode(Pt3Device *device, uint32_t isdb, uint32_t tuner, uint8_t testMode, uint16_t initial, uint8_t notOp) {
    if (device == nullptr || device->impl == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }

    std::int32_t status = device->impl->SetTransferTestMode(
        static_cast<Earthsoft::Pt3::Isdb>(isdb),
        static_cast<std::uint32_t>(tuner),
        static_cast<bool>(testMode),
        static_cast<std::uint16_t>(initial),
        static_cast<bool>(notOp));

    return static_cast<int32_t>(status);
}

int32_t GetPt3TransferInfo(Pt3Device *device, uint32_t isdb, uint32_t tuner, Pt3TransferInfo *transferInfo) {
    if (device == nullptr || device->impl == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }
    if (transferInfo == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }

    std::int32_t status = device->impl->GetTransferInfo(
        static_cast<Earthsoft::Pt3::Isdb>(isdb),
        static_cast<std::uint32_t>(tuner),
        reinterpret_cast<Earthsoft::Pt3::TransferInfo *>(transferInfo));

    return static_cast<int32_t>(status);
}

int32_t LockPt3Buffer(Pt3Device *device, void *ptr, uint32_t size, uint32_t direction, void **handle) {
    if (device == nullptr || device->impl == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }
    if (ptr == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }
    if (size == 0) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }
    if (handle == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }

    std::int32_t status = device->impl->LockBuffer(
        ptr,
        static_cast<std::uint32_t>(size),
        static_cast<Earthsoft::Pt3::TransferDirection>(direction),
        handle);

    return static_cast<int32_t>(status);
}

int32_t SyncPt3BufferCpu(Pt3Device *device, void *handle) {
    if (device == nullptr || device->impl == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }
    if (handle == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }

    std::int32_t status = device->impl->SyncBufferCpu(handle);

    return static_cast<int32_t>(status);
}

int32_t SyncPt3BufferIo(Pt3Device *device, void *handle) {
    if (device == nullptr || device->impl == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }
    if (handle == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }

    std::int32_t status = device->impl->SyncBufferIo(handle);

    return static_cast<int32_t>(status);
}

#ifdef  __cplusplus
}
#endif
