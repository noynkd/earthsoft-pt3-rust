#pragma once

namespace Earthsoft::Example::OS {
    class Library {
    public:
        Library() {
            module_.reset(::LoadLibraryW(path_.c_str()));
            if (module_ == nullptr) {
                return;
            }

            auto *functionPtr = ::GetProcAddress(module_.get(), "_");
            if (functionPtr == nullptr) {
                return;
            }

            busFactory_ = reinterpret_cast<Pt3::NewBusFunction>(functionPtr);
        }

        ~Library() noexcept = default;

        Library(const Library&) = delete;
        Library& operator=(const Library&) = delete;

        Library(Library&&) noexcept = default;
        Library& operator=(Library&&) noexcept = default;

        [[nodiscard]]
        const std::filesystem::path& Path() const noexcept {
            return path_;
        }

        [[nodiscard]]
        Pt3::NewBusFunction BusFactory() const noexcept {
            return busFactory_;
        }

        [[nodiscard]]
        bool IsValid() const noexcept {
            return (module_ != nullptr && busFactory_ != nullptr);
        }

        [[nodiscard]]
        explicit operator bool() const noexcept {
            return IsValid();
        }

    private:
        struct ModuleHandleDeleter {
            void operator()(::HMODULE module) noexcept {
                ::FreeLibrary(module);
            }
        };

        std::filesystem::path path_ = L"SDK_EARTHSOFT_PT3.dll";
        std::unique_ptr<std::remove_pointer_t<::HMODULE>, ModuleHandleDeleter> module_ = nullptr;
        Pt3::NewBusFunction busFactory_ = nullptr;
    };
}
