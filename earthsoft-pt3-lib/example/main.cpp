#include "pch.h"
#include "main.h"
#include <crtdbg.h>

int main() {
    ::SetConsoleOutputCP(CP_UTF8);

    _CrtSetDbgFlag(_CRTDBG_ALLOC_MEM_DF | _CRTDBG_CHECK_ALWAYS_DF | _CRTDBG_LEAK_CHECK_DF);

    Earthsoft::Example::Main main;
    main.Run();

    std::clog << "\n続行するには Enter キーを押してください...";
    std::cin.get();

    return 0;
}
