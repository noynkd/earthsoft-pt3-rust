#pragma once

#include "menu/device_menu.h"
#include "utility/utility.h"

namespace Earthsoft::Example::Command {
    class BusCommand {
    public:
        BusCommand(Pt3::NewBusFunction busFactory) {
            valid_ = Initialize(busFactory);
        };

        ~BusCommand() = default;

        BusCommand(const BusCommand&) = delete;
        BusCommand& operator=(const BusCommand&) = delete;

        BusCommand(BusCommand&&) noexcept = default;
        BusCommand& operator=(BusCommand&&) noexcept = default;

        const bool IsValid() const noexcept {
            return valid_;
        }

        std::vector<Pt3::DeviceInfo>& GetDeviceInfo() {
            return deviceInfo_;
        }

        void ShowDeviceMenu(std::uint32_t index) {
            Menu::DeviceMenu deviceMenu;
            deviceMenu.Run(bus_.get(), &deviceInfo_[index]);
        }

    private:
        bool Initialize(Pt3::NewBusFunction busFactory) {
            if (busFactory == nullptr) {
                Utility::PrintError("Earthsoft::Pt3::NewBusFunction が nullptr です。");
                return false;
            }

            Pt3::Bus *busPtr = nullptr;
            std::int32_t status = busFactory(&busPtr);
            if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                Utility::PrintError("Earthsoft::Pt3::NewBusFunction() に失敗しました。", status);
                return false;
            }
            if (busPtr == nullptr) {
                Utility::PrintError("Earthsoft::Pt3::Bus が nullptr です。");
                return false;
            }

            bus_.reset(busPtr);

            std::uint32_t version = 0;
            status = bus_->GetVersion(&version);
            if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                Utility::PrintError("Earthsoft::Pt3::Bus::GetVersion() に失敗しました。", status);
                return false;
            }
            std::println("PT3 SDK バージョン: {:#010x}", version);
            if ((version >> 8) != 2) {
                Utility::PrintError("インストールされている SDK のバージョンには対応していません。");
                return false;
            }

            std::array<Pt3::DeviceInfo, 9> deviceInfo {};
            std::uint32_t deviceInfoCount = static_cast<std::uint32_t>(deviceInfo.size());
            status = bus_->Scan(deviceInfo.data(), &deviceInfoCount);
            if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                Utility::PrintError("Earthsoft::Pt3::Bus::Scan() に失敗しました。", status);
                return false;
            }

            for (std::uint32_t index = 0; index < deviceInfoCount; ++index) {
                deviceInfo_.push_back(deviceInfo[index]);
            }

            return true;
        }

        struct BusDeleter {
            void operator()(Pt3::Bus *bus) const noexcept {
                if (bus != nullptr) {
                    std::int32_t status = bus->Delete();
                    if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                        Utility::PrintError("Earthsoft::Pt3::Bus::Delete() で失敗しました。", status);
                        return;
                    }
                }
            }
        };

        std::unique_ptr<Pt3::Bus, BusDeleter> bus_ = nullptr;
        std::vector<Pt3::DeviceInfo> deviceInfo_ {};
        bool valid_ = false;
    };
}
