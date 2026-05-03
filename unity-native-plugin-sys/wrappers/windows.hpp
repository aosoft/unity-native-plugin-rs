#include <cstdint>
#include <cstddef>

#define UINT32 uint32_t
#define UINT uint32_t

#include <IUnityInterface.h>

struct ID3D11Device {};
struct ID3D11Resource {};
struct ID3D11RenderTargetView {};
struct ID3D11ShaderResourceView {};
struct IDXGISwapChain {};
#include <IUnityGraphicsD3D11.h>

typedef std::int32_t D3D12_RESOURCE_STATES;
typedef unsigned long long UINT64;
struct ID3D12Resource {};
struct ID3D12Device {};
struct ID3D12Fence {};
struct ID3D12GraphicsCommandList {};
struct ID3D12CommandQueue {};
#include <IUnityGraphicsD3D12.h>
