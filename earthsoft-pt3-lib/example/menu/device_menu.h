#pragma once

#include "command/device_command.h"
#include "utility/utility.h"

namespace Earthsoft::Example::Menu {
    class DeviceMenu {
    public:
        DeviceMenu() = default;

        ~DeviceMenu() = default;

        DeviceMenu(const DeviceMenu&) = delete;
        DeviceMenu& operator=(const DeviceMenu&) = delete;

        DeviceMenu(DeviceMenu&&) noexcept = default;
        DeviceMenu& operator=(DeviceMenu&&) noexcept = default;

        void Run(Pt3::Bus *bus, const Pt3::DeviceInfo *deviceInfo) {
            Command::DeviceCommand command = Command::DeviceCommand(bus, deviceInfo);
            if (!command.IsValid()) {
                return;
            }

            ShowDeviceMenuPage1(command);
        }
    
    private:
        void ShowDeviceMenuPage1(Command::DeviceCommand &command) {
            bool done = false;
            while (!done) {
                std::println("[デバイスメニュー (1 of 2)]");
                std::println("0: (戻る)");
                std::println("1: -> 2ページ目");
                std::println("2: LNB 電源設定");
                std::println("3: チャンネルスキャン");
                std::println("4: チャンネル設定");
                std::println("5: TS-ID 設定");
                std::println("6: ステータス表示");
                std::println("7: キャプチャ{}", busy_ ? "停止" : "開始");
                std::println("8: 機能検査");
                std::println("9: 地上アンプ電源");

                std::print(">");

                const std::uint32_t number = Utility::GetNumber(9);
                switch (number) {
                    case 0:
                        done = true;
                        break;
                    case 1:
                        ShowDeviceMenuPage2(command);
                        break;
                    case 2:
                        command.SetLnbPower();
                        break;
                    case 3:
                        command.ScanChannel();
                        break;
                    case 4:
                        command.SetChannel();
                        break;
                    case 5:
                        command.SetTsId();
                        break;
                    case 6:
                        command.ShowErrorRateCount();
                        break;
                    case 7:
                        if (busy_) {
                            command.StopCapture();
                            busy_ = false;
                        } else {
                            command.StartCapture();
                            busy_ = true;
                        }
                        break;
                    case 8:
                        command.CheckHardware();
                        break;
                    case 9:
                        command.SetAmpPower();
                        break;
                    default:
                        break;
                }
            }
        }

        void ShowDeviceMenuPage2(Command::DeviceCommand &command) {
            std::println("[デバイスメニュー (2 of 2)]");
            std::println("0: (戻る)");
            std::println("1: チューナー省電力制御設定");
            std::println("2: チャンネル設定×4");
            std::println("3: エラッタ(FPGA 0x03)検証");

            std::print(">");

            const std::uint32_t number = Utility::GetNumber(3);
            switch (number) {
                case 0:
                    break;
                case 1:
                    command.SetTunerSleep();
                    break;
                case 2:
                    command.ScanTest();
                    break;
                case 3:
                    command.CheckErrata();
                    break;
                default:
                    break;
            }
        }

    private:
        bool busy_ = false;
    };
}
