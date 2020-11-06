use unity_native_plugin_sys::*;

static mut GFX_RENDERER: UnityGfxRenderer = UnityGfxRenderer_kUnityGfxRendererNull;

extern "system" fn get_renderer() -> UnityGfxRenderer {
    unsafe { GFX_RENDERER }
}

extern "system" fn register_device_event_callback(_: IUnityGraphicsDeviceEventCallback) {}

extern "system" fn unregister_device_event_callback(_: IUnityGraphicsDeviceEventCallback) {}

extern "system" fn reserve_event_id_range(_: ::std::os::raw::c_int) -> ::std::os::raw::c_int {
    0
}

pub fn set_gfx_renderer(renderer: unity_native_plugin::graphics::GfxRenderer) {
    unsafe {
        GFX_RENDERER = renderer as _;
    }
}

pub fn get_unity_graphics() -> IUnityGraphics {
    IUnityGraphics {
        GetRenderer: Some(get_renderer),
        RegisterDeviceEventCallback: Some(register_device_event_callback),
        UnregisterDeviceEventCallback: Some(unregister_device_event_callback),
        ReserveEventIDRange: Some(reserve_event_id_range),
    }
}
