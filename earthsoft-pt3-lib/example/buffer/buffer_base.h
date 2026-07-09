#pragma once

#include "buffer/physical_block.h"
#include "os/memory_buffer.h"

namespace Earthsoft::Example::Buffer {
    class BufferBase {
    public:
        [[nodiscard]]
        const bool IsValid() const noexcept {
            return device_ != nullptr;
        }

        [[nodiscard]]
        const bool IsAllocated() const noexcept {
            return ptr_ != nullptr
                && size_ != 0
                && blockSize_ != 0
                && blockCount_ != 0
                && physicalBlocks_.size() != 0;
        }

    protected:
        BufferBase() = default;
        explicit BufferBase(Pt3::Device *device) noexcept : device_(device) {}

        virtual ~BufferBase() {
            Free();
        }

        BufferBase(const BufferBase&) = delete;
        BufferBase& operator=(const BufferBase&) = delete;

        BufferBase(BufferBase&& other) noexcept
            : device_(std::exchange(other.device_, nullptr))
            , ptr_(std::exchange(other.ptr_, nullptr))
            , buffer_(std::move(other.buffer_))
            , size_(std::exchange(other.size_, 0))
            , blockSize_(std::exchange(other.blockSize_, 0))
            , blockCount_(std::exchange(other.blockCount_, 0))
            , physicalBlocks_(std::move(other.physicalBlocks_)) {
        }

        BufferBase& operator=(BufferBase&& other) noexcept {
            if (this == &other) {
                return *this;
            }

            Free();

            device_ = std::exchange(other.device_, nullptr);
            ptr_ = std::exchange(other.ptr_, nullptr);
            buffer_ = std::move(other.buffer_);
            size_ = std::exchange(other.size_, 0);
            blockSize_ = std::exchange(other.blockSize_, 0);
            blockCount_ = std::exchange(other.blockCount_, 0);
            physicalBlocks_ = std::move(other.physicalBlocks_);

            return *this;
        }

        std::int32_t Allocate(Pt3::TransferDirection direction, uint32_t blockSize, std::uint32_t blockCount) {
            std::int32_t status = 0;

            std::uint32_t size = blockSize * blockCount;

            buffer_ = OS::MemoryBuffer(size);

            std::uint8_t *ptr = static_cast<std::uint8_t *>(buffer_.Ptr());

            ptr_        = ptr;
            size_       = size;
            blockSize_  = blockSize;
            blockCount_ = blockCount;

            if (ptr == nullptr) {
                return static_cast<std::int32_t>(Pt3::Status::OutOfMemory);
            }

            std::uint32_t offset = 0;
            physicalBlocks_.reserve(blockCount);
            for (std::uint32_t index = 0; index < blockCount; ++index) {
                physicalBlocks_.emplace_back(device_);

                std::int32_t status = physicalBlocks_.back().Lock(ptr + offset, blockSize, direction);
                if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                    physicalBlocks_.pop_back();
                    return status;
                }

                offset += blockSize;
            }

            return status;
        }

        std::int32_t Free() {
            physicalBlocks_.clear();

            buffer_ = OS::MemoryBuffer();
            ptr_ = nullptr;
            size_ = 0;
            blockSize_ = 0;
            blockCount_ = 0;

            return 0;
        }

        [[nodiscard]]
        void *Ptr() const noexcept {
            return ptr_;
        }

        [[nodiscard]]
        std::uint32_t Size() const noexcept {
            return size_;
        }

        [[nodiscard]]
        std::uint32_t BlockSize() const noexcept {
            return blockSize_;
        }

        [[nodiscard]]
        std::uint32_t BlockCount() const noexcept {
            return blockCount_;
        }

        [[nodiscard]]
        const std::vector<PhysicalBlock> &PhysicalBlocks() const noexcept {
            return physicalBlocks_;
        }

    private:
        Pt3::Device *device_ = nullptr;
        OS::MemoryBuffer buffer_;
        void *ptr_ = nullptr;
        std::uint32_t size_ = 0;
        std::uint32_t blockSize_ = 0;
        std::uint32_t blockCount_ = 0;
        std::vector<PhysicalBlock> physicalBlocks_ {};
    };
}
