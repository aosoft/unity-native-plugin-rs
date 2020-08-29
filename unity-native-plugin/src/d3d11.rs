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
    pub unsafe fn get_device(&self) -> *const () {
        match &(*self.interface).GetDevice {
            Some(intf) => intf() as *const (),
            None => std::ptr::null(),
        }
    }

    pub unsafe fn texture_from_render_buffer(&self, buffer: interface::RenderBuffer) -> *const () {
        match &(*self.interface).TextureFromRenderBuffer {
            Some(intf) => intf(buffer) as *const (),
            None => std::ptr::null(),
        }
    }

    pub unsafe fn texture_from_natvie_texture(&self, texture: interface::TextureID) -> *const () {
        match &(*self.interface).TextureFromNativeTexture {
            Some(intf) => intf(texture) as *const (),
            None => std::ptr::null(),
        }
    }

    pub unsafe fn rtv_from_render_buffer(&self, buffer: interface::RenderBuffer) -> *const () {
        match &(*self.interface).RTVFromRenderBuffer {
            Some(intf) => intf(buffer) as *const (),
            None => std::ptr::null(),
        }
    }

    pub unsafe fn srv_from_natvie_texture(&self, texture: interface::TextureID) -> *const () {
        match &(*self.interface).SRVFromNativeTexture {
            Some(intf) => intf(texture) as *const (),
            None => std::ptr::null(),
        }
    }
}
