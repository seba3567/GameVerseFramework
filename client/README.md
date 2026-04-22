# GameVerse Client Architecture

Complete client architecture for connecting players to GameVerse servers, similar to FiveM.

## Overview

```
┌────────────────────────────────────────────────────────────────────┐
│                      GameVerse Launcher                             │
│                     (gameverse_client)                              │
├────────────────────────────────────────────────────────────────────┤
│  ┌──────────────────────────────────────────────────────────────┐   │
│  │  GameLauncher + Injector                                      │   │
│  │  - Auto-detects GTA V / RDR3 installation                    │   │
│  │  - Launches game with custom arguments                        │   │
│  │  - Injects game_hook.dll when game starts                     │   │
│  └──────────────────────────────────────────────────────────────┘   │
└────────────────────────────────────────────────────────────────────┘
                              │
                              │ game_hook.dll injected
                              ▼
┌────────────────────────────────────────────────────────────────────┐
│                      GTA V / RDR2 Process                           │
├────────────────────────────────────────────────────────────────────┤
│                                                                     │
│  ┌──────────────────────────────────────────────────────────────┐   │
│  │  GameVerse Hook DLL (game_hook.dll)                          │   │
│  │                                                               │   │
│  │  ┌─────────────┐  ┌──────────────┐  ┌────────────────────┐   │   │
│  │  │  Memory     │  │  DirectX     │  │  Network Socket    │   │   │
│  │  │  Pattern    │  │  Hook        │  │  (QUIC/WebSocket)  │   │   │
│  │  │  Scanner    │  │  (Present)   │  │                    │   │   │
│  │  └─────────────┘  └──────────────┘  └────────────────────┘   │   │
│  │                                                               │   │
│  │  ┌─────────────┐  ┌──────────────┐  ┌────────────────────┐   │   │
│  │  │  Native     │  │  UI Layer   │  │  Script Engine    │   │   │
│  │  │  Functions  │  │  (CEF/IMGUI)│  │  (Lua)            │   │   │
│  │  └─────────────┘  └──────────────┘  └────────────────────┘   │   │
│  │                                                               │   │
│  └──────────────────────────────────────────────────────────────┘   │
│                                                                     │
│  ┌──────────────────────────────────────────────────────────────┐   │
│  │  Game Internals (GTA V / RDR2)                                │   │
│  │  - Entity system (peds, vehicles, objects)                   │   │
│  │  - Native functions                                           │   │
│  │  - Render engine (DirectX 11/12)                             │   │
│  └──────────────────────────────────────────────────────────────┘   │
│                                                                     │
└────────────────────────────────────────────────────────────────────┘
                              │
                              │ QUIC / HTTP3
                              ▼
┌────────────────────────────────────────────────────────────────────┐
│                      GameVerse Server                               │
└────────────────────────────────────────────────────────────────────┘
```

## Components

### 1. GameVerse Launcher (`gameverse_client`)

Rust binary that:
- Auto-detects game installation (Steam, Epic, Rockstar)
- Launches the game with custom arguments
- Injects `game_hook.dll` when game process starts
- Handles connection to GameVerse server

### 2. Game Hook DLL (`game_hook.dll`)

C++ DLL injected into the game process:

| Module | Purpose |
|--------|---------|
| `hooking/mod_patterns.h` | Memory pattern scanning |
| `hooking/inline_hook.h` | Function hooking |
| `hooking/dx_hook.h` | DirectX Present hook |
| `game/gta5/offsets.h` | Memory offsets |
| `network/socket.h` | QUIC/WebSocket transport |
| `renderer/ui_layer.h` | UI overlay rendering |
| `scripts/script_engine.h` | Lua script execution |

## Usage

### Auto-Launch (Recommended)

```bash
# Launch GTA V and connect to server
./gameverse_client --launch --server 192.168.1.100 --port 8080

# Launch RDR3
./gameverse_client --launch --game rdr3 --server 192.168.1.100 --port 8080

# Windowed mode with custom path
./gameverse_client --launch --game gta5 --windowed --game-path "D:\Games\GTAV"
```

### Development Mode (no game injection)

```bash
# Connect to server without launching game
./gameverse_client --server 192.168.1.100 --port 8080 --resource ./resources
```

## Building

### Rust Client

```bash
cargo build -p gameverse-client --release
```

### Game Hook DLL

```bash
# Requires Visual Studio 2022 + Windows SDK
mkdir -p client/game_hook/build && cd client/game_hook/build
cmake .. -A x64 -DCMAKE_BUILD_TYPE=Release
cmake --build . --config Release
```

Output: `client/game_hook/bin/game_hook.dll`

## Connection Flow

```
1. User runs: gameverse_client --launch --server 192.168.1.100

2. Launcher:
   - Detects GTA V installation (Steam/Epic/Rockstar)
   - Launches GTAV.exe with -noBrowser -skipPrivacy flags
   - Waits for game process to start

3. DLL Injection:
   - Injects game_hook.dll into GTAV.exe process
   - DLL initializes: hooking, memory scan, network

4. Network Handshake:
   - DLL connects to 192.168.1.100:8080 via QUIC
   - Server sends ACCEPTED
   - Downloads and loads resources

5. Game Loop:
   - Scripts execute (Lua/WASM)
   - UI renders (CEF overlay)
   - Events sync with server

6. User plays in GTA V multiplayer!
```

## File Structure

```
client/
├── src/
│   ├── main.rs              # CLI entry point
│   ├── lib.rs               # Library exports
│   ├── core/                # Client state management
│   ├── network/             # Network protocols
│   ├── scripting/           # Script runtimes
│   ├── ui/                  # UI rendering
│   ├── resources/           # Resource loader
│   ├── game/                # Game integration
│   └── launcher/            # Auto-launch system
├── game_hook/               # C++ injection layer
│   ├── src/
│   │   ├── dllmain.cpp
│   │   ├── hooking/
│   │   ├── game/
│   │   ├── network/
│   │   ├── renderer/
│   │   ├── scripts/
│   │   └── utils/
│   ├── CMakeLists.txt
│   └── README.md
├── Cargo.toml
└── ARCHITECTURE.md
```

## Supported Games

| Game | Status | Game ID |
|------|--------|---------|
| Grand Theft Auto V | ✅ Tested | `gta5`, `gta` |
| Red Dead Redemption 2 | 🛠️ In development | `rdr3`, `rdr` |

## Network Protocols

1. **QUIC (primary)** - HTTP/3 based, low latency
2. **WebSocket (fallback)** - For firewalls/proxies
3. **HTTP/3** - For REST API calls

## Scripting

- **Lua** - Primary scripting language
- **TypeScript** - Via Deno (future)
- **WebAssembly** - Via Wasmtime (future)

## Security Notes

- Authentication token sent during handshake
- TLS encryption for QUIC connections
- Server validates client version before accepting
