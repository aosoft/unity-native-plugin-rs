use crate::interface::UnityInterfaces;

pub mod enums;
pub mod graphics;
pub mod interface;

#[no_mangle]
#[allow(non_snake_case)]
extern "system" fn UnityPluginLoad(interfaces: Option::<*const unity_native_plugin_sys::IUnityInterfaces> ) {
    interface::UnityInterfaces::set_native_unity_interfaces(interfaces);
}

#[no_mangle]
#[allow(non_snake_case)]
extern "system" fn UnityPluginUnload() {
    interface::UnityInterfaces::set_native_unity_interfaces(None);
}
