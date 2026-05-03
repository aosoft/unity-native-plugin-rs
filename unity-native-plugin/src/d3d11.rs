use crate::graphics;
use crate::interface::UnityInterface;
use crate::windows::ComPtr;

define_unity_interface!(
    UnityGraphicsD3D11,
    unity_native_plugin_sys::IUnityGraphicsD3D11,
    0xAAB37EF87A87D748_u64,
    0xBF76967F07EFB177_u64
);

pub trait UnityGraphicsD3D11Interface {
    unsafe fn device(&self) -> ComPtr;
    unsafe fn texture_from_render_buffer(&self, buffer: graphics::RenderBuffer) -> ComPtr;
    unsafe fn texture_from_native_texture(&self, texture: graphics::TextureID) -> ComPtr;
    unsafe fn rtv_from_render_buffer(&self, buffer: graphics::RenderBuffer) -> ComPtr;
    unsafe fn srv_from_native_texture(&self, texture: graphics::TextureID) -> ComPtr;
    unsafe fn swap_chain(&self) -> ComPtr;
    fn sync_interval(&self) -> u32;
    fn present_flags(&self) -> u32;
}

macro_rules! impl_d3d11 {
    ($intf:ty) => {
        impl UnityGraphicsD3D11Interface for $intf {
            unsafe fn device(&self) -> ComPtr {
                unsafe { self.interface().GetDevice.expect("GetDevice")() as ComPtr }
            }

            unsafe fn texture_from_render_buffer(&self, buffer: graphics::RenderBuffer) -> ComPtr {
                unsafe {
                    self.interface()
                        .TextureFromRenderBuffer
                        .expect("TextureFromRenderBuffer")(buffer) as ComPtr
                }
            }

            unsafe fn texture_from_native_texture(&self, texture: graphics::TextureID) -> ComPtr {
                unsafe {
                    self.interface()
                        .TextureFromNativeTexture
                        .expect("TextureFromNativeTexture")(texture) as ComPtr
                }
            }

            unsafe fn rtv_from_render_buffer(&self, buffer: graphics::RenderBuffer) -> ComPtr {
                unsafe {
                    self.interface()
                        .RTVFromRenderBuffer
                        .expect("RTVFromRenderBuffer")(buffer) as ComPtr
                }
            }

            unsafe fn srv_from_native_texture(&self, texture: graphics::TextureID) -> ComPtr {
                unsafe {
                    self.interface()
                        .SRVFromNativeTexture
                        .expect("SRVFromNativeTexture")(texture) as ComPtr
                }
            }

            unsafe fn swap_chain(&self) -> ComPtr {
                unsafe { self.interface().GetSwapChain.expect("GetSwapChain")() as ComPtr }
            }

            fn sync_interval(&self) -> u32 {
                unsafe { self.interface().GetSyncInterval.expect("GetSyncInterval")() }
            }

            fn present_flags(&self) -> u32 {
                unsafe { self.interface().GetPresentFlags.expect("GetPresentFlags")() }
            }
        }
    };
}

impl_d3d11!(UnityGraphicsD3D11);
