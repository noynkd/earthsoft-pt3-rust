#pragma once

#include "menu/bus_menu.h"
#include "os/library.h"
#include <timeapi.h>

#pragma comment(lib, "winmm.lib")

namespace Earthsoft::Example {
    class Main {
    public:
        Main() {
            ::timeBeginPeriod(1);
        };

        ~Main() noexcept {
            ::timeEndPeriod(1);
        };
    
        Main(const Main&) = delete;
        Main& operator=(const Main&) = delete;

        Main(Main&&) noexcept = default;
        Main& operator=(Main&&) noexcept = default;

        void Run() {
            std::println("Example.exe for PT3 バージョン 4.0.0");
            
            OS::Library library;
            if (!library.IsValid()) {
                std::println("[ERROR] {} を読み込めませんでした。", library.Path().string());
                return;
            }

            Menu::BusMenu busMenu;
            busMenu.Run(library.BusFactory());
        }
    };
}
