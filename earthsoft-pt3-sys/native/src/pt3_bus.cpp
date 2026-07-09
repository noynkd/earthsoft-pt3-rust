#include "internal.h"

#ifdef  __cplusplus
extern "C" {
#endif

int32_t DeletePt3Bus(Pt3Bus *bus) {
    if (bus == nullptr || bus->impl == nullptr) {
        // 既に削除済みのため Ok とする
        // return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
        return static_cast<int32_t>(Earthsoft::Pt3::Status::Ok);
    }

    std::int32_t status = bus->impl->Delete();
    if (status == 0) {
        if (bus->impl != nullptr) {
            bus->impl = nullptr;
        }

        delete bus;
    }

    return static_cast<int32_t>(status);
}

int32_t GetPt3Version(Pt3Bus *bus, uint32_t *version) {
    if (bus == nullptr || bus->impl == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }
    if (version == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }

    std::int32_t status = bus->impl->GetVersion(version);

    return static_cast<int32_t>(status);
}

int32_t ScanPt3DeviceInfo(Pt3Bus *bus, Pt3DeviceInfo *deviceInfo, uint32_t *deviceInfoCount) {
    if (bus == nullptr || bus->impl == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }
    if (deviceInfo == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }
    if (deviceInfoCount == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }
    if (*deviceInfoCount == 0) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }

    std::int32_t status = bus->impl->Scan(
        reinterpret_cast<Earthsoft::Pt3::DeviceInfo *>(deviceInfo),
        reinterpret_cast<std::uint32_t *>(deviceInfoCount));

    return static_cast<int32_t>(status);
}

int32_t CreatePt3Device(Pt3Bus *bus, const Pt3DeviceInfo *deviceInfo, Pt3Device **device) {
    if (bus == nullptr || bus->impl == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }
    if (deviceInfo == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }
    if (device == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }

    Earthsoft::Pt3::Device *rawDevice = nullptr;

    std::int32_t status = bus->impl->NewDevice(
        reinterpret_cast<const Earthsoft::Pt3::DeviceInfo *>(deviceInfo),
        &rawDevice,
        nullptr
    );

    if (status != static_cast<int32_t>(Earthsoft::Pt3::Status::Ok)) {
        if (rawDevice != nullptr) {
            rawDevice->Delete();
            rawDevice = nullptr;
        }

        return static_cast<int32_t>(status);
    }

    if (rawDevice == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InternalError);
    }

    Pt3Device *newDevice = new (std::nothrow) Pt3Device {
        .impl = rawDevice,
    };
    if (newDevice == nullptr) {
        rawDevice->Delete();
        rawDevice = nullptr;
        return static_cast<int32_t>(Earthsoft::Pt3::Status::OutOfMemory);
    }
    *device = newDevice;

    return static_cast<int32_t>(Earthsoft::Pt3::Status::Ok);
}

#ifdef  __cplusplus
}
#endif
