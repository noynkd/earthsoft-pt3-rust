#pragma once

namespace Earthsoft::Example::OS {
    class MemoryBuffer {
    public:
        static inline constexpr std::size_t PageSize = 4 * 1024; 

        MemoryBuffer() = default;

        explicit MemoryBuffer(std::size_t size) {
            if (size == 0) {
                return;
            }

            ptr_.reset(::VirtualAlloc(nullptr, size, MEM_COMMIT, PAGE_READWRITE));
        }

        ~MemoryBuffer() = default;

        MemoryBuffer(const MemoryBuffer&) = delete;
        MemoryBuffer& operator=(const MemoryBuffer&) = delete;

        MemoryBuffer(MemoryBuffer&& other) noexcept : ptr_(std::move(other.ptr_)) {
        }

        MemoryBuffer& operator=(MemoryBuffer&& other) noexcept {
            if (this != &other) {
                ptr_ = std::move(other.ptr_);
            }
            return *this;
        }

        [[nodiscard]]
        void *Ptr() const noexcept {
            return ptr_.get();
        }

    private:
        struct PtrDeleter {
            void operator()(void *ptr) const noexcept {
                if (ptr == nullptr) {
                    return;
                }

                ::VirtualFree(ptr, 0, MEM_RELEASE);
            }
        };

        std::unique_ptr<void, PtrDeleter> ptr_ = nullptr;
    };
}
