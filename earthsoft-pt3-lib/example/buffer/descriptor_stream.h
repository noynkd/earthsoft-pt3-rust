#pragma once

namespace Earthsoft::Example::Buffer {
    class DescriptorStream {
    public:
        #pragma pack(push, 1)
        struct alignas(1) Descriptor {
        private:
            std::uint64_t tsAddress_;
            std::uint32_t tsPageSize_;
            std::uint64_t nextPageAddress_;

        public:
            void Write(std::uint64_t address, std::uint32_t size) noexcept {
                tsAddress_       = address | 7ULL;
                tsPageSize_      = size    | 7ULL;
                nextPageAddress_ = 0       | 2ULL;
            }

            void Link(std::uint64_t next) noexcept {
                nextPageAddress_ = next    | 2ULL;
            }
        };
        #pragma pack(pop)

        explicit DescriptorStream(
            std::span<std::uint8_t> buffer,
            std::span<const Pt3::BufferInfo> pages) noexcept
            : pages_(pages)
            , current_(buffer.data())
            , size_(buffer.size())
            , address_(pages.empty() ? 0 : pages.front().Address)
            , remain_(pages.empty() ? 0 : pages.front().Size)
            , firstAddress_(pages.empty() ? 0 : pages.front().Address) {
            if (buffer.empty() || pages.empty()) {
                valid_ = false;
                return;
            }

            std::size_t totalSize = 0;
            for (const Pt3::BufferInfo &page : pages) {
                totalSize += page.Size;
            }

            // 仮想バッファが物理バッファに対して十分なサイズを持っていない
            if (size_ < totalSize) {
                valid_ = false;
            }
        }

        ~DescriptorStream() = default;

        DescriptorStream(const DescriptorStream&) = delete;
        DescriptorStream& operator=(const DescriptorStream&) = delete;

        DescriptorStream(DescriptorStream&&) noexcept = default;
        DescriptorStream& operator=(DescriptorStream&&) noexcept = default;


        [[nodiscard]]
        bool IsValid() const noexcept {
            return valid_ && !end_;
        }

        bool Write(std::uint64_t tsAddress, std::uint32_t tsSize) {
            if (!IsValid()) {
                return false;
            }

            // 残りバッファが記述子に満たない場合は次の記述子ページに移動する
            while (remain_ < sizeof(Descriptor)) {
                current_ += remain_;    // スキップして仮想バッファのページサイズを埋める
                
                pageIndex_++;
                // 物理バッファの次のページがなかった場合、書き込むものがなくなるので終了する
                if (pageIndex_ >= pages_.size()) {
                    end_ = true;
                    return false;
                }

                // 次の BufferInfo の先頭に移動する
                address_ = pages_[pageIndex_].Address;
                remain_  = pages_[pageIndex_].Size;
            }

            Descriptor *current = reinterpret_cast<Descriptor *>(current_);

            if (previous_ != nullptr) {
                previous_->Link(address_);
            }

            // 書き込み後のサイズが仮想バッファのサイズを超える場合、容量オーバーのため終了する
            if (size_ < written_ + sizeof(Descriptor)) {
                end_ = true;
                return false;
            }
            current->Write(tsAddress, tsSize);
            written_ += sizeof(Descriptor);

            previous_ = current;

            current_ += sizeof(Descriptor);
            address_ += sizeof(Descriptor);
            remain_  -= sizeof(Descriptor);

            return true;
        }

        void Finalize(bool loop) {
            if (previous_ == nullptr) {
                return;
            }

            if (loop) {
                previous_->Link(firstAddress_); // 最初のアドレスを設定する
            } else {
                previous_->Link(1ULL);          // 末尾符号(1)
            }
        }

    private:
        std::span<const Pt3::BufferInfo> pages_;
        std::size_t pageIndex_ = 0;

        std::uint8_t *current_ = nullptr;
        std::uint64_t address_ = 0;
        std::uint32_t remain_  = 0;

        std::size_t   size_    = 0;
        std::size_t   written_ = 0;
        std::uint64_t firstAddress_ = 0;
        Descriptor   *previous_ = nullptr;

        bool valid_ = true;
        bool end_   = false;
    };
}
