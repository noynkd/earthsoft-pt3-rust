#pragma once

namespace Earthsoft::Example::Utility {
    [[nodiscard]]
    inline void PrintError(std::string_view message) {
        std::println("[ERROR] {}", message);
    }

    [[nodiscard]]
    inline void PrintError(std::string_view message, std::int32_t status) {
        std::println("[ERROR] {} (status: {:#010x})", message, static_cast<std::uint32_t>(status));
    }

    [[nodiscard]]
    inline std::uint32_t GetNumber(std::uint32_t max) {
        std::string line;
        std::uint32_t number = 0;

        while(true) {
            if (!std::getline(std::cin, line)) {
                std::cin.clear();
                continue;
            }
            if (line.empty()) {
                std::print(">");
                continue;
            }

            auto [ptr, errc] = std::from_chars(line.data(), line.data() + line.size(), number);

            if (errc == std::errc {} && ptr == line.data() + line.size() && number <= max) {
                std::println("{}", number);
                break;
            }

            std::println("[WARNING] 0 から {} の範囲の正しい数値を入力してください。", max);
            std::print(">");
        }

        return number;
    }

    [[nodiscard]]
    inline std::uint32_t GetHexNumber(std::uint32_t max) {
        std::string line;
        std::uint32_t number = 0;

        while(true) {
            if (!std::getline(std::cin, line)) {
                std::cin.clear();
                continue;
            }
            if (line.empty()) {
                std::print(">0x");
                continue;
            }

            auto [ptr, errc] = std::from_chars(line.data(), line.data() + line.size(), number, 16);

            if (errc == std::errc {} && ptr == line.data() + line.size() && number <= max) {
                std::println("{}", number);
                break;
            }

            std::println("[WARNING] 0000 から {:04x} の範囲の正しい数値を入力してください。", max);
            std::print(">0x");
        }

        return number;
    }
}
