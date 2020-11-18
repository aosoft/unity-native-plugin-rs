use raw_window_handle::HasRawWindowHandle;
use unity_native_plugin::interface::UnityInterface;
use unity_native_plugin_sys::*;
use winapi::shared::{dxgi, dxgiformat, dxgitype, minwindef, winerror};
use winapi::um::{combaseapi, d3d11, d3dcommon, objbase, winnt};
use winit::window::Window;
use wio::com::ComPtr;

pub struct TesterContextGraphicsD3D11 {
    device: ComPtr<d3d11::ID3D11Device>,
    interfaces: IUnityGraphicsD3D11,
    swap_chain: ComPtr<dxgi::IDXGISwapChain>,
    back_buffer: ComPtr<d3d11::ID3D11Texture2D>
}

impl TesterContextGraphicsD3D11 {
    fn new(window: &Window) -> Result<Self, winnt::HRESULT> {
        unsafe {
            let size = window.inner_size();
            let desc = dxgi::DXGI_SWAP_CHAIN_DESC {
                BufferDesc: dxgitype::DXGI_MODE_DESC {
                    Width: size.width,
                    Height: size.height,
                    RefreshRate: dxgitype::DXGI_RATIONAL {
                        Numerator: 60,
                        Denominator: 1,
                    },
                    Format: dxgiformat::DXGI_FORMAT_R8G8B8A8_UNORM,
                    ScanlineOrdering: dxgitype::DXGI_MODE_SCANLINE_ORDER_UNSPECIFIED,
                    Scaling: dxgitype::DXGI_MODE_SCALING_UNSPECIFIED,
                },
                SampleDesc: dxgitype::DXGI_SAMPLE_DESC {
                    Count: 1,
                    Quality: 0,
                },
                BufferUsage: dxgitype::DXGI_USAGE_RENDER_TARGET_OUTPUT,
                BufferCount: 2,
                OutputWindow: match window.raw_window_handle() {
                    raw_window_handle::RawWindowHandle::Windows(h) => h.hwnd,
                    _ => std::ptr::null_mut(),
                } as _,
                Windowed: minwindef::TRUE,
                SwapEffect: dxgi::DXGI_SWAP_EFFECT_DISCARD,
                Flags: dxgi::DXGI_SWAP_CHAIN_FLAG_ALLOW_MODE_SWITCH,
            };

            let mut device: *mut d3d11::ID3D11Device = std::ptr::null_mut();
            let mut swap_chain: *mut dxgi::IDXGISwapChain = std::ptr::null_mut();

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
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );
            if winerror::SUCCEEDED(hr) {
                let mut back_buffer: *mut d3d11::ID3D11Texture2D = std::ptr::null_mut();

                (*swap_chain).GetBuffer(0, &d3d11::IID_ID3D11Texture2D, &mut back_buffer as *mut *mut d3d11::ID3D11Texture2D as _);

                Ok(TesterContextGraphicsD3D11 {
                    device: ComPtr::from_raw(device),
                    swap_chain: ComPtr::from_raw(swap_chain),
                    back_buffer: ComPtr::from_raw(back_buffer),
                    interfaces: IUnityGraphicsD3D11 {
                        GetDevice: Some(get_device),
                        TextureFromRenderBuffer: Some(texture_from_render_buffer),
                        TextureFromNativeTexture: Some(texture_from_native_texture),
                        RTVFromRenderBuffer: Some(rtv_from_render_buffer),
                        SRVFromNativeTexture: Some(srv_from_native_texture),
                    },
                })
            } else {
                Err(hr)
            }
        }
    }

    pub fn device(&self) -> &ComPtr<d3d11::ID3D11Device> {
        &self.device
    }

    pub fn swap_chain(&self) -> &ComPtr<dxgi::IDXGISwapChain> {
        &self.swap_chain
    }

    pub fn back_buffer(&self) -> &ComPtr<d3d11::ID3D11Texture2D> { &self.back_buffer }

    pub fn unity_back_buffer(&self) -> UnityRenderBuffer {
        self.back_buffer.as_raw() as _
    }
}

impl crate::interface::UnityInterfaceBase for TesterContextGraphicsD3D11 {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn get_unity_interface(&self) -> *mut IUnityInterface {
        unsafe { std::mem::transmute::<_, _>(&self.interfaces) }
    }
}

impl crate::interface::UnityInterfaceID for TesterContextGraphicsD3D11 {
    fn get_interface_guid() -> UnityInterfaceGUID {
        unity_native_plugin::d3d11::UnityGraphicsD3D11::get_interface_guid()
    }
}

extern "system" fn get_device() -> *mut ID3D11Device {
    unsafe { crate::interface::get_unity_interface::<TesterContextGraphicsD3D11>().device().as_raw() as _ }
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


pub fn test_plugin_d3d11<
    FnMain: FnMut(&Window, &TesterContextGraphicsD3D11) -> crate::window::LoopResult,
>(
    mut fn_main: FnMain,
    fn_unity_plugin_load: fn(interfaces: &unity_native_plugin::interface::UnityInterfaces),
    fn_unity_plugin_unload: fn(),
) -> Result<(), winnt::HRESULT> {
    unsafe {
        objbase::CoInitialize(std::ptr::null_mut());
    }

    crate::interface::initialize_unity_interfaces();
    crate::graphics::initialize_interface(unity_native_plugin::graphics::GfxRenderer::D3D11);

    crate::window::run_window_app(
        |window| TesterContextGraphicsD3D11::new(window).unwrap(),
        |window, context| {
            let ret = fn_main(window, context);

            unsafe { context.swap_chain().Present(0, 0); }
            ret
        },
        |_window| {},
        fn_unity_plugin_load,
        fn_unity_plugin_unload,
    );

    crate::interface::finalize_unity_interfaces();

    unsafe {
        combaseapi::CoUninitialize();
    }

    Ok(())
}
