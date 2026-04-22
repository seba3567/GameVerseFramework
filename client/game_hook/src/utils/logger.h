// Simple logger for DLL
// logger.h

#pragma once

#include <windows.h>
#include <cstdio>
#include <cstdarg>
#include <string>
#include <chrono>
#include <iomanip>
#include <sstream>

namespace Logger {

// Log levels
enum Level {
    Debug,
    Info,
    Warn,
    Error
};

inline FILE* g_logfile = nullptr;
inline Level g_min_level = Level::Debug;

inline void Init() {
    // Open log file
    g_logfile = fopen("gameverse_hook.log", "a");
}

inline void Shutdown() {
    if (g_logfile) {
        fclose(g_logfile);
        g_logfile = nullptr;
    }
}

inline const char* LevelString(Level level) {
    switch (level) {
        case Debug: return "DEBUG";
        case Info:  return "INFO ";
        case Warn:  return "WARN ";
        case Error: return "ERROR";
        default:    return "UNKN ";
    }
}

inline void Log(Level level, const char* format, va_list args) {
    if (level < g_min_level) return;
    
    auto now = std::chrono::system_clock::now();
    auto time = std::chrono::system_clock::to_time_t(now);
    auto ms = std::chrono::duration_cast<std::chrono::milliseconds>(
        now.time_since_epoch()) % 1000;
    
    std::stringstream ss;
    ss << std::put_time(std::localtime(&time), "%H:%M:%S");
    std::string timestamp = ss.str();
    
    FILE* out = g_logfile ? g_logfile : stdout;
    
    fprintf(out, "[%s.%03d] [%s] ", timestamp.c_str(), (int)ms.count(), LevelString(level));
    vfprintf(out, format, args);
    fprintf(out, "\n");
    
    if (g_logfile) {
        fflush(g_logfile);
    }
}

inline void Debug(const char* format, ...) {
    va_list args;
    va_start(args, format);
    Log(Level::Debug, format, args);
    va_end(args);
}

inline void Info(const char* format, ...) {
    va_list args;
    va_start(args, format);
    Log(Level::Info, format, args);
    va_end(args);
}

inline void Warn(const char* format, ...) {
    va_list args;
    va_start(args, format);
    Log(Level::Warn, format, args);
    va_end(args);
}

inline void Error(const char* format, ...) {
    va_list args;
    va_start(args, format);
    Log(Level::Error, format, args);
    va_end(args);
}

} // namespace Logger
