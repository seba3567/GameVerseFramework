cmake_minimum_required(VERSION 3.20)
project(GameVerseHook)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Directories
include_directories(include)
include_directories(src)

# Source files
set(SOURCES
    src/dllmain.cpp
    src/hooking/mod_patterns.cpp
    src/hooking/inline_hook.cpp
    src/hooking/dx_hook.cpp
    src/network/socket.cpp
    src/renderer/ui_layer.cpp
    src/scripts/script_engine.cpp
)

# Libraries
find_package(DirectX REQUIRED)
find_package(Threads REQUIRED)

# Create DLL
add_library(game_hook SHARED ${SOURCES})

# Link libraries
target_link_libraries(game_hook 
    DirectX::d3d11 
    DirectX::dxgi
    Threads::Threads
)

# Output directory
set_target_properties(game_hook PROPERTIES
    LIBRARY_OUTPUT_DIRECTORY ${CMAKE_SOURCE_DIR}/bin
    RUNTIME_OUTPUT_DIRECTORY ${CMAKE_SOURCE_DIR}/bin
)

# Copy to client bin on build
add_custom_command(TARGET game_hook POST_BUILD
    COMMAND ${CMAKE_COMMAND} -E copy
        $<TARGET_FILE:game_hook>
        ${CMAKE_SOURCE_DIR}/../bin/
)

# Preprocessor definitions
target_compile_definitions(game_hook PRIVATE
    WIN32_LEAN_AND_MEAN
    _WINSOCK_DEPRECATED_NO_WARNINGS
)
