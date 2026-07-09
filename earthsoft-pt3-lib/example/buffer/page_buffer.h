#pragma once

#include "buffer/buffer_base.h"
#include "buffer/descriptor_stream.h"
#include "buffer/ts_buffer.h"
#include "utility/utility.h"

namespace Earthsoft::Example::Buffer {
    class PageBuffer : public BufferBase {
    public:
        PageBuffer() = default;

        explicit PageBuffer(Pt3::Device *device) : BufferBase(device) {
        }

        ~PageBuffer() {
            Free();
        }

        PageBuffer(const PageBuffer&) = delete;
        PageBuffer& operator=(const PageBuffer&) = delete;

        PageBuffer(PageBuffer&& other) = default;
        PageBuffer& operator=(PageBuffer&& other) = default;

        std::int32_t Allocate(std::uint32_t blockSize, std::uint32_t blockCount) {
            constexpr std::uint32_t descriptorsPerPage = OS::MemoryBuffer::PageSize / sizeof(DescriptorStream::Descriptor);
            const std::uint32_t totalTsPages = (blockSize * blockCount) / OS::MemoryBuffer::PageSize;
            const std::uint32_t requiredPageCount = (totalTsPages + descriptorsPerPage - 1) / descriptorsPerPage;
            const std::uint32_t requiredPageSize  = requiredPageCount * OS::MemoryBuffer::PageSize;

            return BufferBase::Allocate(Pt3::TransferDirection::Read, requiredPageSize, 1);
        }

        std::int32_t Free() {
            return BufferBase::Free();
        }

		[[nodiscard]]
        std::uint64_t DescriptorAddress() const noexcept
		{
            if (BufferBase::PhysicalBlocks().empty()) {
                return 0;
            }

            std::span<const Pt3::BufferInfo> bufferInfos = BufferBase::PhysicalBlocks().front().BufferInfos();

            if (bufferInfos.empty()) {
                return 0;
            }

            std::uint64_t address = bufferInfos.front().Address;

            return address;
		}

        std::int32_t BuildPageDescriptor(TsBuffer &tsBuffer, bool loop) {
            if (BufferBase::Ptr() == nullptr || BufferBase::Size() == 0 || BufferBase::PhysicalBlocks().empty()) {
                Utility::PrintError("Earthsoft::Example::PageBuffer::BuildPageDescriptor(): メモリが割り当てられていません。");
                return static_cast<std::int32_t>(Pt3::Status::InternalError);
            }

            std::span<std::uint8_t> memoryBuffer(static_cast<uint8_t *>(BufferBase::Ptr()), BufferBase::Size());
            if (memoryBuffer.empty()) {
                Utility::PrintError("Earthsoft::Example::PageBuffer::BuildPageDescriptor(): 仮想メモリバッファがありません。");
                return static_cast<std::int32_t>(Pt3::Status::InternalError);
            }

            std::span<const Pt3::BufferInfo> pages = BufferBase::PhysicalBlocks().front().BufferInfos();
            if (pages.empty()) {
                Utility::PrintError("Earthsoft::Example::PageBuffer::BuildPageDescriptor(): 物理バッファ情報がありません。");
                return static_cast<std::int32_t>(Pt3::Status::InternalError);
            }

            DescriptorStream stream(memoryBuffer, pages);
            if (!stream.IsValid()) {
                Utility::PrintError("Earthsoft::Example::PageBuffer::BuildPageDescriptor(): 仮想メモリバッファサイズが足りません。");
                return static_cast<std::int32_t>(Pt3::Status::OutOfMemory);
            }


            for (const PhysicalBlock &tsBlock : tsBuffer.PhysicalBlocks()) {
                for (const Pt3::BufferInfo &tsBufferInfo : tsBlock.BufferInfos()) {
                    const std::uint32_t totalPages = tsBufferInfo.Size / OS::MemoryBuffer::PageSize;
                    std::uint64_t currentBlockAddress = tsBufferInfo.Address;

                    for (std::uint32_t pageIndex = 0; pageIndex < totalPages; ++pageIndex) {
                        if (!stream.Write(currentBlockAddress, OS::MemoryBuffer::PageSize)) {
                            Utility::PrintError("Earthsoft::Example::PageBuffer::BuildPageDescriptor(): 物理バッファ情報が足りないか、仮想メモリバッファを使い切りました。");
                            return static_cast<std::int32_t>(Pt3::Status::OutOfMemory);
                        }

                        currentBlockAddress += OS::MemoryBuffer::PageSize;
                    }
                }
            }

            stream.Finalize(loop);

            return static_cast<std::int32_t>(Pt3::Status::Ok);
        }

        std::int32_t SyncCpu() {
            if (BufferBase::PhysicalBlocks().empty()) {
                Utility::PrintError("Earthsoft::Example::PageBuffer::SyncCpu(): メモリが割り当てられていません。");
                return static_cast<std::int32_t>(Pt3::Status::InternalError);
            }

            std::int32_t status = BufferBase::PhysicalBlocks().front().SyncCpu();
            if (status != static_cast<std::int32_t>(Pt3::Status::Ok)) {
                Utility::PrintError("Earthsoft::Pt3::Device::SyncCpu() に失敗しました。", status);
                return status;
            }

            return status;
        }
    };
}
