use crate::define_unity_interface;
use crate::interface;
use unity_native_plugin_sys::*;

define_unity_interface!(
    UnityGraphicsD3D11,
    IUnityGraphicsD3D11,
    0xAAB37EF87A87D748_u64,
    0xBF76967F07EFB177_u64
);

impl UnityGraphicsD3D11 {
    pub unsafe fn get_device(&self) -> *mut std::ffi::c_void {
        self.get_interface().GetDevice.map_or_else(
            || std::ptr::null_mut(),
            |method| method() as *mut std::ffi::c_void,
        )
    }

    pub unsafe fn texture_from_render_buffer(
        &self,
        buffer: interface::RenderBuffer,
    ) -> *mut std::ffi::c_void {
        self.get_interface().TextureFromRenderBuffer.map_or_else(
            || std::ptr::null_mut(),
            |method| method(buffer) as *mut std::ffi::c_void,
        )
    }

    pub unsafe fn texture_from_natvie_texture(
        &self,
        texture: interface::TextureID,
    ) -> *mut std::ffi::c_void {
        self.get_interface().TextureFromNativeTexture.map_or_else(
            || std::ptr::null_mut(),
            |method| method(texture) as *mut std::ffi::c_void,
        )
    }

    pub unsafe fn rtv_from_render_buffer(
        &self,
        buffer: interface::RenderBuffer,
    ) -> *mut std::ffi::c_void {
        self.get_interface().RTVFromRenderBuffer.map_or_else(
            || std::ptr::null_mut(),
            |method| method(buffer) as *mut std::ffi::c_void,
        )
    }

    pub unsafe fn srv_from_natvie_texture(
        &self,
        texture: interface::TextureID,
    ) -> *mut std::ffi::c_void {
        self.get_interface().SRVFromNativeTexture.map_or_else(
            || std::ptr::null_mut(),
            |method| method(texture) as *mut std::ffi::c_void,
        )
    }
}
