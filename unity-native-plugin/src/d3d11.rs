use crate::define_unity_interface;
use crate::graphics;
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

    pub unsafe fn texture_from_render_buffer(&self, buffer: graphics::RenderBuffer) -> ComPtr {
        self.interface()
            .TextureFromRenderBuffer
            .expect("TextureFromRenderBuffer")(buffer) as ComPtr
    }

    pub unsafe fn texture_from_natvie_texture(&self, texture: graphics::TextureID) -> ComPtr {
        self.interface()
            .TextureFromNativeTexture
            .expect("TextureFromNativeTexture")(texture) as ComPtr
    }

    pub unsafe fn rtv_from_render_buffer(&self, buffer: graphics::RenderBuffer) -> ComPtr {
        self.interface()
            .RTVFromRenderBuffer
            .expect("RTVFromRenderBuffer")(buffer) as ComPtr
    }

    pub unsafe fn srv_from_natvie_texture(&self, texture: graphics::TextureID) -> ComPtr {
        self.interface()
            .SRVFromNativeTexture
            .expect("SRVFromNativeTexture")(texture) as ComPtr
    }

    pub unsafe fn swap_chain(&self) -> ComPtr {
        self.interface().GetSwapChain.expect("GetSwapChain")() as ComPtr
    }

    pub fn sync_interval(&self) -> u32 {
        unsafe {
            self.interface()
                .GetSyncInterval
                .expect("GetSyncInterval")()
        }
    }

    pub fn present_flags(&self) -> u32 {
        unsafe {
            self.interface()
                .GetPresentFlags
                .expect("GetPresentFlags")()
        }
    }
}
