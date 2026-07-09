#pragma once

#include "buffer/page_buffer.h"
#include "buffer/ts_buffer.h"

namespace Earthsoft::Example::Buffer {
    class RingBuffer {
    public:
        RingBuffer() = default;

        RingBuffer(Pt3::Device *device)
            : tsBuffer_(device)
            , pageBuffer_(device) {
        }

        ~RingBuffer() {
            Free();
        }

        RingBuffer(const RingBuffer&) = delete;
        RingBuffer& operator=(const RingBuffer&) = delete;

        RingBuffer(RingBuffer&&) noexcept = default;
        RingBuffer& operator=(RingBuffer&&) noexcept = default;

        [[nodiscard]]
        const bool IsValid() const noexcept {
            return tsBuffer_.IsValid()
                && pageBuffer_.IsValid();
        }

        [[nodiscard]]
        const bool IsAllocated() const noexcept {
            return tsBuffer_.IsAllocated()
                && pageBuffer_.IsAllocated();
        }

        std::int32_t Allocate(std::uint32_t blockSize, std::uint32_t blockCount, bool loop = true) {
            std::int32_t status = 0;

            status = tsBuffer_.Allocate(blockSize, blockCount);
            if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                return status;
            }

            status = pageBuffer_.Allocate(blockSize, blockCount);
            if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                return status;
            }

            status = pageBuffer_.BuildPageDescriptor(tsBuffer_, loop);
            if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                return status;
            }

            status = pageBuffer_.SyncCpu();
            if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                return status;
            }

            return status;
        }

        std::int32_t Free() {
            std::int32_t status = 0;

            status = tsBuffer_.Free();
            if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                // return status;
            }

            status = pageBuffer_.Free();
            if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                // return status;
            }

            return status;
        }

        std::int32_t SyncCpu(std::uint32_t blockIndex) {
            return tsBuffer_.SyncCpu(blockIndex);
        }

        std::int32_t SyncIo(std::uint32_t blockIndex) {
            return tsBuffer_.SyncIo(blockIndex);
        }

        void *Ptr(std::uint32_t blockIndex) const noexcept {
            return tsBuffer_.Ptr(blockIndex);
        }

        std::uint64_t PageDescriptorAddress() const noexcept {
            return pageBuffer_.DescriptorAddress();
        }

    private:
        TsBuffer tsBuffer_ {};
        PageBuffer pageBuffer_ {};
    };
}
