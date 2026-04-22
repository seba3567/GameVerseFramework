# GameVerse Client Architecture

## Overview

Client-side implementation for GameVerse Framework, enabling players to connect directly to GameVerse servers.

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                      GameVerse Client                        │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────┐  ┌──────────┐  ┌───────────┐  ┌───────────┐  │
│  │   UI    │  │  Script  │  │   Game    │  │  Network  │  │
│  │ Renderer│  │ Runtime  │  │ Integration│ │  Manager  │  │
│  └────┬────┘  └────┬─────┘  └─────┬─────┘  └─────┬─────┘  │
│       │           │             │              │          │
│  ┌────┴───────────┴─────────────┴──────────────┴────┐      │
│  │                  Client Core                    │      │
│  │  ┌─────────┐  ┌─────────┐  ┌─────────────────┐ │      │
│  │  │Resource │  │  Event  │  │  Connection     │ │      │
│  │  │ Loader  │  │  Bus    │  │  Manager        │ │      │
│  │  └─────────┘  └─────────┘  └─────────────────┘ │      │
│  └────────────────────────────────────────────────┘      │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼ HTTP/3 + QUIC / WebSocket
                            │
┌─────────────────────────────────────────────────────────────┐
│                    GameVerse Server                         │
└─────────────────────────────────────────────────────────────┘
```

## Directory Structure

```
client/
├── src/
│   ├── lib.rs                 # Client library entry
│   ├── lib.rs
│   ├── main.rs                # Client executable entry
│   ├── core/
│   │   ├── mod.rs
│   │   ├── context.rs         # Client context
│   │   ├── events.rs          # Event system
│   │   └── tick.rs            # Game tick management
│   ├── network/
│   │   ├── mod.rs
│   │   ├── manager.rs          # Connection management
│   │   ├── protocols/
│   │   │   ├── mod.rs
│   │   │   ├── quic.rs         # QUIC protocol
│   │   │   ├── websocket.rs    # WebSocket fallback
│   │   │   └── http3.rs        # HTTP/3 protocol
│   │   └── packets.rs          # Packet serialization
│   ├── scripting/
│   │   ├── mod.rs
│   │   ├── lua_runtime.rs      # Lua scripting
│   │   ├── ts_runtime.rs       # TypeScript runtime (Deno)
│   │   └── wasm_runtime.rs     # WebAssembly runtime
│   ├── ui/
│   │   ├── mod.rs
│   │   ├── renderer.rs         # UI rendering
│   │   ├── cef_bridge.rs       # CEF/NUI bridge
│   │   └── webrenderer.rs      # Web-based renderer
│   ├── resources/
│   │   ├── mod.rs
│   │   ├── loader.rs           # Resource loader
│   │   ├── manifest.rs         # fxmanifest parsing
│   │   └── watcher.rs          # Hot-reload file watcher
│   ├── game/
│   │   ├── mod.rs
│   │   ├── integration.rs      # Game hooking
│   │   ├── natives.rs          # Native function bindings
│   │   └── sync.rs            # Client-side prediction & sync
│   └── build/
│       └── mod.rs
├── Cargo.toml
└── README.md
```

## Module Responsibilities

### Core Modules

| Module | Responsibility |
|--------|---------------|
| `core/context` | Global client state, initialization |
| `core/events` | Pub/sub event system for client-server communication |
| `core/tick` | Main game loop coordination |

### Network Module

| Module | Responsibility |
|--------|---------------|
| `network/manager` | Connection pool, reconnection logic |
| `network/quic` | QUIC transport (primary protocol) |
| `network/websocket` | WebSocket fallback |
| `network/packets` | FlatBuffers serialization |

### Scripting Module

| Module | Responsibility |
|--------|---------------|
| `scripting/lua` | Lua/LuaJIT runtime |
| `scripting/ts` | TypeScript via Deno runtime |
| `scripting/wasm` | WebAssembly via Wasmtime |

### UI Module

| Module | Responsibility |
|--------|---------------|
| `ui/renderer` | Cross-platform UI rendering |
| `ui/cef_bridge` | Chromium Embedded Framework for NUI |
| `ui/webrenderer` | Lightweight web renderer alternative |

### Resource Module

| Module | Responsibility |
|--------|---------------|
| `resources/loader` | Load/unload client resources |
| `resources/manifest` | Parse fxmanifest.lua |
| `resources/watcher` | Hot-reload file monitoring |

## Connection Flow

```
1. Client Start
   └── Load fxmanifest.lua from resources/
2. Connect to Server
   └── network::Manager::connect(address, port)
       └── Try QUIC first, fallback to WebSocket
3. Handshake
   └── Send HELLO packet with version, auth token
   └── Receive ACCEPTED / REJECTED
4. Resource Loading
   └── Server sends resource list
   └── Client downloads and loads resources
5. Game Loop
   └── Tick events → Script execution → Network sync
6. Disconnect
   └── Cleanup resources, notify server
```

## Event System

```rust
// Client events
enum ClientEvent {
    Connect { server: String },
    Disconnect { reason: String },
    ResourceStart { name: String },
    ResourceStop { name: String },
    Tick { delta_ms: u64 },
    ServerEvent { name: String, data: Vec<u8> },
}

// Trigger events to server
client.trigger("event_name", payload);
client.trigger_server("server_event", payload);
```

## Dependencies (Cargo.toml)

```toml
[dependencies]
# Networking
quinn = "0.10"
ws_stream_tungstenite = "0.10"
futures-util = "0.3"

# Scripting
mlua = { version = "0.8", features = ["luajit"] }
ruscript = "0.1"
wasmtime = "20"

# UI
wry = "0.20"
web_view = "0.7"

# Serialization
flatbuffers = "23.3"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Utilities
tokio = { version = "1.35", features = ["full"] }
tracing = "0.1"
uuid = "1.6"
```

## Build Targets

- `gameverse_client` - Main client executable
- `gameverse_client_shared` - Shared library for embedding

## Integration with Game Engine

The client integrates with GTA V / RDR2 via:
1. **Natives bridge** - Bindings to game native functions
2. **Memory injection** - For entity manipulation
3. **Render hook** - UI overlay rendering
