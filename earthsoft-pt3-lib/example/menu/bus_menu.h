#pragma once

#include "command/bus_command.h"
#include "utility/utility.h"

namespace Earthsoft::Example::Menu {
    class BusMenu {
    public:
        BusMenu() = default;

        ~BusMenu() = default;

        BusMenu(const BusMenu&) = delete;
        BusMenu& operator=(const BusMenu&) = delete;

        BusMenu(BusMenu&&) noexcept = default;
        BusMenu& operator=(BusMenu&&) noexcept = default;

        void Run(Pt3::NewBusFunction busFactory) {
            Command::BusCommand busCommand = Command::BusCommand(busFactory);
            if (!busCommand.IsValid()) {
                return;
            }

            std::vector<Pt3::DeviceInfo> deviceInfo = busCommand.GetDeviceInfo();
            std::uint32_t deviceInfoCount = static_cast<std::uint32_t>(deviceInfo.size());

            bool done = false;
            while (!done) {
                std::println("[デバイス選択メニュー]");
                std::println("   Bus:バス番号 / Dev:デバイス番号 / Fun:ファンクション番号 / PTn:品番");
                std::println("--+---+---+---+---");
                std::println("#  Bus Dev Fun PTn");
                std::println("--+---+---+---+---");

                std::println("0: (終了)");
                for (std::uint32_t index = 0; index < deviceInfoCount; ++index) {
                    const Pt3::DeviceInfo& info = deviceInfo[index];
                    std::println(
                        "{}: {:3} {:3} {:3} {:3}", 
                        index + 1,
                        info.Bus,
                        info.Slot,
                        info.Function,
                        info.PtVersion);
                }

                std::print(">");
                
                const std::uint32_t number = Utility::GetNumber(deviceInfoCount);
                if (number == 0) {
                    done = true;
                } else {
                    busCommand.ShowDeviceMenu(number -1);
                }
            }
        }
    };
}
