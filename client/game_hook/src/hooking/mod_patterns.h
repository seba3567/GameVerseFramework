// Pattern scanning for memory addresses
// mod_patterns.h

#pragma once

#include <windows.h>
#include <cstdint>
#include <vector>
#include <string>
#include <optional>
#include <functional>

namespace Hooking {

// Memory region to scan
struct MemoryRegion {
    uintptr_t address;
    size_t size;
};

// Byte pattern with wildcard support
class Pattern {
public:
    Pattern(const char* pattern) : pattern_(pattern), mask_("") {
        parse_pattern(pattern);
    }
    
    // Find pattern in memory
    std::optional<uintptr_t> find(const uint8_t* data, size_t size) const {
        size_t pattern_len = pattern_.size();
        
        for (size_t i = 0; i <= size - pattern_len; i++) {
            if (matches(data + i)) {
                return std::make_optional(reinterpret_cast<uintptr_t>(data + i));
            }
        }
        
        return std::nullopt;
    }
    
    // Find pattern in all memory regions
    std::optional<uintptr_t> find_process() const;
    
private:
    std::vector<uint8_t> pattern_;
    std::string mask_;
    
    void parse_pattern(const char* pattern);
    bool matches(const uint8_t* data) const;
};

// Pattern scanner utility
struct pattern_scanner {
    // Find a pattern in memory
    static std::optional<uintptr_t> find(const char* module_name, const char* pattern) {
        Pattern p(pattern);
        
        HMODULE module = GetModuleHandleA(module_name);
        if (!module) return std::nullopt;
        
        MODULEINFO info;
        if (!GetModuleInformation(GetCurrentProcess(), module, &info, sizeof(info))) {
            return std::nullopt;
        }
        
        return p.find(reinterpret_cast<uint8_t*>(info.lpBaseOfDll), info.SizeOfImage);
    }
    
    // Find in specific module by offset range
    static std::optional<uintptr_t> find_in_range(
        const char* module_name,
        size_t start_offset,
        size_t end_offset,
        const char* pattern
    ) {
        Pattern p(pattern);
        
        HMODULE module = GetModuleHandleA(module_name);
        if (!module) return std::nullopt;
        
        uint8_t* base = reinterpret_cast<uint8_t*>(module);
        uint8_t* start = base + start_offset;
        uint8_t* end = base + end_offset;
        
        return p.find(start, end - start);
    }
};

// Pattern parsing helper
inline void Pattern::parse_pattern(const char* pattern) {
    std::string cur_mask;
    const char* p = pattern;
    
    while (*p) {
        if (*p == ' ') {
            p++;
            continue;
        }
        
        if (*p == '?') {
            pattern_.push_back(0);
            cur_mask += '?';
            p++;
        } else {
            // Read two hex digits
            char byte_str[3] = { p[0], p[1], 0 };
            uint8_t byte = static_cast<uint8_t>(strtol(byte_str, nullptr, 16));
            pattern_.push_back(byte);
            cur_mask += 'x';
            p += 2;
        }
    }
    
    mask_ = cur_mask;
}

inline bool Pattern::matches(const uint8_t* data) const {
    for (size_t i = 0; i < pattern_.size(); i++) {
        if (mask_[i] == 'x' && data[i] != pattern_[i]) {
            return false;
        }
    }
    return true;
}

} // namespace Hooking
