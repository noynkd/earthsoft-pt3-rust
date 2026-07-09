cmake_minimum_required(VERSION 3.15)

project(earthsoft-pt3-lib VERSION 0.0.1 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

set(SOURCES
    example/main.cpp
)
add_executable(pt3-example-cpp ${SOURCES})
target_precompile_headers(pt3-example-cpp PRIVATE example/pch.h)

if(MSVC)
    target_compile_options(pt3-example-cpp PRIVATE
        /W3
        /EHsc
        /utf-8
    )

    set_target_properties(pt3-example-cpp PROPERTIES
        MSVC_RUNTIME_LIBRARY "MultiThreaded$<$<CONFIG:Debug>:Debug>"
    )
endif()

set_target_properties(pt3-example-cpp PROPERTIES
    RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}
)

install(TARGETS pt3-example-cpp RUNTIME DESTINATION bin)
install(DIRECTORY include/ DESTINATION include)
