// GameVerse Hook DLL
// dllmain.cpp - DLL entry point

#include <windows.h>
#include <cstdint>
#include <cstring>
#include <string>

#include "hooking/mod_patterns.h"
#include "hooking/inline_hook.h"
#include "hooking/dx_hook.h"
#include "game/gta5/offsets.h"
#include "network/socket.h"
#include "renderer/ui_layer.h"
#include "scripts/script_engine.h"

// Logger
#include "utils/logger.h"

// Export macros
#define EXPORT __declspec(dllexport)

namespace GV {
    bool g_initialized = false;
    bool g_connected = false;
    std::string g_server_addr;
    int g_server_port = 0;
    
    Network::Socket* g_socket = nullptr;
    ScriptEngine* g_script_engine = nullptr;
    UILayer* g_ui_layer = nullptr;
}

// DLL entry point
BOOL WINAPI DllMain(HMODULE hModule, DWORD ul_reason_for_call, LPVOID lpReserved) {
    switch (ul_reason_for_call) {
    case DLL_PROCESS_ATTACH:
        Logger::Init();
        Logger::Info("GameVerse Hook DLL loaded");
        
        // Disable thread notifications for performance
        DisableThreadLibraryCalls(hModule);
        
        // Initialize hooking system
        Hooking::Init();
        
        // Find game offsets
        Game::GTA5::InitOffsets();
        
        // Setup DirectX hook for rendering
        Hooking::DXHook::Install();
        
        // Setup inline hooks for game functions
        Hooking::InstallInlineHooks();
        
        Logger::Info("GameVerse Hook initialized");
        break;
        
    case DLL_PROCESS_DETACH:
        Logger::Info("GameVerse Hook DLL unloading");
        if (GV::g_socket) {
            GV::g_socket->Disconnect();
            delete GV::g_socket;
        }
        Hooking::Shutdown();
        break;
    }
    return TRUE;
}

// Exported functions for Rust client
extern "C" {
    
EXPORT bool GV_Init(const char* server_addr, int port) {
    if (GV::g_initialized) {
        Logger::Warn("GV_Init called but already initialized");
        return true;
    }
    
    Logger::Info("Initializing GameVerse Hook");
    Logger::Info("Server: %s:%d", server_addr, port);
    
    GV::g_server_addr = server_addr;
    GV::g_server_port = port;
    
    // Initialize network socket
    GV::g_socket = new Network::Socket();
    if (!GV::g_socket->Connect(server_addr, port)) {
        Logger::Error("Failed to connect to server");
        return false;
    }
    
    // Initialize script engine
    GV::g_script_engine = new ScriptEngine();
    GV::g_script_engine->Init();
    
    // Initialize UI layer
    GV::g_ui_layer = new UILayer();
    GV::g_ui_layer->Init();
    
    // Start network receive thread
    std::thread([=]() {
        while (GV::g_initialized) {
            GV::g_socket->Receive();
            std::this_thread::sleep_for(std::chrono::milliseconds(1));
        }
    }).detach();
    
    GV::g_initialized = true;
    GV::g_connected = true;
    
    Logger::Info("GameVerse Hook ready");
    return true;
}

EXPORT void GV_Shutdown() {
    if (!GV::g_initialized) return;
    
    Logger::Info("Shutting down GameVerse Hook");
    
    GV::g_initialized = false;
    GV::g_connected = false;
    
    if (GV::g_socket) {
        GV::g_socket->Disconnect();
        delete GV::g_socket;
        GV::g_socket = nullptr;
    }
    
    if (GV::g_script_engine) {
        GV::g_script_engine->Shutdown();
        delete GV::g_script_engine;
        GV::g_script_engine = nullptr;
    }
    
    if (GV::g_ui_layer) {
        GV::g_ui_layer->Shutdown();
        delete GV::g_ui_layer;
        GV::g_ui_layer = nullptr;
    }
    
    Logger::Info("GameVerse Hook shutdown complete");
}

EXPORT int GV_GetStatus() {
    if (!GV::g_initialized) return 0; // Not initialized
    if (GV::g_connected) return 1;    // Connected
    return 2;                          // Error/Disconnected
}

EXPORT void GV_TriggerEvent(const char* event, const char* data) {
    if (!GV::g_initialized || !GV::g_socket) return;
    
    // Forward event to server
    GV::g_socket->SendEvent(event, data);
}

EXPORT void GV_LoadResource(const char* resource_name) {
    if (!GV::g_initialized) return;
    
    Logger::Info("Loading resource: %s", resource_name);
    
    // Load resource scripts
    std::string path = std::string("resources/") + resource_name + "/fxmanifest.lua";
    
    // Parse and load scripts
    if (GV::g_script_engine) {
        GV::g_script_engine->LoadResource(path.c_str());
    }
}

EXPORT void GV_DoTick() {
    if (!GV::g_initialized) return;
    
    // Game tick - called from game loop
    if (GV::g_script_engine) {
        GV::g_script_engine->Tick();
    }
    
    if (GV::g_ui_layer) {
        GV::g_ui_layer->Render();
    }
}

EXPORT uintptr_t GV_GetNative(uint64_t hash) {
    return Game::GTA5::GetNative(hash);
}

}
