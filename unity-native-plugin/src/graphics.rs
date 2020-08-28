use unity_native_plugin_sys::*;


pub type UnityGraphicsDeviceEventCallback = extern "system" fn(eventType: crate::enums::xDeviceEventType);

pub struct UnityGraphics {
    interface: *const IUnityGraphics,
}

impl crate::interface::UnityInterface for UnityGraphics {
    fn get_interface_guid() -> UnityInterfaceGUID {
        UnityInterfaceGUID::new(0x7CBA0A9CA4DDB544_u64, 0x8C5AD4926EB17B11_u64)
    }

    fn new(interface: *const IUnityInterface) -> Self {
        UnityGraphics{ interface: interface as *const IUnityGraphics }
    }
}

impl UnityGraphics {
    pub fn get_renderer(&self) -> crate::enums::GfxRenderer {
        unsafe {
            match &(*self.interface).GetRenderer {
                Some(intf) => intf() as crate::enums::GfxRenderer,
                None => crate::enums::GfxRenderer::Null
            }
        }
    }

    pub fn register_device_event_callback(&self, callback: Option::<UnityGraphicsDeviceEventCallback>) {
        unsafe {
            if let Some(intf) = &(*self.interface).RegisterDeviceEventCallback {
                intf(std::mem::transmute(intf));
            }
        }
    }

    pub fn unregister_device_event_callback(&self, callback: Option::<UnityGraphicsDeviceEventCallback>) {
        unsafe {
            if let Some(intf) = &(*self.interface).UnRegisterDeviceEventCallback {
                intf(std::mem::transmute(intf));
            }
        }
    }
}