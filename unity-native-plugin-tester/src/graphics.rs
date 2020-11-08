use unity_native_plugin_sys::*;

struct Resources {
    pub renderer: UnityGfxRenderer,
    pub interfaces: IUnityGraphics,
}

static mut RESOURCES: Option<Resources> = None;

extern "system" fn get_renderer() -> UnityGfxRenderer {
    unsafe { RESOURCES.as_ref().unwrap().renderer }
}

extern "system" fn register_device_event_callback(_: IUnityGraphicsDeviceEventCallback) {}

extern "system" fn unregister_device_event_callback(_: IUnityGraphicsDeviceEventCallback) {}

extern "system" fn reserve_event_id_range(_: ::std::os::raw::c_int) -> ::std::os::raw::c_int {
    0
}

pub fn initialize_interface(
    renderer: unity_native_plugin::graphics::GfxRenderer,
) {
    unsafe {
        RESOURCES = Some(Resources {
            renderer: renderer as _,
            interfaces: IUnityGraphics {
                GetRenderer: Some(get_renderer),
                RegisterDeviceEventCallback: Some(register_device_event_callback),
                UnregisterDeviceEventCallback: Some(unregister_device_event_callback),
                ReserveEventIDRange: Some(reserve_event_id_range),
            },
        });

    }
}
