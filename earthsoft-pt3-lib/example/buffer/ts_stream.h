#pragma once

#include "buffer/ring_buffer.h"
#include "utility/utility.h"

namespace Earthsoft::Example::Buffer {
    class TsStream {
    public:
        TsStream() = default;
        explicit TsStream(Pt3::Device *device, Pt3::Isdb isdb, std::uint32_t tuner)
            : device_(device)
            , isdb_(isdb)
            , tuner_(tuner) {

            const std::array<char, 2> isdbCapital { 'S', 'T' };
            
            // チューナー名 (TSファイル名)
            name_ = std::format("ISDB-{}{}", isdbCapital[static_cast<std::size_t>(isdb)], tuner + 1);
        }

        ~TsStream() = default;

        TsStream(const TsStream&) = delete;
        TsStream& operator=(const TsStream&) = delete;

        TsStream(TsStream&& other) noexcept
            : device_(std::exchange(other.device_, nullptr))
            , isdb_(std::exchange(other.isdb_, {}))
            , tuner_(std::exchange(other.tuner_, 0))
            , name_(std::move(other.name_))
            , worker_(std::move(other.worker_))
            , buffer_(std::move(other.buffer_))
            , blockIndex_(std::exchange(other.blockIndex_, 0)) {
            if (other.file_.is_open()) {
                file_.swap(other.file_);
            }
        }

        TsStream& operator=(TsStream&& other) noexcept {
            if (this == &other) {
                return *this;
            }

            Stop();
            if (file_.is_open()) {
                file_.close();
            }

            device_ = std::exchange(other.device_, nullptr);
            isdb_   = std::exchange(other.isdb_, {});
            tuner_  = std::exchange(other.tuner_, 0);
            name_   = std::move(other.name_);
            worker_ = std::move(other.worker_);
            buffer_ = std::move(other.buffer_);
            blockIndex_  = std::exchange(other.blockIndex_, 0);

            if (other.file_.is_open()) {
                file_.swap(other.file_);
            }

            return *this;
        }

        const bool IsValid() const noexcept {
            if (device_ == nullptr) {
                return false;
            }

            return true;
        }

        void Run() {
            Stop();

            worker_ = std::jthread([this](std::stop_token token) {
                std::int32_t status = 0;

                status = Begin();
                if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                    return;
                }

                while (!token.stop_requested()) {
                    status = Process();
                    if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                        return;
                    }
                }

                status = End();
                if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                    return;
                }
            });
        }

        void Stop() {
            if (!worker_.joinable()) {
                return;
            }

            worker_.request_stop();
            worker_.join();
        }

    private:
        std::int32_t Begin() {
            // 出力先のファイルを作る
            const std::string filename = std::format("{}.ts", name_);
            file_.open(filename, std::ios::out | std::ios::binary);
            if (!file_.is_open()) {
                Utility::PrintError("ファイルを開けませんでした。");
                return 0x800;
            }

            // リングバッファを作る
            if (!buffer_.IsValid()) {
                buffer_ = std::move(RingBuffer(device_));
            }
            if (!buffer_.IsAllocated()) {
                std::int32_t status = buffer_.Allocate(blockSize_, blockCount_);
                if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                    Utility::PrintError("Earthsoft::Example::Buffer::Allocate() に失敗しました。", status);
                    return status;
                }
            }

            _ASSERT(buffer_.Ptr(0));

            // 同期バイトにダミーデータを書き込む
            std::span<std::uint8_t>bytes(static_cast<std::uint8_t *>(buffer_.Ptr(0)), bufferSize_);

            for (std::uint32_t index = 0; index < blockCount_; ++index) {
                bytes[index * blockSize_] = notSyncByte_;

                std::int32_t status = buffer_.SyncCpu(index);
                if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                    Utility::PrintError("Earthsoft::Example::Buffer::SyncCpu() に失敗しました。", status);
                    return status;
                }
            }

            // スレッドループ内で使用する変数を初期化する
            blockIndex_ = 0;

            // 送信開始
            std::int32_t status = 0;

            status = device_->SetTransferTestMode(isdb_, tuner_);
            if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                Utility::PrintError("Earthsoft::Pt3::Device::SetTransferTestMode() に失敗しました。", status);
                return status;
            }

            std::uint64_t pageAddress = buffer_.PageDescriptorAddress();

            status = device_->SetTransferPageDescriptorAddress(isdb_, tuner_, pageAddress);
            if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                Utility::PrintError("Earthsoft::Pt3::Device::SetTransferPageDescriptorAddress() に失敗しました。", status);
                return status;
            }

            status = device_->SetTransferEnabled(isdb_, tuner_, true);
            if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                Utility::PrintError("Earthsoft::Pt3::Device::SetTransferEnabled() に失敗しました。", status);
                return status;
            }

            return status;
        }

        std::int32_t Process() {
            using namespace std::chrono_literals;
            std::this_thread::sleep_for(1ms);

            std::uint32_t next = blockIndex_ + 1;
            if (blockCount_ <= next) {
                next = 0;
            }

            if (CheckReady(next)) {
                {
                    static thread_local bool first = true;
                    if (first) {
                        std::println("{}.ts の書き出しを開始します。", name_);
                        first = false;
                    }
                }

                std::int32_t status = Write(blockIndex_);
                if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                    return status;
                }

                blockIndex_++;
                if (blockCount_ <= blockIndex_) {
                    blockIndex_ = 0;
                }
            }

            return static_cast<std::int32_t>(Pt3::Status::Ok);
        }

        std::int32_t End() {
            std::int32_t status = 0;

            Pt3::TransferInfo transferInfo;
            status = device_->GetTransferInfo(isdb_, tuner_, &transferInfo);
            if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                Utility::PrintError("Earthsoft::Pt3::Device::GetTransferInfo() に失敗しました。", status);
                return status;
            }

            if (transferInfo.InternalFifoAOverflow) {
                std::println("[{}][Internal FIFO A Overflow]", name_);
            }
            if (transferInfo.InternalFifoAUnderflow) {
                std::println("[{}][Internal FIFO A Underflow]", name_);
            }
            if (transferInfo.ExternalFifoOverflow) {
                std::println("[{}][External FIFO Overflow]", name_);
            } else {
                std::println("[{}][External FIFO Max Usage: {} bytes]", name_, transferInfo.ExternalFifoMaxUsedBytes);
            }
            if (transferInfo.InternalFifoBOverflow) {
                std::println("[{}][Internal FIFO B Overflow]", name_);
            }
            if (transferInfo.InternalFifoBUnderflow) {
                std::println("[{}][Internal FIFO B Underflow]", name_);
            }

            status = device_->SetTransferEnabled(isdb_, tuner_, false);
            if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                Utility::PrintError("Earthsoft::Pt3::Device::SetTransferEnabled() に失敗しました。", status);
                return status;
            }

            // ファイルを閉じる
            if (file_.is_open()) {
                file_.close();
            }

            return status;
        }

        bool CheckReady(std::uint32_t blockIndex) {
            std::int32_t status = buffer_.SyncCpu(blockIndex);
            if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                Utility::PrintError("Earthsoft::Example::Buffer::SyncCpu() に失敗しました。", status);
                return false;
            }

            std::uint8_t *ptr = static_cast<std::uint8_t *>(buffer_.Ptr(blockIndex));
            std::uint8_t syncData = ptr[0];

            switch (syncData) {
                case syncByte_:
                    return true;
                case notSyncByte_:
                    return false;
                default:
                    {
                        static thread_local bool first = true;
                        if (first) {
                            std::println("同期バイトの値({:#04x})が同期({:#04x})でも初期値({:#04x})でもありません。:{}", syncData, syncByte_, notSyncByte_, name_);
                            first = false;
                        }
                    }
                    return false;
            }
        }

        std::int32_t Write(std::uint32_t blockIndex) {
            std::int32_t status = 0;

            status = buffer_.SyncIo(blockIndex);
            if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                Utility::PrintError("Earthsoft::Example::Buffer::SyncIo() に失敗しました。", status);
                return status;
            }

            std::span<std::uint8_t> bytes(static_cast<std::uint8_t *>(buffer_.Ptr(blockIndex)), blockSize_);

            // ファイルに書き出す
            file_.write(reinterpret_cast<const char *>(bytes.data()), bytes.size());
            if (!file_) {
                Utility::PrintError("ファイルの書き込みに失敗しました。");
                return 0x801;
            }

            bytes[0] = notSyncByte_;    // 同期バイトに初期値を設定する

            status = buffer_.SyncCpu(blockIndex);
            if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                Utility::PrintError("Earthsoft::Example::Buffer::SyncCpu() に失敗しました。", status);
                return status;
            }

            return status;
        }

        Pt3::Device  *device_ = nullptr;
        Pt3::Isdb     isdb_ {};
        std::uint32_t tuner_ = 0;
        std::string   name_ {};

        std::jthread worker_ {};
        RingBuffer buffer_ {};
        std::uint32_t blockIndex_ = 0;
        std::ofstream file_ {};

        static constexpr std::size_t  blockSize_   =  OS::MemoryBuffer::PageSize * 47 * 8;
        static constexpr std::size_t  blockCount_  = 32;
        static constexpr std::size_t  bufferSize_  = blockSize_ * blockCount_;
        static constexpr std::uint8_t syncByte_    = static_cast<std::uint8_t>(0x47);
        static constexpr std::uint8_t notSyncByte_ = static_cast<std::uint8_t>(~syncByte_);
    };
}
