use unity_native_plugin_sys::*;

struct Resources {
    interfaces: IUnityGraphicsD3D11,
}

static mut RESOURCES: Option<Resources> = None;

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

pub fn initialize_d3d11() -> &'static IUnityGraphicsD3D11 {
    unsafe {
        RESOURCES = Some(Resources {
            interfaces: IUnityGraphicsD3D11 {
                GetDevice: Some(get_device),
                TextureFromRenderBuffer: Some(texture_from_render_buffer),
                TextureFromNativeTexture: Some(texture_from_native_texture),
                RTVFromRenderBuffer: Some(rtv_from_render_buffer),
                SRVFromNativeTexture: Some(srv_from_native_texture),
            },
        });

        &RESOURCES.as_ref().unwrap().interfaces
    }
}
