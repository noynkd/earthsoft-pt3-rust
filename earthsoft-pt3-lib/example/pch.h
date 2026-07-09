#pragma once

// 事前コンパイルヘッダー
// - cpp ファイルの先頭でインクルードする
// - h/hpp ファイルではインクルードしないこと
// - ここで定義したヘッダーについては h/hpp ファイルでも参照できる

//
// C++ 標準ヘッダー
//
#include <array>
#include <charconv>
#include <chrono>
#include <cstddef>
#include <cstdint>
#include <expected>
#include <filesystem>
#include <format>
#include <fstream>
#include <iostream>
#include <memory>
#include <print>
#include <random>
#include <span>
#include <string>
#include <thread>
#include <tuple>
#include <type_traits>
#include <utility>
#include <vector>

//
// Windows SDK ヘッダー
//
#define WIN32_LEAN_AND_MEAN
#include <Windows.h>

//
// PT3 SDK ヘッダー
//
#include "../include/pt3.h"
