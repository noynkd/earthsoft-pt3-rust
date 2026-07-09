#pragma once

#include <cstdint>

namespace Earthsoft::Pt3 {
    class Device;

    struct DeviceInfo {
        std::uint32_t Bus;
        std::uint32_t Slot;
        std::uint32_t Function;
        std::uint32_t PtVersion;
    };

    class Bus {
    public:
        virtual std::int32_t Delete() = 0;
        virtual std::int32_t GetVersion(std::uint32_t *version) const = 0;
        virtual std::int32_t Scan(DeviceInfo *deviceInfo, std::uint32_t *deviceInfoCount) = 0;
        virtual std::int32_t NewDevice(const DeviceInfo *deviceInfo, Device **device, void **device_ = nullptr) = 0;

    protected:
        virtual ~Bus() {}
    };
}
