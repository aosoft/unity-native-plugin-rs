#include <cstdint>

#include <IUnityGraphics.h>
#include <IUnityInterface.h>
#include <IUnityProfilerCallbacks.h>
#include <IUnityRenderingExtensions.h>
#include <IUnityShaderCompilerAccess.h>

struct ID3D11Device {};
struct ID3D11Resource {};
struct ID3D11RenderTargetView {};
struct ID3D11ShaderResourceView {};
#include <IUnityGraphicsD3D11.h>

typedef std::int32_t D3D12_RESOURCE_STATES;
typedef std::uint64_t UINT64;
struct ID3D12Resource {};
struct ID3D12Device {};
struct ID3D12Fence {};
struct ID3D12GraphicsCommandList {};
struct ID3D12CommandQueue {};
#include <IUnityGraphicsD3D12.h>
