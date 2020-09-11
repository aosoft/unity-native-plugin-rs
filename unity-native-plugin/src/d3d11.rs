use crate::define_unity_interface;
use crate::interface;
use crate::interface::UnityInterface;

define_unity_interface!(
    UnityGraphicsD3D11,
    unity_native_plugin_sys::IUnityGraphicsD3D11,
    0xAAB37EF87A87D748_u64,
    0xBF76967F07EFB177_u64
);

pub type ComPtr = *mut std::ffi::c_void;

impl UnityGraphicsD3D11 {
    pub unsafe fn device(&self) -> ComPtr {
        self.interface().GetDevice.expect("GetDevice")() as ComPtr
    }

    pub unsafe fn texture_from_render_buffer(&self, buffer: interface::RenderBuffer) -> ComPtr {
        self.interface()
            .TextureFromRenderBuffer
            .expect("TextureFromRenderBuffer")(buffer) as ComPtr
    }

    pub unsafe fn texture_from_natvie_texture(&self, texture: interface::TextureID) -> ComPtr {
        self.interface()
            .TextureFromNativeTexture
            .expect("TextureFromNativeTexture")(texture) as ComPtr
    }

    pub unsafe fn rtv_from_render_buffer(&self, buffer: interface::RenderBuffer) -> ComPtr {
        self.interface()
            .RTVFromRenderBuffer
            .expect("RTVFromRenderBuffer")(buffer) as ComPtr
    }

    pub unsafe fn srv_from_natvie_texture(&self, texture: interface::TextureID) -> ComPtr {
        self.interface()
            .SRVFromNativeTexture
            .expect("SRVFromNativeTexture")(texture) as ComPtr
    }
}
