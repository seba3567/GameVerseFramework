// UI Layer for overlay rendering
// ui_layer.h

#pragma once

#include <windows.h>
#include <functional>
#include <string>
#include <memory>
#include <vector>

// Forward declare D3D types
struct ID3D11Device;
struct ID3D11DeviceContext;
struct ID3D12Device;
struct ID3D12CommandQueue;

namespace UI {

// UI element base
struct Element {
    virtual ~Element() = default;
    virtual void Render() = 0;
    float x = 0, y = 0;
    float width = 100, height = 100;
    bool visible = true;
};

// Text element
struct Text : Element {
    std::string text;
    float font_size = 16.0f;
    DWORD color = 0xFFFFFFFF;
    bool outlined = false;
    bool drop_shadow = false;
    
    void Render() override;
};

// Rectangle element
struct Rect : Element {
    DWORD color = 0x80000000;
    float rounding = 0.0f;
    
    void Render() override;
};

// Button element
struct Button : Element {
    std::string text;
    std::function<void()> callback;
    DWORD bg_color = 0xFF222222;
    DWORD hover_color = 0xFF444444;
    
    void Render() override;
};

// Input element
struct Input : Element {
    std::string value;
    std::string placeholder;
    bool focused = false;
    
    void Render() override;
};

// UI Layer - renders overlay UI
class Layer {
public:
    Layer();
    ~Layer();
    
    // Initialize
    void Init();
    
    // Shutdown
    void Shutdown();
    
    // Render all elements
    void Render();
    
    // Add element
    template<typename T>
    T* Add() {
        auto elem = new T();
        elements_.push_back(std::unique_ptr<Element>(elem));
        return elem;
    }
    
    // Remove element
    void Remove(Element* elem);
    
    // Clear all elements
    void Clear();
    
    // Show/hide cursor
    void ShowCursor(bool show);
    
    // Check if cursor shown
    bool IsCursorShown() const { return cursor_visible_; }
    
    // Handle input
    void HandleInput(UINT msg, WPARAM wparam, LPARAM lparam);

private:
    std::vector<std::unique_ptr<Element>> elements_;
    bool cursor_visible_ = false;
    bool initialized_ = false;
    
    // DirectX device references for rendering
    void* d3d_device_ = nullptr;
    void* d3d_context_ = nullptr;
};

// NUI (HTML/CSS) support
class NUI {
public:
    NUI();
    ~NUI();
    
    // Load HTML page
    bool LoadPage(const char* url);
    
    // Execute JavaScript
    bool ExecuteJS(const char* js);
    
    // Send message to page
    void SendMessage(const char* target, const char* message);
    
    // Check if ready
    bool IsReady() const { return ready_; }

private:
    bool ready_ = false;
    void* cef_browser_ = nullptr;
};

} // namespace UI
