use unity_native_plugin_sys::*;

struct UnityGraphicsD3D11 {
    interfaces: IUnityGraphicsD3D11,
}

impl UnityGraphicsD3D11 {
    pub fn new() -> UnityGraphicsD3D11 {
        UnityGraphicsD3D11 {
            interfaces: IUnityGraphicsD3D11 {
                GetDevice: Some(get_device),
                TextureFromRenderBuffer: Some(texture_from_render_buffer),
                TextureFromNativeTexture: Some(texture_from_native_texture),
                RTVFromRenderBuffer: Some(rtv_from_render_buffer),
                SRVFromNativeTexture: Some(srv_from_native_texture),
            }
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

pub fn initialize_interface()  {
    unsafe {
        crate::intreface::get_unity_interfaces()
            .register_interface::<UnityGraphicsD3D11>(Some(Box::new(UnityGraphicsD3D11::new())));
    }
}
