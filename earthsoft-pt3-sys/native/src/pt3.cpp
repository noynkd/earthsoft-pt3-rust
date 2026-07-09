#include "internal.h"

::HMODULE Pt3Module = nullptr;
Earthsoft::Pt3::NewBusFunction Pt3BusFactory = nullptr;

static const std::filesystem::path Pt3SdkDllName = L"SDK_EARTHSOFT_PT3.dll";

#ifdef  __cplusplus
extern "C" {
#endif

int32_t LoadPt3Lib(void) {
    if (Pt3Module != nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::Ok);
    }

    ::HMODULE module = ::LoadLibraryW(Pt3SdkDllName.c_str());
    if (module == nullptr) {
        // SdkDllLoadError
        return static_cast<int32_t>(0x202);
    }

    Earthsoft::Pt3::NewBusFunction busFactory = reinterpret_cast<Earthsoft::Pt3::NewBusFunction>(::GetProcAddress(module, "_"));
    if (busFactory == nullptr) {
        ::FreeLibrary(module);
        module = nullptr;

        // BusFactoryNotFoundError
        return static_cast<int32_t>(0x203);
    }

    Pt3Module = module;
    Pt3BusFactory = busFactory;

    return static_cast<int32_t>(Earthsoft::Pt3::Status::Ok);
}

int32_t FreePt3Lib(void)
{
    if (Pt3Module == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::Ok);
    }

    Pt3BusFactory = nullptr;

    ::FreeLibrary(Pt3Module);
    Pt3Module = nullptr;

    return static_cast<int32_t>(Earthsoft::Pt3::Status::Ok);
}

int32_t CreatePt3Bus(Pt3Bus **bus)
{
    if (bus == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InvalidParameter);
    }
    if (Pt3BusFactory == nullptr) {
        // BusFactoryNotFoundError
        return static_cast<int32_t>(0x203);
    }

    Earthsoft::Pt3::Bus *rawBus = nullptr;

    std::int32_t status = Pt3BusFactory(&rawBus);
    if (status != 0) {
        return static_cast<int32_t>(status);
    }
    if (rawBus == nullptr) {
        return static_cast<int32_t>(Earthsoft::Pt3::Status::InternalError);
    }

    Pt3Bus *newBus = new (std::nothrow) Pt3Bus {
        .impl = rawBus,
    };
    if (newBus == nullptr) {
        rawBus->Delete();
        rawBus = nullptr;
        return static_cast<int32_t>(Earthsoft::Pt3::Status::OutOfMemory);
    }
    *bus = newBus;

    return static_cast<int32_t>(Earthsoft::Pt3::Status::Ok);
}

#ifdef  __cplusplus
}
#endif
