// Lua Script Engine
// script_engine.h

#pragma once

#include <string>
#include <vector>
#include <functional>
#include <memory>
#include <map>

extern "C" {
#include <lua.h>
#include <lauxlib.h>
#include <lualib.h>
}

namespace Script {

// Lua state wrapper
class LuaState {
public:
    LuaState();
    ~LuaState();
    
    // Create new state
    bool Init();
    
    // Shutdown
    void Shutdown();
    
    // Execute script
    bool Execute(const char* script);
    
    // Load script file
    bool LoadFile(const char* path);
    
    // Call function
    bool Call(const char* func, int args, int results);
    
    // Get global
    void GetGlobal(const char* name);
    void SetGlobal(const char* name);
    
    // Push values
    void Push(int value);
    void Push(float value);
    void Push(const char* value);
    void PushBool(bool value);
    
    // Pop values
    int PopInt();
    float PopFloat();
    const char* PopString();
    bool PopBool();
    
    // Create table
    void CreateTable();
    void SetField(const char* key);
    void GetField(const char* key);
    
    lua_State* State() { return L_; }

private:
    lua_State* L_ = nullptr;
};

// Script resource
struct Resource {
    std::string name;
    std::string path;
    std::vector<std::string> client_scripts;
    std::vector<std::string> server_scripts;
    std::vector<std::string> shared_scripts;
    bool loaded = false;
};

// Script engine - manages Lua execution
class Engine {
public:
    Engine();
    ~Engine();
    
    // Initialize engine
    void Init();
    
    // Shutdown engine
    void Shutdown();
    
    // Load resource
    bool LoadResource(const char* path);
    
    // Unload resource
    void UnloadResource(const char* name);
    
    // Tick - called every frame
    void Tick();
    
    // Trigger event to all scripts
    void TriggerEvent(const char* event, const char* data);
    
    // Register native function
    void RegisterNative(const char* name, int (*func)(lua_State*));
    
    // Register callable function
    void RegisterFunction(const char* name, std::function<int()> func);
    
private:
    // Load manifest
    bool LoadManifest(const char* path);
    
    // Setup globals
    void SetupGlobals();
    
    // Create client API table
    void CreateClientAPI();
    
    std::unique_ptr<LuaState> state_;
    std::map<std::string, Resource> resources_;
    
    // Registered functions
    std::map<std::string, std::function<int()>> functions_;
};

} // namespace Script
