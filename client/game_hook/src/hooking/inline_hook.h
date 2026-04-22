// Inline function hooking
// inline_hook.h

#pragma once

#include <windows.h>
#include <cstdint>
#include <cstring>
#include <type_traits>

namespace Hooking {

// Trampoline for original function
template<typename T>
struct Trampoline {
    T function;
    uint8_t* original_bytes;
    size_t original_size;
};

// Inline hook manager
class InlineHook {
public:
    // Install hook at address
    template<typename T, typename H>
    static Trampoline<T> Install(uintptr_t address, H hook_function, size_t hook_size = 16) {
        Trampoline<T> trampoline;
        trampoline.original_bytes = new uint8_t[hook_size];
        trampoline.original_size = hook_size;
        
        // Save original bytes
        memcpy(trampoline.original_bytes, reinterpret_cast<void*>(address), hook_size);
        
        // Create trampoline
        uint8_t* trampoline_code = new uint8_t[hook_size + 16];
        
        // Copy original bytes
        memcpy(trampoline_code, trampoline.original_bytes, hook_size);
        
        // Add jump back to original + hook_size
        uintptr_t jump_back = address + hook_size;
        *reinterpret_cast<uintptr_t*>(trampoline_code + hook_size) = jump_back;
        
        trampoline.function = reinterpret_cast<T>(trampoline_code);
        
        // Overwrite with jump to hook
        DWORD old_protect;
        VirtualProtect(reinterpret_cast<void*>(address), hook_size, PAGE_EXECUTE_READWRITE, &old_protect);
        
        // Write jmp [hook_function]
        uint8_t* jmp_instruction = reinterpret_cast<uint8_t*>(address);
        
        // x64 jmp instruction (FF 25 [rip + offset])
        jmp_instruction[0] = 0xFF;
        jmp_instruction[1] = 0x25;
        *reinterpret_cast<uint32_t*>(jmp_instruction + 2) = 0; // offset to be filled
        
        // We need to calculate RIP-relative offset
        uintptr_t rel_offset = reinterpret_cast<uintptr_t>(hook_function) - (address + 6);
        *reinterpret_cast<uintptr_t*>(jmp_instruction + 2) = rel_offset;
        
        VirtualProtect(reinterpret_cast<void*>(address), hook_size, old_protect, &old_protect);
        
        // Flush instruction cache
        FlushInstructionCache(GetCurrentProcess(), reinterpret_cast<void*>(address), hook_size);
        
        return trampoline;
    }
    
    // Uninstall hook
    template<typename T>
    static void Uninstall(Trampoline<T> trampoline, uintptr_t address) {
        if (trampoline.original_bytes) {
            DWORD old_protect;
            VirtualProtect(reinterpret_cast<void*>(address), trampoline.original_size, 
                          PAGE_EXECUTE_READWRITE, &old_protect);
            
            memcpy(reinterpret_cast<void*>(address), trampoline.original_bytes, 
                   trampoline.original_size);
            
            VirtualProtect(reinterpret_cast<void*>(address), trampoline.original_size, 
                           old_protect, &old_protect);
            
            delete[] trampoline.original_bytes;
        }
    }
};

// Function to hook with calling convention awareness
namespace detail {
    // x64 fastcall style hook - first 4 params in RCX, RDX, R8, R9
    template<typename Ret, typename... Args>
    Ret FASTCALL_call(uintptr_t func, Args... args) {
        // Not directly callable - use assembly
        // This would need actual assembly implementation
        return Ret{};
    }
}

// Macro for easier hook definition
#define HOOK(prompt_name, original_func, hook_func, trampoline_var) \
    static Trampoline<decltype(original_func)> trampoline_var = \
        InlineHook::Install(reinterpret_cast<uintptr_t>(original_func), \
                           hook_func, 16)

} // namespace Hooking
