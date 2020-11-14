use unity_native_plugin_sys::*;
use winapi::shared::{dxgi, dxgiformat, dxgitype};
use winapi::um::{d3d11, d3dcommon, unknwnbase::IUnknown};
use winit::window::Window;
use wio::com::ComPtr;

struct UnityGraphicsD3D11 {
    device: ComPtr<d3d11::ID3D11Device>,
    interfaces: IUnityGraphicsD3D11,
}

impl UnityGraphicsD3D11 {
    pub fn new(device: ComPtr<d3d11::ID3D11Device>) -> UnityGraphicsD3D11 {
        UnityGraphicsD3D11 {
            device: device,
            interfaces: IUnityGraphicsD3D11 {
                GetDevice: Some(get_device),
                TextureFromRenderBuffer: Some(texture_from_render_buffer),
                TextureFromNativeTexture: Some(texture_from_native_texture),
                RTVFromRenderBuffer: Some(rtv_from_render_buffer),
                SRVFromNativeTexture: Some(srv_from_native_texture),
            },
        }
    }
}

impl crate::intreface::UnityInterfaceBase for UnityGraphicsD3D11 {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn get_unity_interface(&self) -> *mut IUnityInterface {
        unsafe { std::mem::transmute::<_, _>(&self.interfaces) }
    }
}

impl crate::intreface::UnityInterfaceID for UnityGraphicsD3D11 {
    fn get_interface_guid() -> UnityInterfaceGUID {
        unity_native_plugin::graphics::UnityGraphicsD3D11::get_interface_guid()
    }
}

extern "system" fn get_device() -> *mut ID3D11Device {
    std::ptr::null_mut()
}

extern "system" fn texture_from_render_buffer(buffer: UnityRenderBuffer) -> *mut ID3D11Resource {
    std::ptr::null_mut()
}

extern "system" fn texture_from_native_texture(texture: UnityTextureID) -> *mut ID3D11Resource {
    std::ptr::null_mut()
}

extern "system" fn rtv_from_render_buffer(
    surface: UnityRenderBuffer,
) -> *mut ID3D11RenderTargetView {
    std::ptr::null_mut()
}

extern "system" fn srv_from_native_texture(
    texture: UnityTextureID,
) -> *mut ID3D11ShaderResourceView {
    std::ptr::null_mut()
}

pub fn initialize_interface(device: ComPtr<d3d11::ID3D11Device>) {
    unsafe {
        crate::intreface::get_unity_interfaces().register_interface::<UnityGraphicsD3D11>(Some(
            Box::new(UnityGraphicsD3D11::new(device)),
        ));
    }
}

struct TesterContext {}

pub fn test_plugin_d3d11<
    FnMain: FnMut(&Window, &mut TesterContext) -> crate::window::LoopResult,
>() {
    crate::intreface::initialize_unity_interfaces();
    crate::graphics::initialize_interface(unity_native_plugin::graphics::GfxRenderer::D3D11);

    unsafe {
        let desc = dxgi::DXGI_SWAP_CHAIN_DESC {
            BufferDesc: dxgitype::DXGI_MODE_DESC{
                Width: 0,
                Height: 0,
                RefreshRate: dxgitype::DXGI_RATIONAL { Numerator: 60, Denominator: 1 },
                Format: dxgiformat::DXGI_FORMAT_R8G8B8A8_UNORM,
                ScanlineOrdering: dxgitype::DXGI_MODE_SCANLINE_ORDER_UNSPECIFIED,
                Scaling: dxgitype::DXGI_MODE_SCALING_UNSPECIFIED
            },
            SampleDesc: dxgitype::DXGI_SAMPLE_DESC{ Count: 0, Quality: 0 },
            BufferUsage: dxgitype::DXGI_USAGE_RENDER_TARGET_OUTPUT,
            BufferCount: 2,
            OutputWindow: (),
            Windowed: 1,
            SwapEffect: dxgi::DXGI_SWAP_EFFECT_DISCARD,
            Flags: dxgi::DXGI_SWAP_CHAIN_FLAG_ALLOW_MODE_SWITCH
        };

        let mut device: *mut d3d11::ID3D11Device = std::ptr::null_mut();
        let mut swap_chain: *mut dxgi::IDXGISwapChain = std::ptr::null_mut();
        let mut feature: d3d11::D3D11_FEATURE = 0;
        let mut dc: *mut d3d11::ID3D11DeviceContext = std::ptr::null_mut();

        let hr = d3d11::D3D11CreateDeviceAndSwapChain(
            std::ptr::null_mut(),
            d3dcommon::D3D_DRIVER_TYPE_HARDWARE,
            std::ptr::null_mut(),
            d3d11::D3D11_CREATE_DEVICE_DEBUG | d3d11::D3D11_CREATE_DEVICE_SINGLETHREADED,
            std::ptr::null_mut(),
            0,
            d3d11::D3D11_SDK_VERSION,
            &desc,
            &mut swap_chain,
            &mut device,
            &mut feature,
            &mut dc,
        );
    }
    crate::intreface::finalize_unity_interfaces();
}

#[test]
fn Test() {
    test_plugin_d3d11();
}
