#pragma once

#include "buffer/ring_buffer.h"
#include "buffer/ts_stream.h"
#include "utility/utility.h"

namespace Earthsoft::Example::Command {
    class DeviceCommand {
    public:
        DeviceCommand(Pt3::Bus *bus, const Pt3::DeviceInfo *deviceInfo) {
            valid_ = Initialize(bus, deviceInfo);
        }

        ~DeviceCommand() = default;

        DeviceCommand(const DeviceCommand&) = delete;
        DeviceCommand& operator=(const DeviceCommand&) = delete;

        DeviceCommand(DeviceCommand&&) noexcept = default;
        DeviceCommand& operator=(DeviceCommand&&) noexcept = default;

        const bool IsValid() const noexcept {
            return valid_;
        }

        void SetLnbPower() {
            std::int32_t status = 0;

            Pt3::LnbPower lnbPower {};
            status = device_->GetLnbPower(&lnbPower);
            if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                Utility::PrintError("Earthsoft::Pt3::Device::GetLnbPower() に失敗しました。", status);
                return;
            }

            constexpr std::array labels { "オフ", "+15V", "+11V" };

            std::println("[LNB 電源設定]");
            std::println("0: (戻る)");
            for (std::size_t index = 0; index < labels.size(); ++index) {
                const char *marker = (static_cast<std::size_t>(lnbPower) == index)
                    ? "<- 現在の値"
                    : "";
                std::println("{}: {} {}", index + 1, labels[index], marker);
            }

            std::print(">");

            std::uint32_t number = Utility::GetNumber(static_cast<std::uint32_t>(labels.size()));
            if (number == 0) {
                return;
            }

            Pt3::LnbPower next = static_cast<Pt3::LnbPower>(number - 1);
            status = device_->SetLnbPower(next);
            if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                Utility::PrintError("Earthsoft::Pt3::Device::SetLnbPower() に失敗しました。", status);
                return;
            }
        }

        void ScanChannel() {
            auto result = SelectIsdbIndex();

            if (!result.has_value()) {
                return;
            }

            auto [isdb, tuner] = *result;

            switch (isdb) {
                case Pt3::Isdb::Satellite:
                    ScanSatteliteChannels(tuner, 0, 23);
                    break;
                case Pt3::Isdb::Terrestrial:
                    ScanTerrestrialChannels(tuner, 0, 112);
                    break;
                default:
                    break;
            }
        }

        void SetChannel() {
            auto result = SelectChannel();

            if (!result.has_value()) {
                return;
            }

            auto [isdb, tuner, channel] = *result;

            switch (isdb) {
                case Pt3::Isdb::Satellite:
                    ScanSatteliteChannels(tuner, channel, channel);
                    break;
                case Pt3::Isdb::Terrestrial:
                    ScanTerrestrialChannels(tuner, channel, channel);
                    break;
                default:
                    break;
            }
        }

        void SetTsId() {
            auto result = SelectTsId();

            if (!result.has_value()) {
                return;
            }

            auto [tuner, id] = *result;

            const std::int32_t status = device_->SetSatelliteId(tuner, id);
            if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                Utility::PrintError("Earthsoft::Pt3::Device::SetSatelliteId() に失敗しました。", status);
                return;
            }
        }

        void ShowErrorRateCount() {
            std::println("--+-----+---+--------+----------------+--------+--------+--------+--------");
            std::println("   推定  AGC RFレベル エラーパケット数 [誤り訂正されたビットレート       ]");
            std::println("   C/N          (dBm)                  [リードソロモン          ] ビタビ");
            std::println("    (dB)                               低階層   高階層");
            std::println("                                        A階層    B階層    C階層");
            std::println("--+-----+---+--------+----------------+--------+--------+--------+--------");

            std::int32_t status = 0;

            for (std::size_t isdbIndex = 0; isdbIndex < Pt3::IsdbCount; ++isdbIndex) {
                Pt3::Isdb isdb = static_cast<Pt3::Isdb>(isdbIndex);
                for (std::uint32_t tuner = 0; tuner < 2; ++tuner) {
                    std::uint32_t cn = 0;
                    std::uint32_t currentAgc = 0;
                    std::uint32_t maxAgc = 0;

                    status = device_->GetCnAgc(isdb, tuner, &cn, &currentAgc, &maxAgc);
                    if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                        Utility::PrintError("Earthsoft::Pt3::Device::GetCnAgc() に失敗しました。", status);
                    }

                    float rfLevel = 0.0;

                    if (isdb == Pt3::Isdb::Terrestrial) {
                        status = device_->GetRfLevel(tuner, &rfLevel);
                        if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                            Utility::PrintError("Earthsoft::Pt3::Device::GetRfLevel() に失敗しました。", status);
                        }
                    }

                    std::uint32_t errorCount = 0;

                    status = device_->GetErrorCount(isdb, tuner, &errorCount);
                    if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                        Utility::PrintError("Earthsoft::Pt3::Device::GetErrorCount() に失敗しました。", status);
                    }

                    std::string errorRateRs;

                    std::uint32_t layerCount = isdb == Pt3::Isdb::Satellite
                        ? Pt3::Satellite::LayerCount
                        : Pt3::Terrestrial::LayerCount;
                    
                    for (std::uint32_t layer = 0; layer < layerCount; ++layer) {
                        Pt3::ErrorRate errorRate {};
                        status = device_->GetCorrectedErrorRate(isdb, tuner, layer, &errorRate);
                        if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                            Utility::PrintError("Earthsoft::Pt3::Device::GetCorrectedErrorRate() に失敗しました。", status);
                        }

                        if (errorRate.Numerator == 0 || errorRate.Denominator == 0) {
                            errorRateRs += std::format(" {:8}", 0);
                        } else {
                            const double rate = static_cast<double>(errorRate.Numerator) / errorRate.Denominator;
                            errorRateRs += std::format(" {:8.2e}", rate);
                        }
                    }

                    if (isdb == Pt3::Isdb::Satellite) {
                        errorRateRs += "         ";
                    }

                    std::string errorRateVi;

                    Pt3::ErrorRate errorRate {};
                    status = device_->GetInnerErrorRate(isdb, tuner, &errorRate);
                    if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                        Utility::PrintError("Earthsoft::Pt3::Device::GetInnerErrorRate() に失敗しました。", status);
                    }

                    if (errorRate.Numerator == 0 || errorRate.Denominator == 0) {
                        errorRateVi += std::format(" {:8}", 0);
                    } else {
                        const double rate = static_cast<double>(errorRate.Numerator) / errorRate.Denominator;
                        errorRateVi += std::format(" {:8.2e}", rate);
                    }

                    std::println("{}{} {:5.2f} {:3} {} {:16}{}{}",
                        isdb == Pt3::Isdb::Satellite ? "S" : "T",
                        tuner + 1,
                        static_cast<double>(cn) / 100.0,
                        currentAgc,
                        isdb == Pt3::Isdb::Satellite ? "        " : std::format("{:8.3f}", rfLevel),
                        errorCount & 0x00ff'ffff,
                        errorRateRs,
                        errorRateVi
                    );
                }
            }
        }

        void StartCapture() {
            for (std::uint32_t isdbIndex = 0; isdbIndex < 2; ++isdbIndex) {
                Pt3::Isdb isdb = static_cast<Pt3::Isdb>(isdbIndex);
                for (std::uint32_t tuner = 0; tuner < 2; ++tuner) {
                    Buffer::TsStream &stream = stream_[isdbIndex * 2 + tuner];

                    if (!stream.IsValid()) {
                        stream = std::move(Buffer::TsStream(device_.get(), isdb, tuner));
                    }

                    if (stream.IsValid()) {
                        stream.Run();
                    }
                }
            }
        }

        void StopCapture() {
            for (std::uint32_t isdbIndex = 0; isdbIndex < 2; ++isdbIndex) {
                Pt3::Isdb isdb = static_cast<Pt3::Isdb>(isdbIndex);
                for (std::uint32_t tuner = 0; tuner < 2; ++tuner) {
                    Buffer::TsStream &stream = stream_[isdbIndex * 2 + tuner];

                    if (stream.IsValid()) {
                        stream.Stop();
                    }
                }
            }
        }

        bool CheckHardware() {
            std::int32_t status = CheckDmaTransferEnabled();
            if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                return false;
            }

            bool checkResult = [&]() -> bool {
                std::println("- 固定部が正しいかチェックしています...");
                if (!CheckConstantInfo()) {
                    return false;
                }

                std::println("- TS ピンをチェックしています...");
                if (!CheckTsPins()) {
                    return false;
                }

                std::println("- TS 同期バイトをチェックしています...");
                if (!CheckTsSyncByte()) {
                    return false;
                }

                std::println("- チューナーの PLL がロックするかチェックしています...");
                if (!CheckTunerPll()) {
                    return false;
                }

                std::println("- テストデータを転送してデータをチェックしています...");
                bool transferResult = CheckTransfer(false);
                if (transferResult) {
                    transferResult = CheckTransfer(true);
                }
                TransferCleanup();
                if (!transferResult) {
                    return false;
                }

                return true;
            }();

            device_->SetRamPinsMode(Pt3::RamPinsMode::Normal);
            if (checkResult) {
                std::println("┌─────────────┐");
                std::println("│ＯＫ  正常に完了しました。│");
                std::println("└─────────────┘");
            } else {
                std::println("■■■■■■■■■■■■■■■■■");
                std::println("■ ＮＧ  エラーが発生しました。 ■");
                std::println("■■■■■■■■■■■■■■■■■");
            }

            return checkResult;
        }

        void SetAmpPower() {
            std::println("0: (戻る)");
            std::println("1: オフ");
            std::println("2: オン");

            const std::uint32_t number = Utility::GetNumber(2);
            if (number == 0) {
                return;
            }

            const std::int32_t status = device_->SetAmpPower(number == 1);
            if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                Utility::PrintError("Earthsoft::Pt3::Device::SetAmpPower() に失敗しました。", status);
                return;
            }
        }

        void SetTunerSleep() {
            auto result = SelectIsdbIndex();

            if (!result.has_value()) {
                return;
            }

            auto [isdb, tuner] = *result;

            std::int32_t status = 0;
            bool sleep;
            status = device_->GetTunerSleep(isdb, tuner, &sleep);
            if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                Utility::PrintError("Earthsoft::Pt3::Device::GetTunerSleep() に失敗しました。", status);
                return;
            }

            constexpr std::array labels { "無効", "有効" };

            std::println("[チューナースリープ設定]");
            std::println("0: (戻る)");
            for (std::size_t index = 0; index < labels.size(); ++index) {
                const char *marker = (static_cast<std::size_t>(sleep) == index)
                    ? "<- 現在の値"
                    : "";
                std::println("{}: {} {}", index + 1, labels[index], marker);
            }

            std::print(">");

            std::uint32_t number = Utility::GetNumber(static_cast<std::uint32_t>(labels.size()));
            if (number == 0) {
                return;
            }

            bool next = number == 2;
            status = device_->SetTunerSleep(isdb, tuner, next);
            if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                Utility::PrintError("Earthsoft::Pt3::Device::SetTunerSleep() に失敗しました。", status);
                return;
            }
        }

        void ScanTest() {
            ScanSatteliteChannels(0,  7,  7);
            ScanSatteliteChannels(1, 15, 15);

            ScanTerrestrialChannels(0, 70, 70);
            ScanTerrestrialChannels(1, 71, 71);

            for (std::uint32_t tuner = 0; tuner < 2; ++tuner) {
                for (std::uint32_t isdbIndex = 0; isdbIndex < Pt3::IsdbCount; ++isdbIndex) {
                    Pt3::Isdb isdb = static_cast<Pt3::Isdb>(isdbIndex);

                    std::int32_t status = device_->ResetCorrectedErrorCount(isdb, tuner);
                    if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                        Utility::PrintError("Earthsoft::Pt3::Device::ResetCorrectedErrorCount() に失敗しました。", status);
                        return;
                    }
                }
            }
        }

        std::int32_t CheckErrata() {
            std::int32_t status = CheckDmaTransferEnabled();
            if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                return status;
            }

            std::println("試行回数を入力してください。");
            std::print(">");

            std::uint32_t count = Utility::GetNumber(9);
            if (count == 0) {
                return 0;
            }

            for (std::uint32_t index = 0; index < count; ++index) {
                std::println("{} 回目の試行を開始します。", index);

                status = [&]() -> std::int32_t {
                    std::int32_t status = 0;

                    std::println("- [{}] Allocate", index);
                    status = Allocate();
                    if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                        return status;
                    }

                    std::println("- [{}] Transfer", index);
                    status = CheckErattaTransfer();
                    if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                        return status;
                    }

                    std::println("- [{}] Cleanup", index);
                    TransferCleanup();

                    std::println("- [{}] Free", index);
                    status = Free();
                    if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                        return status;
                    }

                    return status;
                }();

                if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                    break;
                }

                std::println("{} 回目の試行は正常に終了しました。", index);
            }

            TransferCleanup();

            if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                return status;
            }

            std::println("全ての試行は正常に終了しました。");

            return status;
        }

    private:
        bool Initialize(Pt3::Bus *bus, const Pt3::DeviceInfo *deviceInfo) {
            std::int32_t status = 0;

            Pt3::Device *devicePtr = nullptr;
            status = bus->NewDevice(deviceInfo, &devicePtr);
            if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                Utility::PrintError("Earthsoft::Pt3::Bus::NewDevice() に失敗しました。", status);
                return false;
            }
            if (devicePtr == nullptr) {
                Utility::PrintError("Earthsoft::Pt3::Device が nullptr です。");
                return false;
            }

            device_.reset(devicePtr);
            device_.get_deleter().Outer = this;

            status = device_->Open();
            if (status == static_cast<std::int32_t>(Pt3::Status::InvalidFpgaVersion)) {
                Pt3::ConstantInfo info {};
                status = device_->GetConstantInfo(&info);
                if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                    Utility::PrintError("Earthsoft::Pt3::Device::GetConstantInfo() に失敗しました。", status);
                    return false;
                }

                std::uint8_t version = info.FpgaVersion;

                std::print("[ERROR] 回路番号 {:#04x} には対応していません。", version);
                if (version <= 0x03) {
                    std::println("回路を更新してください。");
                }

                return false;
            }
            if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                Utility::PrintError("Earthsoft::Pt3::Device::Open() に失敗しました。", status);
                return false;
            }
            open_ = true;

            status = device_->InitTuner();
            if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                Utility::PrintError("Earthsoft::Pt3::Device::InitTuner() に失敗しました。", status);
                return false;
            }

            for (std::uint32_t index = 0; index < 2; ++index) {
                for (std::uint32_t isdb = 0; isdb < 2; ++isdb) {
                    status = device_->SetTunerSleep(static_cast<Pt3::Isdb>(isdb), index, false);
                    if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                        Utility::PrintError("Earthsoft::Pt3::Device::SetTunerSleep() に失敗しました。", status);
                        return false;
                    }
                }
            }

            return true;
        }

        std::expected<std::pair<Pt3::Isdb, std::uint32_t>, bool> SelectIsdbIndex(bool satelliteOnly = false) {
            std::println("0: (戻る)");
            std::println("1: S1");
            std::println("2: S2");
            if (!satelliteOnly) {
                std::println("3: T1");
                std::println("4: T2");
            }

            const std::uint32_t number = Utility::GetNumber(satelliteOnly ? 2 : 4);
            if (number == 0) {
                return std::unexpected(false);
            }

            const std::uint32_t selected = number - 1;
            const Pt3::Isdb isdb = static_cast<Pt3::Isdb>(selected / 2);
            const std::uint32_t index = static_cast<std::uint32_t>(selected % 2);

            return std::pair { isdb, index };
        }

        std::expected<std::pair<std::uint32_t, std::uint32_t>, bool> SelectTsId() {
            auto result = SelectIsdbIndex(/* satellite: */ false);

            if (!result.has_value()) {
                return std::unexpected(false);
            }

            auto [isdb, index] = *result;

            constexpr std::uint32_t maxId = 0xffff;
            std::println("TS-ID を入力してください。(範囲:0x0000～0x{:04x})", maxId);
            std::print(">0x");

             const std::uint32_t id = Utility::GetHexNumber(maxId);

            return std::pair { index, id };
        }

        std::expected<std::tuple<Pt3::Isdb, std::uint32_t, std::uint32_t>, bool> SelectChannel() {
            auto result = SelectIsdbIndex();

            if (!result.has_value()) {
                return std::unexpected(false);
            }

            auto [isdb, index] = *result;

            std::uint32_t max = isdb == Pt3::Isdb::Satellite
                ? 23
                : 112;

            std::println("チャンネル番号を入力してください。(範囲:0～{})", max);
            std::print(">");

            std::uint32_t channel = Utility::GetNumber(max);

            return std::tuple { isdb, index, channel };
        }

        std::pair<bool, std::uint32_t> GetSatteliteChannelName(std::uint32_t channel) {
            if (channel < 12) {
                return std::pair { true, 1 + 2 * channel };
            } else if (channel < 24) {
                return std::pair { false, 2 + 2 * (channel - 12) };
            } else {
                return std::pair { false, 1 + 2 * (channel - 24) };
            }
        }

        std::int32_t ScanSatteliteChannel(std::uint32_t tuner, std::uint32_t channel) {
            std::int32_t status = 0;

            auto [bs, number] = GetSatteliteChannelName(channel);
            std::print("{:3} {}{:02}", channel, bs ? "BS" : "ND", number);

            status = device_->SetFrequency(Pt3::Isdb::Satellite, tuner, channel);
            if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                Utility::PrintError("Earthsoft::Pt3::Device::SetFrequency() に失敗しました。", status);
                return status;
            }

            using namespace std::chrono_literals;
            const auto startTime = std::chrono::steady_clock::now();
            Pt3::Satellite::Tmcc tmcc {};
            bool validTmcc = false;

            while (true) {
                status = device_->GetSatelliteTmcc(tuner, &tmcc);
                if (status == static_cast<std::int32_t>(Pt3::Status::Ok)) {
                    validTmcc = true;
                    break;
                }
                if (status != static_cast<std::int32_t>(Pt3::Status::Ok)
                    && status != static_cast<std::int32_t>(Pt3::Status::Unspecified)) {
                    Utility::PrintError("Earthsoft::Pt3::Device::GetSatelliteTmcc() に失敗しました。", status);
                }

                if (std::chrono::steady_clock::now() - startTime > 1s) {
                    break;
                }

                std::this_thread::sleep_for(10ms);
            }

            std::uint32_t cn = 0;
            std::uint32_t currentAgc = 0;
            std::uint32_t maxAgc = 0;

            status = device_->GetCnAgc(Pt3::Isdb::Satellite, tuner, &cn, &currentAgc, &maxAgc);
            if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                Utility::PrintError("Earthsoft::Pt3::Device::GetCnAgc() に失敗しました。", status);
                return status;
            }

            std::print(" {:3}", currentAgc);

            if (!validTmcc) {
                std::println(" (TMCC 受信不可)");
                return status;
            }

            std::array<std::int32_t, 2> offsets;
            status = device_->GetFrequencyOffset(Pt3::Isdb::Satellite, tuner, &offsets[0], &offsets[1]);
            if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                Utility::PrintError("Earthsoft::Pt3::Device::GetFrequencyOffset() に失敗しました。", status);
                return status;
            }

            std::print(" {:+7.2f} {:+6}",
                static_cast<double>(offsets[0]) / 100.0,
                offsets[1] / 1000
            );

            std::print(" {:2} {:2} {:2}", tmcc.Indicator, tmcc.Emergency, tmcc.UpLink);

            for (std::size_t index = 0; index < 4; ++index) {
                const std::uint32_t mode = tmcc.Mode[index];
                const std::uint32_t slot = tmcc.Slot[index];
                std::print(" {}/{}",
                    mode != 0xf ? std::to_string(mode) : "-",
                    mode != 0xf ? std::format("{:2}", slot) : "--"
                );
            }

            std::size_t lastIndex = 0;
            for (std::size_t index = 0; index < 8; ++index) {
                if (tmcc.Id[index] != 0xffff) {
                    lastIndex = index;
                }
            }

            for (std::size_t index = 0; index < 8; ++index) {
                const std::uint32_t id = tmcc.Id[index];
                std::print("{}",
                    id != 0xffff
                        ? std::format(" {:04x}", id)
                        : index < lastIndex
                        ? " ----"
                        : ""
                );
            }

            std::println();

            return status;
        }

        std::int32_t ScanSatteliteChannels(std::uint32_t tuner, std::uint32_t beginChannel, std::uint32_t endChannel) {
            std::int32_t status = 0;

            std::println("                        変:変更指示 / 起:起動制御信号 / ア:アップリンク制御情報");
            std::println("---+----+---+-------+------+--+--+--+-------------------+----------------------");
            std::println("No. Ch.  AGC Δclock Δcarr 変 起 ア 伝送モード/Slot数   TS-ID (Hex)");
            std::println("        /128   (ppm)  (kHz)          1    2    3    4    1    2    3    4    5");
            std::println("---+----+---+-------+------+--+--+--+-------------------+----------------------");

            for (std::uint32_t channel = beginChannel; channel <= endChannel; ++channel) {
                status = ScanSatteliteChannel(tuner, channel);
                if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                    break;
                }
            }

            std::println("---+----+---+-------+------+--+--+--+-------------------+----------------------");

            return status;
        }

        std::pair<bool, std::uint32_t> GetTerrestrialChannelName(std::uint32_t channel) {
            constexpr std::array<std::tuple<std::uint32_t, bool, std::uint32_t>, 5> ranges {
                std::tuple {   2, false,  3},
                std::tuple {  12,  true, 22},
                std::tuple {  21, false, 12},
                std::tuple {  62,  true, 63},
                std::tuple { 112, false, 62},
            };

            bool catv = false;
            std::uint32_t number = 0;

            for (const auto [maxChannel, isCatv, offset] : ranges) {
                if (channel <= maxChannel) {
                    catv = isCatv;
                    number = channel + offset - maxChannel;
                    break;
                }
            }

            return std::pair { catv, number };
        }

        std::int32_t ScanTerrestrialChannel(std::uint32_t tuner, std::uint32_t channel) {
            std::int32_t status = 0;

            auto [catv, number] = GetTerrestrialChannelName(channel);
            std::print("{:3} {}{:02}", channel, catv ? "C" : " ", number);

            status = device_->SetFrequency(Pt3::Isdb::Terrestrial, tuner, channel);
            if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                Utility::PrintError("Earthsoft::Pt3::Device::SetFrequency() に失敗しました。", status);
                return status;
            }

            Pt3::Terrestrial::Tmcc tmcc {};
            bool validTmcc = false;
            status = device_->GetTerrestrialTmcc(tuner, &tmcc);
            if (status == static_cast<std::int32_t>(Pt3::Status::Ok)) {
                validTmcc = true;
            }
            if (status != static_cast<std::int32_t>(Pt3::Status::Ok)
                && status != static_cast<std::int32_t>(Pt3::Status::Unspecified)) {
                Utility::PrintError("Earthsoft::Pt3::Device::GetTerrestrialTmcc() に失敗しました。", status);
            }

            std::uint32_t cn = 0;
            std::uint32_t currentAgc = 0;
            std::uint32_t maxAgc = 0;

            status = device_->GetCnAgc(Pt3::Isdb::Terrestrial, tuner, &cn, &currentAgc, &maxAgc);
            if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                Utility::PrintError("Earthsoft::Pt3::Device::GetCnAgc() に失敗しました。", status);
                return status;
            }

            std::print(" {:3}", currentAgc);

            if (!validTmcc) {
                std::println(" (TMCC 受信不可)");
                return status;
            }

            std::array<std::int32_t, 2> offsets;
            status = device_->GetFrequencyOffset(Pt3::Isdb::Terrestrial, tuner, &offsets[0], &offsets[1]);
            if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                Utility::PrintError("Earthsoft::Pt3::Device::GetFrequencyOffset() に失敗しました。", status);
                return status;
            }

            std::print(" {:+7.2f} {:+6}",
                static_cast<double>(offsets[0]) / 100.0,
                offsets[1] / 1000
            );

            std::print(" {}", tmcc.System);

            for (std::size_t index = 0; index < 3; ++index) {
                const std::uint32_t mode = tmcc.Mode[index];
                const std::uint32_t rate = tmcc.Rate[index];
                const std::uint32_t interleave = tmcc.Interleave[index];
                const std::uint32_t segment = tmcc.Segment[index];

                std::print(" {}/{}/{}/{}",
                    (mode != 7) ? std::to_string(mode) : "-",
                    (mode != 7) ? std::to_string(rate) : "-",
                    (mode != 7) ? std::to_string(interleave) : "-",
                    (segment != 15) ? std::format("{:02}", segment) : "--"
                );
            }

            std::println("");

            return status;
        }

        std::int32_t ScanTerrestrialChannels(std::uint32_t tuner, std::uint32_t beginChannel, std::uint32_t endChannel) {
            std::int32_t status = 0;

            std::println("---+---+---+-------+------+-+------------------------------------------");
            std::println("No. Ch. AGC Δclock Δcarr S 変調/符号化率/インターリーブ/セグメント数");
            std::println("        255   (ppm)  (kHz)   A階層    B階層    C階層");
            std::println("---+---+---+-------+------+-+------------------------------------------");

            for (std::uint32_t channel = beginChannel; channel <= endChannel; ++channel) {
                status = ScanTerrestrialChannel(tuner, channel);
                if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                    break;
                }
            }

            std::println("---+---+---+-------+------+-+------------------------------------------");

            return status;
        }

        std::int32_t CheckDmaTransferEnabled() {
            for (std::size_t isdbIndex = 0; isdbIndex < Pt3::IsdbCount; ++isdbIndex) {
                Pt3::Isdb isdb = static_cast<Pt3::Isdb>(isdbIndex);
                for (std::uint32_t tuner = 0; tuner < 2; ++tuner) {
                    bool enabled = false;
                    std::int32_t status = device_->GetTransferEnabled(isdb, tuner, &enabled);
                    if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                        Utility::PrintError("Earthsoft::Pt3::Device::GetTransferEnabled() に失敗しました。", status);
                        return status;
                    }
                    if (enabled) {
                        Utility::PrintError("すべての DMA が停止状態ではないため実行できません");
                        return 0x314;
                    }
                }
            }

            return static_cast<std::int32_t>(Pt3::Status::Ok);
        }

        std::int32_t Allocate() {
            for (std::size_t isdbIndex = 0; isdbIndex < Pt3::IsdbCount; ++isdbIndex) {
                Pt3::Isdb isdb = static_cast<Pt3::Isdb>(isdbIndex);
                for (std::uint32_t tuner = 0; tuner < 2; ++tuner) {
                    Buffer::RingBuffer &buffer = buffer_[isdbIndex * 2 + tuner];

                    if (!buffer.IsValid()) {
                        buffer = std::move(Buffer::RingBuffer(device_.get()));
                    }

                    if (!buffer.IsAllocated()) {
                        std::int32_t status = buffer.Allocate(1024 * 1024, 1, false);
                        if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                            Utility::PrintError("Earthsoft::Example::Buffer::Allocate() に失敗しました。", status);
                            return status;
                        }
                    }
                }
            }

            return static_cast<std::int32_t>(Pt3::Status::Ok);
        }

        std::int32_t Free() {
            for (auto &buffer : buffer_) {
                std::int32_t status = buffer.Free();
                if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                    Utility::PrintError("Earthsoft::Example::Buffer::Free() に失敗しました。", status);
                    // return status;
                }
            }

            return static_cast<std::int32_t>(Pt3::Status::Ok);
        }

        std::int32_t CheckErattaTransfer() {
            for (std::size_t isdbIndex = 0; isdbIndex < Pt3::IsdbCount; ++isdbIndex) {
                Pt3::Isdb isdb = static_cast<Pt3::Isdb>(isdbIndex);
                for (std::uint32_t tuner = 0; tuner < 2; ++tuner) {
                    Buffer::RingBuffer &buffer = buffer_[isdbIndex * 2 + tuner];

                    if (buffer.IsValid()) {
                        std::int32_t status = device_->SetTransferPageDescriptorAddress(isdb, tuner, buffer.PageDescriptorAddress());
                        if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                            Utility::PrintError("Earthsoft::Pt3::Device::SetTransferPageDescriptorAddress() に失敗しました。", status);
                            return status;
                        }
                    }
                }
            }

            for (std::size_t isdbIndex = 0; isdbIndex < Pt3::IsdbCount; ++isdbIndex) {
                Pt3::Isdb isdb = static_cast<Pt3::Isdb>(isdbIndex);
                for (std::uint32_t tuner = 0; tuner < 2; ++tuner) {
                    std::int32_t status = 0;

                    std::uint16_t lfsr = GetTransferLfsr(isdb, tuner);
                    status = device_->SetTransferTestMode(isdb, tuner, true, lfsr);
                    if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                        Utility::PrintError("Earthsoft::Pt3::Device::SetTransferTestMode() に失敗しました。", status);
                        return status;
                    }

                    Buffer::RingBuffer &buffer = buffer_[isdbIndex * 2 + tuner];

                    void *ptr = buffer.Ptr(0);
                    std::fill_n(static_cast<std::uint8_t *>(ptr), 1024 * 1024, 0);

                    status = buffer.SyncCpu(0);
                    if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                        Utility::PrintError("Earthsoft::Example::Buffer::SyncCpu() に失敗しました。", status);
                        return status;
                    }

                    status = device_->SetTransferEnabled(isdb, tuner, true);
                    if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                        Utility::PrintError("Earthsoft::Pt3::Device::SetTransferEnabled() に失敗しました。", status);
                        return status;
                    }
                }
            }

            // ランダム値で対象のチューナーを選んで、ランダム値でスリープしてから止める
            Pt3::Isdb stopIsdb {};
            std::uint32_t stopTuner = 0;
            {
                thread_local std::mt19937 engine(std::random_device{}());
                std::uniform_int_distribution<std::uint32_t> targetDistribution(0, 1);
                std::uniform_int_distribution<std::uint32_t> durationDistiribution(10, 49);

                stopIsdb = static_cast<Pt3::Isdb>(targetDistribution(engine));
                stopTuner = targetDistribution(engine);

                const auto sleepDuration = std::chrono::milliseconds(durationDistiribution(engine));
                std::this_thread::sleep_for(sleepDuration);

                std::int32_t status = 0;

                status = device_->SetTransferEnabled(stopIsdb, stopTuner, false);
                if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                    Utility::PrintError("Earthsoft::Pt3::Device::SetTransferEnabled() に失敗しました。", status);
                    return status;
                }

                status = device_->SetTransferTestMode(stopIsdb, stopTuner);
                if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                    Utility::PrintError("Earthsoft::Pt3::Device::SetTransferTestMode() に失敗しました。", status);
                    return status;
                }

                std::println("- ISDB-{}{} を停止します", stopIsdb == Pt3::Isdb::Satellite ? "S" : "T", stopTuner);
            }

            for (std::size_t isdbIndex = 0; isdbIndex < Pt3::IsdbCount; ++isdbIndex) {
                Pt3::Isdb isdb = static_cast<Pt3::Isdb>(isdbIndex);
                for (std::uint32_t tuner = 0; tuner < 2; ++tuner) {
                    bool done = false;

                    using namespace std::chrono_literals;
                    const auto startTime = std::chrono::steady_clock::now();

                    while (true) {
                        Pt3::TransferInfo transferInfo {};
                        std::int32_t status = device_->GetTransferInfo(isdb, tuner, &transferInfo);
                        if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                            Utility::PrintError("Earthsoft::Pt3::Device::GetTransferInfo() に失敗しました。", status);
                            return false;
                        }

                        if (transferInfo.InternalFifoAOverflow) {
                            done = true;
                            std::println("- Internal FIFO A Overflow が発生しました。");
                        }

                        if (transferInfo.InternalFifoAUnderflow) {
                            done = true;
                            std::println("- Internal FIFO A Underflow が発生しました。");
                        }

                        if (transferInfo.ExternalFifoOverflow) {
                            done = true;
                            std::println("- External FIFO Overflow が発生しました。");
                        }

                        if (transferInfo.InternalFifoBOverflow) {
                            done = true;
                            std::println("- Internal FIFO B Overflow が発生しました。");
                        }

                        if (transferInfo.InternalFifoBUnderflow) {
                            done = true;
                            std::println("- Internal FIFO B Underflow が発生しました。");
                        }

                        if (!transferInfo.Busy) {
                            done = true;
                            break;
                        }

                        if (std::chrono::steady_clock::now() - startTime > 2s) {
                            break;
                        }                        

                        std::this_thread::sleep_for(1ms);
                    }

                    if (!done) {
                        std::println("- 転送が完了しませんでした。");
                        return 0x314;
                    }
                }
            }

            for (std::size_t isdbIndex = 0; isdbIndex < Pt3::IsdbCount; ++isdbIndex) {
                Pt3::Isdb isdb = static_cast<Pt3::Isdb>(isdbIndex);
                for (std::uint32_t tuner = 0; tuner < 2; ++tuner) {
                    std::println("- ISDB-{}{}", isdb == Pt3::Isdb::Satellite ? "S" : "T", tuner);
                    if (isdb == stopIsdb && tuner == stopTuner) {
                        std::println("- 停止しています。");
                       continue; 
                    }

                    Buffer::RingBuffer &buffer = buffer_[isdbIndex * 2 + tuner];

                    std::int32_t status = buffer.SyncIo(0);
                    if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                        Utility::PrintError("Earthsoft::Example::Buffer::SyncIo() に失敗しました。", status);
                        return status;
                    }

                    const std::uint16_t *ptr = static_cast<const std::uint16_t *>(buffer.Ptr(0));
                    std::span<const std::uint16_t> stream(ptr, 1024 * 1024 / sizeof(std::uint16_t));
                    std::uint16_t lfsr = GetTransferLfsr(isdb, tuner);

                    for (std::uint16_t current : stream) {
                        if (current != lfsr) {
                            std::println("- 転送データに誤りがありました。current: {:#06x} lfsr: {:#06x}", current, lfsr);
                            return false;
                        }
                        lfsr = (lfsr >> 1) ^ (-(lfsr & 1) & 0xb400);
                    }
                    std::println("- 転送データは問題ありません。");
                }
            }

            return static_cast<std::int32_t>(Pt3::Status::Ok);
        }

        std::int32_t StopTransfer() {
            for (std::uint32_t isdbIndex = 0; isdbIndex < Pt3::IsdbCount; ++isdbIndex) {
                Pt3::Isdb isdb = static_cast<Pt3::Isdb>(isdbIndex);
                for (std::uint32_t tuner = 0; tuner < 2; ++tuner) {
                    std::int32_t status = 0;

                    status = device_->SetTransferEnabled(isdb, tuner, false);
                    if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                        Utility::PrintError("Earthsoft::Pt3::Device::SetTransferEnabled() に失敗しました。", status);
                        // return status;
                    }
                    status = device_->SetTransferTestMode(isdb, tuner, false);
                    if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                        Utility::PrintError("Earthsoft::Pt3::Device::SetTransferTestMode() に失敗しました。", status);
                        // return status;
                    }
                }
            }

            return static_cast<std::int32_t>(Pt3::Status::Ok);
        }

        bool CheckConstantInfo() {
            Pt3::ConstantInfo constantInfo {};
            std::int32_t status = device_->GetConstantInfo(&constantInfo);
            if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                Utility::PrintError("Earthsoft::Pt3::Device::GetConstantInfo() に失敗しました。", status);
                return false;
            }
            if (constantInfo.PtVersion != 0x03) {
                std::println("  - PtVersion ({:#04x}) が誤っています。", constantInfo.PtVersion);
                return false;
            }
            if (constantInfo.RegisterMapVersion != 0x01) {
                std::println("  - RegisterMapVersion ({:#04x}) が誤っています。", constantInfo.RegisterMapVersion);
                return false;
            }
            if (constantInfo.FpgaVersion != 0x04) {
                std::println("  - FpgaVersion ({:#04x}) が誤っています。", constantInfo.FpgaVersion);
                return false;
            }
            if (constantInfo.IsTsSupported != true) {
                std::println("  - IsTsSupported ({}) が誤っています。", constantInfo.IsTsSupported);
                return false;
            }

            return true;
        }

        bool CheckTsPins() {
            for (std::uint32_t modeIndex = 0; modeIndex < 3; ++modeIndex) {
                Pt3::RamPinsMode mode = static_cast<Pt3::RamPinsMode>(2 - modeIndex);
                std::uint32_t status = device_->SetRamPinsMode(mode);
                if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                    Utility::PrintError("Earthsoft::Pt3::Device::SetRamPinsMode() に失敗しました。", status);
                    return false;
                }

                for (std::uint32_t index = 0; index < 256; ++index) {
                    std::uint32_t level = 0;

                    if (!CheckTsPins(index, &level)) {
                        return false;
                    }

                    if (index != level) {
                        std::println("{:02x}, {:02x}", index, level);
                    }
                }

                std::println("  - RAM ピンモード {} は問題ありません。", 2 - modeIndex);
            }

            Pt3::TsPinsMode mode {
                .ClockData = Pt3::TsPinMode::Normal,
                .Byte      = Pt3::TsPinMode::Normal,
                .Valid     = Pt3::TsPinMode::Normal,
            };

            for (std::uint32_t isdbIndex = 0; isdbIndex < Pt3::IsdbCount; ++isdbIndex) {
                Pt3::Isdb isdb = static_cast<Pt3::Isdb>(isdbIndex);
                for (std::uint32_t tuner = 0; tuner < 2; ++tuner) {
                    std::uint32_t status = device_->SetTsPinsMode(isdb, tuner, &mode);
                    if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                        Utility::PrintError("Earthsoft::Pt3::Device::SetTsPinsMode() に失敗しました。", status);
                    }
                }
            }

            using namespace std::chrono_literals;
            std::this_thread::sleep_for(10ms);

            return true;
        }

        bool CheckTsPins(std::uint32_t mode, std::uint32_t *level) {
            std::array<std::array<Pt3::TsPinsMode, 2>, 2> table {};
            std::uint32_t bit = 0;
            for (auto& row : table) {
                for (auto& pin : row) {
                    pin.Byte  = (mode & (1 << (bit + 0))) ? Pt3::TsPinMode::High : Pt3::TsPinMode::Low;
                    pin.Valid = (mode & (1 << (bit + 1))) ? Pt3::TsPinMode::High : Pt3::TsPinMode::Low;
                    
                    bit += 2;
                }
            }

            for (std::uint32_t isdbIndex = 0; isdbIndex < Pt3::IsdbCount; ++isdbIndex) {
                Pt3::Isdb isdb = static_cast<Pt3::Isdb>(isdbIndex);
                for (std::uint32_t tuner = 0; tuner < 2; ++tuner) {
                    Pt3::TsPinsMode &pin = table[isdbIndex][tuner];

                    std::array<bool, 4> clocks {};
                    for (bool &clock : clocks) {
                        for (std::uint32_t index = 0; index < 2; ++index) {
                            pin.ClockData = (index == 0) ? Pt3::TsPinMode::High : Pt3::TsPinMode::Low;

                            std::int32_t status = device_->SetTsPinsMode(isdb, tuner, &pin);
                            if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                                Utility::PrintError("Earthsoft::Pt3::Device::SetTsPinsMode() に失敗しました。", status);
                            }

                            if (index == 0) {
                                Pt3::TsPinsLevel pinsLevel;
                                std::int32_t status = device_->GetTsPinsLevel(isdb, tuner, &pinsLevel);
                                if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                                    Utility::PrintError("Earthsoft::Pt3::Device::GetTsPinsLevel() に失敗しました。", status);
                                }

                                clock = pinsLevel.Clock;
                            }
                        }
                    }

                    if (clocks[0] == clocks[1] || clocks[1] == clocks[2] || clocks[2] == clocks[3]) {
                        std::println("  - TS クロックに異常があります。");
                        return false;
                    }
                }
            }

            std::uint32_t culculatedLevel = 0;
            bit = 0;
            for (std::uint32_t isdbIndex = 0; isdbIndex < Pt3::IsdbCount; ++isdbIndex) {
                Pt3::Isdb isdb = static_cast<Pt3::Isdb>(isdbIndex);
                for (std::uint32_t tuner = 0; tuner < 2; ++tuner) {
                    Pt3::TsPinsLevel pinsLevel;
                    std::int32_t status = device_->GetTsPinsLevel(isdb, tuner, &pinsLevel);
                    if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                        Utility::PrintError("Earthsoft::Pt3::Device::GetTsPinsLevel() に失敗しました。", status);
                    }

                    culculatedLevel |= (pinsLevel.Byte  ? 1 : 0) << (bit + 0);
                    culculatedLevel |= (pinsLevel.Valid ? 1 : 0) << (bit + 1);

                    bit += 2;
                }
            }
            *level = culculatedLevel;

            return true;
        }

        bool CheckTsSyncByte() {
            for (std::size_t isdbIndex = 0; isdbIndex < Pt3::IsdbCount; ++isdbIndex) {
                Pt3::Isdb isdb = static_cast<Pt3::Isdb>(isdbIndex);
                for (std::uint32_t tuner = 0; tuner < 2; ++tuner) {
                    std::uint8_t syncByte = 0x00;
                    std::int32_t status = device_->GetTsSyncByte(isdb, tuner, &syncByte);
                    if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                        Utility::PrintError("Earthsoft::Pt3::Device::GetTsSyncByte() に失敗しました。", status);
                        return false;
                    }
                    if (syncByte != 0x47) {
                        std::println("  - 同期バイトに異常があります。 ({:#04x})", syncByte);
                        return false;
                    }
                }
            }

            return true;
        }

        bool CheckTunerPll() {
            for (std::size_t isdbIndex = 0; isdbIndex < Pt3::IsdbCount; ++isdbIndex) {
                Pt3::Isdb isdb = static_cast<Pt3::Isdb>(isdbIndex);
                for (std::uint32_t tuner = 0; tuner < 2; ++tuner) {
                    std::uint32_t status = device_->SetFrequency(isdb, tuner, 0);
                    if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                        Utility::PrintError("Earthsoft::Pt3::Device::SetFrequency() に失敗しました。", status);
                        return false;
                    }
                }
            }

            return true;
        }

        bool CheckTransfer(bool notOpLfsr) {
            for (std::size_t isdbIndex = 0; isdbIndex < Pt3::IsdbCount; ++isdbIndex) {
                Pt3::Isdb isdb = static_cast<Pt3::Isdb>(isdbIndex);
                for (std::uint32_t tuner = 0; tuner < 2; ++tuner) {
                    Buffer::RingBuffer &buffer = buffer_[isdbIndex * 2 + tuner];

                    if (!buffer.IsValid()) {
                        buffer = std::move(Buffer::RingBuffer(device_.get()));
                        // buffer.~Buffer();
                        // ::new (&buffer) Buffer(device_);
                        buffer.Allocate(1024 * 1024, 1, false);

                        std::int32_t status = device_->SetTransferPageDescriptorAddress(isdb, tuner, buffer.PageDescriptorAddress());
                        if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                            Utility::PrintError("Earthsoft::Pt3::Device::SetTransferPageDescriptorAddress() に失敗しました。", status);
                            return false;
                        }
                    }
                }
            }

            for (std::size_t isdbIndex = 0; isdbIndex < Pt3::IsdbCount; ++isdbIndex) {
                Pt3::Isdb isdb = static_cast<Pt3::Isdb>(isdbIndex);
                for (std::uint32_t tuner = 0; tuner < 2; ++tuner) {
                    Buffer::RingBuffer &buffer = buffer_[isdbIndex * 2 + tuner];

                    void *ptr = buffer.Ptr(0);
                    std::fill_n(static_cast<std::uint8_t *>(ptr), 1024 * 1024, 0);

                    std::int32_t status = 0;

                    status = buffer.SyncCpu(0);
                    if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                        Utility::PrintError("Earthsoft::Example::Buffer::SyncCpu() に失敗しました。", status);
                        return false;
                    }

                    std::uint16_t lfsr = GetTransferLfsr(isdb, tuner);
                    status = device_->SetTransferTestMode(isdb, tuner, true, lfsr, notOpLfsr);
                    if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                        Utility::PrintError("Earthsoft::Pt3::Device::SetTransferTestMode() に失敗しました。", status);
                        return false;
                    }

                    status = device_->SetTransferEnabled(isdb, tuner, true);
                    if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                        Utility::PrintError("Earthsoft::Pt3::Device::SetTransferEnabled() に失敗しました。", status);
                        return false;
                    }
                }
            }

            for (std::size_t isdbIndex = 0; isdbIndex < Pt3::IsdbCount; ++isdbIndex) {
                Pt3::Isdb isdb = static_cast<Pt3::Isdb>(isdbIndex);
                for (std::uint32_t tuner = 0; tuner < 2; ++tuner) {
                    bool done = false;

                    using namespace std::chrono_literals;
                    const auto startTime = std::chrono::steady_clock::now();

                    while (true) {
                        Pt3::TransferInfo transferInfo {};
                        std::int32_t status = device_->GetTransferInfo(isdb, tuner, &transferInfo);
                        if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                            Utility::PrintError("Earthsoft::Pt3::Device::GetTransferInfo() に失敗しました。", status);
                            return false;
                        }
                        if (!transferInfo.Busy) {
                            done = true;
                            break;
                        }

                        if (std::chrono::steady_clock::now() - startTime > 2s) {
                            break;
                        }                        

                        std::this_thread::sleep_for(1ms);
                    }

                    if (!done) {
                        std::println("  - 転送が完了しませんでした。");
                        return false;
                    }

                    Buffer::RingBuffer &buffer = buffer_[isdbIndex * 2 + tuner];

                    std::int32_t status = buffer.SyncIo(0);
                    if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                        Utility::PrintError("Earthsoft::Example::Buffer::SyncIo() に失敗しました。", status);
                        return false;
                    }

                    const std::uint16_t *ptr = static_cast<const std::uint16_t *>(buffer.Ptr(0));
                    std::span<const std::uint16_t> stream(ptr, 1024 * 1024 / sizeof(std::uint16_t));
                    std::uint16_t lfsr = GetTransferLfsr(isdb, tuner);

                    for (std::uint16_t current : stream) {
                        if (current != (notOpLfsr ? lfsr ^ 0xFFFFu : lfsr)) {
                            std::println("  - 転送データに誤りがありました。current: {:#06x} lfsr: {:#06x} not: {:#06x} flag: {}", current, lfsr, lfsr ^ 0xFFFFu, notOpLfsr);
                            return false;
                        }
                        lfsr = (lfsr >> 1) ^ (-(lfsr & 1) & 0xb400);
                    }
                }
            }

            return true;
        }

        void TransferCleanup() {
            std::int32_t status = 0;

            status = StopTransfer();
            if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                // return;
            }

            status = Free();
            if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                // return;
            }
        }

        std::uint16_t GetTransferLfsr(Pt3::Isdb isdb, std::uint32_t tuner) const
        {
            std::uint32_t lfsr = (1 + 2 * static_cast<std::uint32_t>(isdb) + tuner) * 12345; 

            return static_cast<std::uint16_t>(lfsr);
        }

    private:
        struct DeviceDeleter {
            DeviceCommand *Outer = nullptr;

            void operator()(Pt3::Device *device) const noexcept {
                std::int32_t status = 0;

                if (device != nullptr) {
                    if (Outer != nullptr && Outer->open_ == true) {
                        status = device->Close();
                        if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                            Utility::PrintError("Earthsoft::Pt3::Device::Close() で失敗しました。", status);
                            return;
                        }
                        Outer->open_ = false;
                    }

                    status = device->Delete();
                    if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                        Utility::PrintError("Earthsoft::Pt3::Device::Delete() で失敗しました。", status);
                        return;
                    }
                }
            }
        };

        std::unique_ptr<Pt3::Device, DeviceDeleter> device_ = nullptr;
        bool open_ = false;
        bool valid_ = false;

        // CaptureCommand は状態を持つので作成->実行->破棄の流れにのせることはできない
        std::array<Buffer::TsStream, 4> stream_ {};
        std::array<Buffer::RingBuffer, 4> buffer_ {};
    };
}
