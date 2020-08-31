use crate::define_unity_interface;
use unity_native_plugin_sys::*;

define_unity_interface!(
    UnityGraphics,
    IUnityGraphics,
    0x7CBA0A9CA4DDB544_u64,
    0x8C5AD4926EB17B11_u64
);

pub type UnityGraphicsDeviceEventCallback =
    extern "system" fn(eventType: crate::enums::GfxDeviceEventType);

impl UnityGraphics {
    pub fn get_renderer(&self) -> crate::enums::GfxRenderer {
        unsafe {
            match (&*self.interface).GetRenderer {
                Some(intf) => std::mem::transmute(intf()),
                None => crate::enums::GfxRenderer::Null,
            }
        }
    }

    pub fn register_device_event_callback(
        &self,
        callback: Option<UnityGraphicsDeviceEventCallback>,
    ) {
        unsafe {
            if let Some(intf) = (&*self.interface).RegisterDeviceEventCallback {
                intf(std::mem::transmute(callback));
            }
        }
    }

    pub fn unregister_device_event_callback(
        &self,
        callback: Option<UnityGraphicsDeviceEventCallback>,
    ) {
        unsafe {
            if let Some(intf) = (&*self.interface).UnregisterDeviceEventCallback {
                intf(std::mem::transmute(callback));
            }
        }
    }
}
