// GTA V Memory Offsets and Native Functions
// offsets.h

#pragma once

#include <cstdint>

namespace Game {
namespace GTA5 {

// Critical memory offsets (may change with game updates)
// These are for reference - use pattern scanning in production

struct Offsets {
    // Global variables
    static constexpr uint64_t GLOBAL_BASE = 0x1A9CC0;
    
    // Player
    static constexpr uint64_t PLAYER_PED = 0x8D18B0;
    static constexpr uint64_t PLAYER_ID = 0x98C8;
    
    // World
    static constexpr uint64_t WORLD_BASE = 0xC00000;
    static constexpr uint64_t POOL_PED = 0x10C4778;
    static constexpr uint64_t POOL_VEHICLE = 0x10C4770;
    static constexpr uint64_t POOL_OBJECT = 0x10C4768;
    
    // Network
    static constexpr uint64_t NETWORK_BASE = 0x1500200;
    static constexpr uint64_t NETWORK_CLAN = 0x200;
    
    // Entity
    static constexpr uint64_t ENTITY_HEALTH = 0x5C0;
    static constexpr uint64_t ENTITY_MAX_HEALTH = 0x5C4;
    static constexpr uint64_t ENTITY_POSITION = 0x30;
    static constexpr uint64_t ENTITY_ROTATION = 0x34;
    static constexpr uint64_t ENTITY_VELOCITY = 0x280;
    
    // Vehicle
    static constexpr uint64_t VEHICLE_HANDLING = 0xD18;
    static constexpr uint64_t VEHICLE_BULLET_PROOF = 0x788;
    static constexpr uint64_t VEHICLE_FIRE_PROOF = 0x78C;
    static constexpr uint64_t VEHICLE_EXPLOSION_PROOF = 0x790;
    
    // Ped
    static constexpr uint64_t PED_RAGDOLL = 0x10A8;
    static constexpr uint64_t PED_ARMOUR = 0x14D8;
    static constexpr uint64_t PED_NETWORKED = 0x280;
    
    // Weapon
    static constexpr uint64_t WEAPON_CURRENT = 0x18;
    
    // Camera
    static constexpr uint64_t CAM_ACTIVE = 0x28;
    static constexpr uint64_t CAM_POSITION = 0x80;
    
    // Script
    static constexpr uint64_t SCRIPT_THREAD_BASE = 0x8F1308;
    
    // UI
    static constexpr uint64_t UI_HUD = 0xC620F0;
    static constexpr uint64_t UI_HELP_TEXT = 0xC77200;
};

// Native hashes (FNV1A64)
struct Natives {
    // World
    static constexpr uint64_t GET_PLAYER_PED = 0x6E31C103;
    static constexpr uint64_t GET_PLAYER_INT = 0x7EE0D61B;
    static constexpr uint64_t GET_ENTITY_COORDS = 0x5E539605;
    static constexpr uint64_t SET_ENTITY_COORDS = 0x4F633F54;
    static constexpr uint64_t GET_DISTANCE_BETWEEN_COORDS = 0x4EE53663;
    
    // Vehicle
    static constexpr uint64_t CREATE_VEHICLE = 0xC2F21F90;
    static constexpr uint64_t DELETE_VEHICLE = 0x5E84E0F6;
    static constexpr uint64_t SET_VEHICLE_ONGROUND_PROPERLY = 0x1F8D0AA3;
    static constexpr uint64_t SET_ENTITY_VISIBLE = 0x2D3F55CC;
    
    // Ped
    static constexpr uint64_t CREATE_PED = 0x28B74F0C;
    static constexpr uint64_t DELETE_PED = 0x961F2776;
    static constexpr uint64_t SET_PED_ARMOUR = 0x0C9D8891;
    static constexpr uint64_t SET_PED_MONEY = 0xB3FC6D6D;
    
    // Weapon
    static constexpr uint64_t GIVE_WEAPON = 0x1A3F1116;
    static constexpr uint64_t REMOVE_WEAPON = 0xE6203FBD;
    static constexpr uint64_t SET_CURRENT_PED_WEAPON = 0x1758F82C;
    
    // Network
    static constexpr uint64_t NETWORK_HAS_NETWORK_TIME = 0x5B87F0E5;
    static constexpr uint64_t NETWORK_GET_TALK_CHAT = 0x33F1E82F;
    
    // UI
    static constexpr uint64_t BEGIN_TEXT_COMMAND = 0x27E1F35C;
    static constexpr uint64_t END_TEXT_COMMAND = 0x5F2BF33;
    static constexpr uint64_t DRAW_TEXT = 0x42B25C06;
};

// Get native function address from hash
inline uint64_t* GetNative(uint64_t hash) {
    // In production, this looks up the native handler table
    // For now, return nullptr
    return nullptr;
}

// Initialize offset scanning
void InitOffsets();

} // namespace GTA5
} // namespace Game
