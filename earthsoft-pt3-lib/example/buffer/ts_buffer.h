#pragma once

#include "buffer/buffer_base.h"
#include "utility/utility.h"

namespace Earthsoft::Example::Buffer {
    class TsBuffer : public BufferBase {
    public:
        TsBuffer() = default;

        explicit TsBuffer(Pt3::Device *device) : BufferBase(device) {
        }

        ~TsBuffer() {
            Free();
        }

        TsBuffer(const TsBuffer&) = delete;
        TsBuffer& operator=(const TsBuffer&) = delete;

        TsBuffer(TsBuffer&& other) = default;
        TsBuffer& operator=(TsBuffer&& other) = default;

        std::int32_t Allocate(std::uint32_t blockSize, std::uint32_t blockCount) {
            return BufferBase::Allocate(Pt3::TransferDirection::Write, blockSize, blockCount);
        }

        std::int32_t Free() {
            return BufferBase::Free();
        }

        std::int32_t SyncCpu(std::uint32_t blockIndex) {
            if (BufferBase::PhysicalBlocks().empty()) {
                Utility::PrintError("Earthsoft::Example::TsBuffer::SyncCpu(): メモリが割り当てられていません。");
                return static_cast<std::int32_t>(Pt3::Status::InternalError);
            }

            std::int32_t status = BufferBase::PhysicalBlocks().at(blockIndex).SyncCpu();
            if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                Utility::PrintError("Earthsoft::Pt3::Device::SyncCpu() に失敗しました。", status);
                return status;
            }

            return status;
        }

        std::int32_t SyncIo(std::uint32_t blockIndex) {
            if (BufferBase::PhysicalBlocks().empty()) {
                Utility::PrintError("Earthsoft::Example::TsBuffer::SyncIo(): メモリが割り当てられていません。");
                return static_cast<std::int32_t>(Pt3::Status::InternalError);
            }

            std::int32_t status = BufferBase::PhysicalBlocks().at(blockIndex).SyncIo();
            if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                Utility::PrintError("Earthsoft::Pt3::Device::SyncIo() に失敗しました。", status);
                return status;
            }

            return status;
        }

        void *Ptr(std::uint32_t blockIndex) const noexcept {
            if (BufferBase::Ptr() == nullptr || BufferBase::PhysicalBlocks().empty()) {
                Utility::PrintError("Earthsoft::Exampble::TsBuffer::Ptr(): メモリが割り当てられていません。");
                return nullptr;
            }

            if (blockIndex >= BufferBase::BlockCount()) {
                Utility::PrintError("Earthsoft::Exampble::TsBuffer::Ptr(): インデックスが範囲外です");
                return nullptr;
            }

            std::uint8_t *ptr = static_cast<std::uint8_t *>(BufferBase::Ptr());
            ptr += static_cast<std::size_t>(BufferBase::BlockSize()) * blockIndex; 

            return ptr; 
        }

        [[nodiscard]]
        const std::vector<PhysicalBlock> &PhysicalBlocks() const noexcept {
            return BufferBase::PhysicalBlocks();
        }
    };
}
