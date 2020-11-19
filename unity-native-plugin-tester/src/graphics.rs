use unity_native_plugin::interface::UnityInterface;
use unity_native_plugin_sys::*;

struct TesterContextGraphics {
    renderer: UnityGfxRenderer,
    interfaces: IUnityGraphics,
}

impl TesterContextGraphics {
    pub fn new(renderer: unity_native_plugin::graphics::GfxRenderer) -> Self {
        TesterContextGraphics {
            renderer: renderer as _,
            interfaces: IUnityGraphics {
                GetRenderer: Some(get_renderer),
                RegisterDeviceEventCallback: Some(register_device_event_callback),
                UnregisterDeviceEventCallback: Some(unregister_device_event_callback),
                ReserveEventIDRange: Some(reserve_event_id_range),
            },
        }
    }

    pub fn renderer(&self) -> UnityGfxRenderer {
        self.renderer
    }
}

impl crate::interface::UnityInterfaceBase for TesterContextGraphics {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn get_unity_interface(&self) -> *mut IUnityInterface {
        unsafe { std::mem::transmute::<_, _>(&self.interfaces) }
    }
}

impl crate::interface::UnityInterfaceID for TesterContextGraphics {
    fn get_interface_guid() -> UnityInterfaceGUID {
        unity_native_plugin::graphics::UnityGraphics::get_interface_guid()
    }
}

extern "system" fn get_renderer() -> UnityGfxRenderer {
    unsafe { crate::interface::get_unity_interface::<TesterContextGraphics>().renderer() }
}

extern "system" fn register_device_event_callback(_: IUnityGraphicsDeviceEventCallback) {}

extern "system" fn unregister_device_event_callback(_: IUnityGraphicsDeviceEventCallback) {}

extern "system" fn reserve_event_id_range(_: ::std::os::raw::c_int) -> ::std::os::raw::c_int {
    0
}

pub fn initialize_interface(renderer: unity_native_plugin::graphics::GfxRenderer) {
    unsafe {
        crate::interface::get_unity_interfaces().register_interface::<TesterContextGraphics>(Some(
            std::rc::Rc::new(TesterContextGraphics::new(renderer)),
        ));
    }
}

#[test]
fn register_graphics() {
    crate::interface::initialize_unity_interfaces();
    crate::graphics::initialize_interface(unity_native_plugin::graphics::GfxRenderer::D3D11);

    assert_eq!(
        unity_native_plugin::graphics::GfxRenderer::D3D11,
        unity_native_plugin::interface::UnityInterfaces::get()
            .interface::<unity_native_plugin::graphics::UnityGraphics>()
            .unwrap()
            .renderer()
    );

    crate::interface::finalize_unity_interfaces();
}
