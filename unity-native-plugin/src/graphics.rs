use crate::define_unity_interface;
use crate::interface::UnityInterface;
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
    ReservedCFE = UnityGfxRenderer_kUnityGfxRendererReservedCFE,
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
    IUnityGraphics,
    0x7CBA0A9CA4DDB544_u64,
    0x8C5AD4926EB17B11_u64
);

pub type GraphicsDeviceEventCallback = extern "system" fn(eventType: GfxDeviceEventType);

impl UnityGraphics {
    pub fn renderer(&self) -> GfxRenderer {
        unsafe {
            match self.interface().GetRenderer {
                Some(intf) => std::mem::transmute(intf()),
                None => GfxRenderer::Null,
            }
        }
    }

    pub fn register_device_event_callback(&self, callback: Option<GraphicsDeviceEventCallback>) {
        unsafe {
            if let Some(intf) = self.interface().RegisterDeviceEventCallback {
                intf(std::mem::transmute(callback));
            }
        }
    }

    pub fn unregister_device_event_callback(&self, callback: Option<GraphicsDeviceEventCallback>) {
        unsafe {
            if let Some(intf) = self.interface().UnregisterDeviceEventCallback {
                intf(std::mem::transmute(callback));
            }
        }
    }
}
