use winapi::shared::dxgiformat;
use winapi::um::{d3d11, unknwnbase::IUnknown};
use wio::com::ComPtr;

unity_native_plugin::unity_native_plugin_entry_point! {
    fn unity_plugin_load(interfaces: &unity_native_plugin::interface::UnityInterfaces) {
    }
    fn unity_plugin_unload() {
    }
}

#[no_mangle]
#[allow(non_snake_case)]
extern "system" fn FillTexture(unity_texture: *mut IUnknown, x: f32, y: f32, z: f32, w: f32) {
    unsafe {
        if unity_texture.is_null() {
            return;
        }
        let texture = ComPtr::<IUnknown>::from_raw(unity_texture);
        (&*unity_texture).AddRef();
        let texture = match texture.cast::<d3d11::ID3D11Texture2D>() {
            Ok(t) => t,
            Err(_) => return,
        };

        let device = match unity_native_plugin::interface::UnityInterfaces::get()
            .interface::<unity_native_plugin::d3d11::UnityGraphicsD3D11>()
        {
            Some(t) => t,
            None => return,
        }
        .device() as *mut d3d11::ID3D11Device;

        let mut dc: *mut d3d11::ID3D11DeviceContext = std::ptr::null_mut();
        (&*device).GetImmediateContext(&mut dc as *mut *mut _);
        let dc = ComPtr::from_raw(dc);

        let mut desc: d3d11::D3D11_RENDER_TARGET_VIEW_DESC = std::mem::zeroed();
        desc.Format = dxgiformat::DXGI_FORMAT_R8G8B8A8_UNORM;
        desc.ViewDimension = d3d11::D3D11_RTV_DIMENSION_TEXTURE2D;
        *desc.u.Texture2D_mut() = d3d11::D3D11_TEX2D_RTV { MipSlice: 0 };
        let mut rtv: *mut d3d11::ID3D11RenderTargetView = std::ptr::null_mut();
        if (&*device).CreateRenderTargetView(
            texture.as_raw() as *mut d3d11::ID3D11Resource,
            &desc,
            &mut rtv as *mut *mut _,
        ) < 0
        {
            return;
        }
        let rtv = ComPtr::from_raw(rtv);
        (&*dc).ClearRenderTargetView(rtv.as_raw(), &[x, y, z, w]);
    }
}

#[test]
fn test() {
    let instant = std::time::Instant::now();
    unity_native_plugin_tester::d3d11::test_plugin_d3d11(
        |window, context| {
            let n = (instant.elapsed().as_millis() % 1000) as f32 / 1000.0;
            FillTexture(context.back_buffer().as_raw() as _, 0.0, 0.0, n, 1.0);
            unity_native_plugin_tester::window::LoopResult::Continue
        },
        unity_plugin_load,
        unity_plugin_unload,
    ).unwrap();
}
