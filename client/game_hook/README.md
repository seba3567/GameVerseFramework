# GameVerse Game Hook DLL

C++ DLL that injects into GTA V / RDR2 process.

## Structure

```
game_hook/
├── src/
│   ├── dllmain.cpp           # DLL entry point
│   ├── hooking/
│   │   ├── mod_patterns.h   # Memory pattern scanning
│   │   ├── vmt_hook.h       # Virtual method table hooking
│   │   ├── inline_hook.h    # Inline function hooking
│   │   └── dx_hook.h        # DirectX hook for rendering
│   ├── game/
│   │   ├── gta5/
│   │   │   ├── natives.h    # GTA V native hashes
│   │   │   ├── offsets.h    # Memory offsets
│   │   │   └── entities.h   # Entity system
│   │   └── rdr3/
│   │       ├── natives.h
│   │       ├── offsets.h
│   │       └── entities.h
│   ├── network/
│   │   ├── socket.h        # Network socket
│   │   └── protocols/
│   │       ├── quic.h
│   │       └── ws.h        # WebSocket fallback
│   ├── renderer/
│   │   ├── ui_layer.h      # UI overlay system
│   │   └── cef_bridge.h    # CEF integration
│   ├── scripts/
│   │   ├── script_engine.h
│   │   └── lua_state.h     # Lua runtime
│   └── utils/
│       ├── logger.h
│       └── memory.h         # Memory utilities
├── include/
│   └── imgui/
├── CMakeLists.txt
└── README.md
```

## Features

- **Memory Pattern Scanning** - Find game functions by byte patterns
- **VMT Hooking** - Hook virtual method tables for rendering
- **Inline Hooking** - Direct function patching
- **DirectX Hook** - Overlay rendering via DirectX 11/12
- **Native Binding** - Call game native functions
- **Network Transport** - QUIC/WebSocket to GameVerse server
- **Script Engine** - Lua script execution

## Building

```bash
# Prerequisites
# - Visual Studio 2022
# - Windows SDK
# - DirectX SDK

mkdir build && cd build
cmake .. -A x64
cmake --build . --config Release
```

## Integration with Client

The DLL exports these functions for the Rust client:

```cpp
extern "C" {
    // Initialize the hook
    __declspec(dllexport) bool GV_Init(const char* server_addr, int port);
    
    // Shutdown and cleanup
    __declspec(dllexport) void GV_Shutdown();
    
    // Get connection status
    __declspec(dllexport) int GV_GetStatus();
    
    // Trigger client event
    __declspec(dllexport) void GV_TriggerEvent(const char* event, const char* data);
}
```

## Memory Offsets

The offsets change with game updates. Use the pattern scanner:

```cpp
// Example: Finding player ped pointer
auto pattern = pattern_scanner::find(
    "48 8B 05 ? ? ? ? 48 85 C0 74 ? 48 8B 0C C8"
);

// Or use known offsets (may need updating)
constexpr size_t PLAYER_PED_OFFSET = 0x8D18B0;
```
