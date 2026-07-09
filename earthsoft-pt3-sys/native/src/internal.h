#pragma once

//
// 外部公開ヘッダー
//
#include "../include/pt3.h"

//
// PT3 SDK ヘッダー
//
#include "../../earthsoft-pt3-lib/include/pt3.h"

//
// Windows SDK ヘッダー
//
#define WIN32_LEAN_AND_MEAN
#include <Windows.h>

//
// C++ 標準ヘッダー
//
#include <cstdint>
#include <filesystem>

//
// 実装側で定義されるグローバル変数
//
extern ::HMODULE Pt3Module;
extern Earthsoft::Pt3::NewBusFunction Pt3BusFactory;

//
// プライベートで使用する共通関数
//
#ifdef  __cplusplus
extern "C" {
#endif

struct Pt3Bus {
    Earthsoft::Pt3::Bus *impl;
};

struct Pt3Device {
    Earthsoft::Pt3::Device *impl;
};

#ifdef  __cplusplus
}
#endif

//
// メモリレイアウト検査
//
// Earthsoft::Pt3::Isdb
static_assert(sizeof(Earthsoft::Pt3::Isdb) == sizeof(uint32_t));
static_assert(alignof(Earthsoft::Pt3::Isdb) == alignof(uint32_t));
// Earthsoft::Pt3::LnbPower
static_assert(sizeof(Earthsoft::Pt3::LnbPower) == sizeof(uint32_t));
static_assert(alignof(Earthsoft::Pt3::LnbPower) == alignof(uint32_t));
// Earthsoft::Pt3::RamPinsMode
static_assert(sizeof(Earthsoft::Pt3::RamPinsMode) == sizeof(uint32_t));
static_assert(alignof(Earthsoft::Pt3::RamPinsMode) == alignof(uint32_t));
// Earthsoft::Pt3::Satellite::LayerIndex
static_assert(sizeof(Earthsoft::Pt3::Satellite::LayerIndex) == sizeof(uint32_t));
static_assert(alignof(Earthsoft::Pt3::Satellite::LayerIndex) == alignof(uint32_t));
// Earthsoft::Pt3::Satellite::LayerMask
static_assert(sizeof(Earthsoft::Pt3::Satellite::LayerMask) == sizeof(uint32_t));
static_assert(alignof(Earthsoft::Pt3::Satellite::LayerMask) == alignof(uint32_t));
// Earthsoft::Pt3::Terrestrial::LayerIndex
static_assert(sizeof(Earthsoft::Pt3::Terrestrial::LayerIndex) == sizeof(uint32_t));
static_assert(alignof(Earthsoft::Pt3::Terrestrial::LayerIndex) == alignof(uint32_t));
// Earthsoft::Pt3::Terrestrial::LayerMask
static_assert(sizeof(Earthsoft::Pt3::Terrestrial::LayerMask) == sizeof(uint32_t));
static_assert(alignof(Earthsoft::Pt3::Terrestrial::LayerMask) == alignof(uint32_t));
// Earthsoft::Pt3::TransferDirection
static_assert(sizeof(Earthsoft::Pt3::TransferDirection) == sizeof(uint32_t));
static_assert(alignof(Earthsoft::Pt3::TransferDirection) == alignof(uint32_t));
// Earthsoft::Pt3::TsPinMode
static_assert(sizeof(Earthsoft::Pt3::TsPinMode) == sizeof(uint32_t));
static_assert(alignof(Earthsoft::Pt3::TsPinMode) == alignof(uint32_t));
// Earthsoft::Pt3::BufferInfo
static_assert(sizeof(Earthsoft::Pt3::BufferInfo) == sizeof(Pt3BufferInfo));
static_assert(alignof(Earthsoft::Pt3::BufferInfo) == alignof(Pt3BufferInfo));
// Earthsoft::Pt3::ConstantInfo
static_assert(sizeof(Earthsoft::Pt3::ConstantInfo) == sizeof(Pt3ConstantInfo));
static_assert(alignof(Earthsoft::Pt3::ConstantInfo) == alignof(Pt3ConstantInfo));
// Earthsoft::Pt3::DeviceInfo
static_assert(sizeof(Earthsoft::Pt3::DeviceInfo) == sizeof(Pt3DeviceInfo));
static_assert(alignof(Earthsoft::Pt3::DeviceInfo) == alignof(Pt3DeviceInfo));
// Earthsoft::Pt3::ErrorRate
static_assert(sizeof(Earthsoft::Pt3::ErrorRate) == sizeof(Pt3ErrorRate));
static_assert(alignof(Earthsoft::Pt3::ErrorRate) == alignof(Pt3ErrorRate));
// Earthsoft::Pt3::Satellite::Tmcc
static_assert(sizeof(Earthsoft::Pt3::Satellite::Tmcc) == sizeof(Pt3SatelliteTmcc));
static_assert(alignof(Earthsoft::Pt3::Satellite::Tmcc) == alignof(Pt3SatelliteTmcc));
// Earthsoft::Pt3::Satellite::Layer
static_assert(sizeof(Earthsoft::Pt3::Satellite::Layer) == sizeof(Pt3SatelliteLayer));
static_assert(alignof(Earthsoft::Pt3::Satellite::Layer) == alignof(Pt3SatelliteLayer));
// Earthsoft::Pt3::Terrestrial::Tmcc
static_assert(sizeof(Earthsoft::Pt3::Terrestrial::Tmcc) == sizeof(Pt3TerrestrialTmcc));
static_assert(alignof(Earthsoft::Pt3::Terrestrial::Tmcc) == alignof(Pt3TerrestrialTmcc));
// Earthsoft::Pt3::TsPinsLevel
static_assert(sizeof(Earthsoft::Pt3::TsPinsLevel) == sizeof(Pt3TsPinsLevel));
static_assert(alignof(Earthsoft::Pt3::TsPinsLevel) == alignof(Pt3TsPinsLevel));
// Earthsoft::Pt3::TsPinsMode
static_assert(sizeof(Earthsoft::Pt3::TsPinsMode) == sizeof(Pt3TsPinsMode));
static_assert(alignof(Earthsoft::Pt3::TsPinsMode) == alignof(Pt3TsPinsMode));
// Earthsoft::Pt3::TransferInfo
static_assert(sizeof(Earthsoft::Pt3::TransferInfo) == sizeof(Pt3TransferInfo));
static_assert(alignof(Earthsoft::Pt3::TransferInfo) == alignof(Pt3TransferInfo));
