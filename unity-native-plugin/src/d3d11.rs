use crate::define_unity_interface;
use crate::interface;
use unity_native_plugin_sys::*;
use std::ffi::c_void;

define_unity_interface!(
    UnityGraphicsD3D11,
    IUnityGraphicsD3D11,
    0xAAB37EF87A87D748_u64,
    0xBF76967F07EFB177_u64
);

impl UnityGraphicsD3D11 {
    pub unsafe fn get_device(&self) -> *mut c_void {
        self.get_interface().GetDevice.expect("GetDevice")() as *mut c_void
    }

    pub unsafe fn texture_from_render_buffer(
        &self,
        buffer: interface::RenderBuffer,
    ) -> *mut c_void {
        self.get_interface()
            .TextureFromRenderBuffer
            .expect("TextureFromRenderBuffer")(buffer) as *mut c_void
    }

    pub unsafe fn texture_from_natvie_texture(
        &self,
        texture: interface::TextureID,
    ) -> *mut c_void {
        self.get_interface()
            .TextureFromNativeTexture
            .expect("TextureFromNativeTexture")(texture) as *mut c_void
    }

    pub unsafe fn rtv_from_render_buffer(
        &self,
        buffer: interface::RenderBuffer,
    ) -> *mut c_void {
        self.get_interface()
            .RTVFromRenderBuffer
            .expect("RTVFromRenderBuffer")(buffer) as *mut c_void
    }

    pub unsafe fn srv_from_natvie_texture(
        &self,
        texture: interface::TextureID,
    ) -> *mut c_void {
        self.get_interface()
            .SRVFromNativeTexture
            .expect("SRVFromNativeTexture")(texture) as *mut c_void
    }
}
