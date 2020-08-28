#include <cstdint>

#include <IUnityGraphics.h>
#include <IUnityInterface.h>
#include <IUnityProfilerCallbacks.h>
#include <IUnityRenderingExtensions.h>
#include <IUnityShaderCompilerAccess.h>

typedef void *ID3D11Device;
typedef void *ID3D11Resource;
typedef void *ID3D11RenderTargetView;
typedef void *ID3D11ShaderResourceView;
#include <IUnityGraphicsD3D11.h>

typedef std::int32_t D3D12_RESOURCE_STATES;
typedef std::uint64_t UINT64;
typedef void *ID3D12Resource;
typedef void *ID3D12Device;
typedef void *ID3D12Fence;
typedef void *ID3D12GraphicsCommandList;
typedef void *ID3D12CommandQueue;
typedef void *ID3D12Resource;
typedef void *ID3D12CommandQueue;
#include <IUnityGraphicsD3D12.h>
