#pragma once

#include "utility/utility.h"

namespace Earthsoft::Example::Buffer {
    class PhysicalBlock {
    public:
        PhysicalBlock() noexcept = default;
        explicit PhysicalBlock(Pt3::Device *device) noexcept : device_(device) {}

        ~PhysicalBlock() {
            Unlock();
        };

        PhysicalBlock(const PhysicalBlock&) = delete;
        PhysicalBlock& operator=(const PhysicalBlock&) = delete;

        PhysicalBlock(PhysicalBlock&& other) noexcept
            : device_(std::exchange(other.device_, nullptr))
            , handle_(std::exchange(other.handle_, nullptr))
            , bufferInfo_(std::exchange(other.bufferInfo_, nullptr))
            , count_(std::exchange(other.count_, 0)) {
        }

        PhysicalBlock& operator=(PhysicalBlock&& other) noexcept {
            if (this == &other) {
                return *this;
            }

            Unlock();

            device_ = std::exchange(other.device_, nullptr);
            handle_ = std::exchange(other.handle_, nullptr);
            bufferInfo_ = std::exchange(other.bufferInfo_, nullptr);
            count_ = std::exchange(other.count_, 0);

            return *this;
        };

        std::int32_t Lock(void *ptr, std::uint32_t size, Pt3::TransferDirection direction) {
            if (device_ == nullptr) {
                Utility::PrintError("Earthsoft::Pt3::Device が nullptr です。");
                return static_cast<std::int32_t>(Pt3::Status::InternalError);
            }
            if (handle_ != nullptr) {
                Utility::PrintError("PhysicalBlock はすでにロック済みです。");
                return static_cast<std::int32_t>(Pt3::Status::InternalError);
            }

            std::int32_t status = 0;

            status = device_->LockBuffer(ptr, size, direction, &handle_);
            if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                Utility::PrintError("Earthsoft::Pt3::Device::LockBuffer() に失敗しました。", status);
                return status;
            }
            if (handle_ == nullptr) {
                Utility::PrintError("PhysicalBlock がロックされていません。");
                return static_cast<std::int32_t>(Pt3::Status::InternalError);
            }

            status = device_->GetBufferInfo(handle_, &bufferInfo_, &count_);
            if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                Utility::PrintError("Earthsoft::Pt3::Device::GetBufferInfo() に失敗しました。", status);
                return status;
            }
            if (bufferInfo_ == nullptr) {
                Utility::PrintError("PhysicalBlock のバッファが nullptr です。");
                return static_cast<std::int32_t>(Pt3::Status::InternalError);
            }

            return status;
        }

        std::int32_t Unlock() {
            if (device_ == nullptr) {
                Utility::PrintError("Earthsoft::Pt3::Device が nullptr です。");
                return static_cast<std::int32_t>(Pt3::Status::InternalError);
            }
            if (handle_ == nullptr) {
                // Utility::PrintError("PhysicalBlock のハンドルが nullptr です。");
                // return static_cast<std::int32_t>(Pt3::Status::InternalError);

                // そもそも未ロックなので空振りにする
                return static_cast<std::int32_t>(Pt3::Status::Ok);
            }

            std::int32_t status = 0;

            status = device_->UnlockBuffer(handle_);
            if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                Utility::PrintError("Earthsoft::Pt3::Device::UnlockBuffer() に失敗しました。", status);

                // リソースクリアまでしてしまう
                // return status;
            }

            handle_ = nullptr;
            bufferInfo_ = nullptr;
            count_ = 0;

            return status;
        }

        std::int32_t SyncCpu() const {
            if (device_ == nullptr) {
                Utility::PrintError("Earthsoft::Pt3::Device が nullptr です。");
                return static_cast<std::int32_t>(Pt3::Status::InternalError);
            }
            if (handle_ == nullptr) {
                Utility::PrintError("PhysicalBlock がロックされていません。");
                return static_cast<std::int32_t>(Pt3::Status::InternalError);
            }

            std::int32_t status = 0;

            status = device_->SyncBufferCpu(handle_);
            if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                Utility::PrintError("Earthsoft::Pt3::Device::SyncBufferCpu() に失敗しました。", status);
                return status;
            }

            return status;
        }

        std::int32_t SyncIo() const {
            if (device_ == nullptr) {
                Utility::PrintError("Earthsoft::Pt3::Device が nullptr です。");
                return static_cast<std::int32_t>(Pt3::Status::InternalError);
            }
            if (handle_ == nullptr) {
                Utility::PrintError("PhysicalBlock がロックされていません。");
                return static_cast<std::int32_t>(Pt3::Status::InternalError);
            }

            std::int32_t status = 0;

            status = device_->SyncBufferIo(handle_);
            if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                Utility::PrintError("Earthsoft::Pt3::Device::SyncBufferIo() に失敗しました。", status);
                return status;
            }

            return status;
        }

        std::span<const Pt3::BufferInfo> BufferInfos() const noexcept {
            if (bufferInfo_ == nullptr || count_ == 0) {
                return {};
            }

            return { bufferInfo_, count_ };
        }

    private:
        Pt3::Device *device_ = nullptr;
        void* handle_ = nullptr;
        const Pt3::BufferInfo *bufferInfo_ = nullptr;
        std::uint32_t count_ = 0;
    };
}
