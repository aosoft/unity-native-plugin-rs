use crate::interface::UnityInterface;
use std::os::raw::c_int;
use unity_native_plugin_sys::*;

pub type RenderBuffer = unity_native_plugin_sys::UnityRenderBuffer;
pub type TextureID = unity_native_plugin_sys::UnityTextureID;
pub type RenderingEvent = unity_native_plugin_sys::UnityRenderingEvent;
pub type RenderingEventAndData = unity_native_plugin_sys::UnityRenderingEventAndData;

#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum GfxRenderer {
    D3D11 = UnityGfxRenderer_kUnityGfxRendererD3D11,
    Null = UnityGfxRenderer_kUnityGfxRendererNull,
    OpenGLES20 = 8, // OpenGL ES 2.0, removed
    OpenGLES30 = UnityGfxRenderer_kUnityGfxRendererOpenGLES30,
    PS4 = UnityGfxRenderer_kUnityGfxRendererPS4,
    XboxOne = UnityGfxRenderer_kUnityGfxRendererXboxOne,
    Metal = UnityGfxRenderer_kUnityGfxRendererMetal,
    OpenGLCore = UnityGfxRenderer_kUnityGfxRendererOpenGLCore,
    D3D12 = UnityGfxRenderer_kUnityGfxRendererD3D12,
    Vulkan = UnityGfxRenderer_kUnityGfxRendererVulkan,
    Nvn = UnityGfxRenderer_kUnityGfxRendererNvn,
    XboxOneD3D12 = UnityGfxRenderer_kUnityGfxRendererXboxOneD3D12,
    GameCoreXboxOne = UnityGfxRenderer_kUnityGfxRendererGameCoreXboxOne,
    GameCoreXboxSeries = UnityGfxRenderer_kUnityGfxRendererGameCoreXboxSeries,
    PS5 = UnityGfxRenderer_kUnityGfxRendererPS5,
    PS5NGGC = UnityGfxRenderer_kUnityGfxRendererPS5NGGC,
    Nvn2 = UnityGfxRenderer_kUnityGfxRendererNvn2,
}

#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum GfxDeviceEventType {
    Initialize = UnityGfxDeviceEventType_kUnityGfxDeviceEventInitialize,
    Shutdown = UnityGfxDeviceEventType_kUnityGfxDeviceEventShutdown,
    BeforeReset = UnityGfxDeviceEventType_kUnityGfxDeviceEventBeforeReset,
    AfterReset = UnityGfxDeviceEventType_kUnityGfxDeviceEventAfterReset,
}

define_unity_interface!(
    UnityGraphics,
    unity_native_plugin_sys::IUnityGraphics,
    0x7CBA0A9CA4DDB544_u64,
    0x8C5AD4926EB17B11_u64
);

pub type GraphicsDeviceEventCallback = extern "system" fn(event_type: GfxDeviceEventType);

pub trait IUnityGraphics {
    fn renderer(&self) -> GfxRenderer;
    fn register_device_event_callback(&self, callback: Option<GraphicsDeviceEventCallback>);
    fn unregister_device_event_callback(&self, callback: Option<GraphicsDeviceEventCallback>);
    fn reserve_event_id_range(&self, count: c_int) -> c_int;
}

impl IUnityGraphics for UnityGraphics {
    fn renderer(&self) -> GfxRenderer {
        unsafe {
            match self.interface().GetRenderer {
                Some(intf) => std::mem::transmute::<UnityGfxRenderer, GfxRenderer>(intf()),
                None => GfxRenderer::Null,
            }
        }
    }

    fn register_device_event_callback(&self, callback: Option<GraphicsDeviceEventCallback>) {
        unsafe {
            if let Some(intf) = self.interface().RegisterDeviceEventCallback {
                intf(std::mem::transmute::<
                    Option<GraphicsDeviceEventCallback>,
                    IUnityGraphicsDeviceEventCallback,
                >(callback));
            }
        }
    }

    fn unregister_device_event_callback(&self, callback: Option<GraphicsDeviceEventCallback>) {
        unsafe {
            if let Some(intf) = self.interface().UnregisterDeviceEventCallback {
                intf(std::mem::transmute::<
                    Option<GraphicsDeviceEventCallback>,
                    IUnityGraphicsDeviceEventCallback,
                >(callback));
            }
        }
    }

    fn reserve_event_id_range(&self, count: c_int) -> c_int {
        unsafe {
            self.interface()
                .ReserveEventIDRange
                .expect("ReserveEventIDRange is missing")(count)
        }
    }
}
