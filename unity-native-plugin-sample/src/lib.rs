use unity_native_plugin::UnityGraphicsD3D11;
use winapi::shared::dxgiformat;
use winapi::um::{d3d11, unknwnbase::IUnknown};
use wio::com::ComPtr;

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

        let device = match unity_native_plugin::UnityInterfaces::get_unity_interfaces()
            .get_interface::<UnityGraphicsD3D11>()
        {
            Some(t) => t,
            None => return,
        }
        .get_device() as *mut d3d11::ID3D11Device;

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
