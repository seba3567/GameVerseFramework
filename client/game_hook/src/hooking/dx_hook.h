// DirectX Hook for UI rendering
// dx_hook.h

#pragma once

#include <windows.h>
#include <d3d11.h>
#include <d3d12.h>
#include <dxgi.h>
#include <functional>

namespace Hooking {

// DirectX hook for overlay rendering
struct DXHook {
    // Check which DirectX version is in use
    static bool Init();
    
    // Present hook for DX11
    static void InstallDX11();
    
    // Present hook for DX12  
    static void InstallDX12();
    
    // EndScene hook for older DirectX
    static void InstallEndScene();
    
    // Render callback - called every frame
    static void SetRenderCallback(std::function<void()> callback);
    
    // Cleanup
    static void Shutdown();
};

// Hook storage
struct DXHookState {
    // DX11
    ID3D11Device* device11 = nullptr;
    ID3D11DeviceContext* context11 = nullptr;
    IDXGISwapChain* swapchain11 = nullptr;
    
    // DX12
    ID3D12Device* device12 = nullptr;
    ID3D12CommandQueue* command_queue12 = nullptr;
    
    // Hooks
    void* original_present_11 = nullptr;
    void* original_present_12 = nullptr;
    void* original_resize_11 = nullptr;
    void* original_resize_12 = nullptr;
    
    // Render callback
    std::function<void()> render_callback;
    
    // Initialization state
    bool initialized = false;
};

extern DXHookState g_dx_hook_state;

// DirectX 11 Present hook
typedef HRESULT(WINAPI* PFN_Present11)(
    IDXGISwapChain* This,
    UINT SyncInterval,
    UINT Flags
);

// DirectX 12 Present hook  
typedef HRESULT(WINAPI* PFN_Present12)(
    IDXGISwapChain* This,
    UINT SyncInterval,
    UINT Flags
);

// Hook implementations
namespace DX11 {
    extern "C" {
        HRESULT WINAPI HookedPresent11(
            IDXGISwapChain* This,
            UINT SyncInterval,
            UINT Flags
        );
    }
}

namespace DX12 {
    extern "C" {
        HRESULT WINAPI HookedPresent12(
            IDXGISwapChain* This,
            UINT SyncInterval,
            UINT Flags
        );
    }
}

} // namespace Hooking
