#[cfg(feature = "d3d11")]
pub mod d3d11;

pub mod enums;
pub mod graphics;
pub mod interface;

#[no_mangle]
#[allow(non_snake_case)]
extern "system" fn UnityPluginLoad(
    interfaces: *mut unity_native_plugin_sys::IUnityInterfaces,
) {
    interface::UnityInterfaces::set_native_unity_interfaces(interfaces);
}

#[no_mangle]
#[allow(non_snake_case)]
extern "system" fn UnityPluginUnload() {
    interface::UnityInterfaces::set_native_unity_interfaces(std::ptr::null_mut());
}

#[macro_export]
macro_rules! define_unity_interface {
    ($s:ident, $intf:ident, $guid_high:expr, $guid_low:expr) => {
        pub struct $s {
            interface: *const $intf,
        }

        impl crate::interface::UnityInterface for $s {
            fn get_interface_guid() -> unity_native_plugin_sys::UnityInterfaceGUID {
                unity_native_plugin_sys::UnityInterfaceGUID::new($guid_high, $guid_low)
            }

            fn new(interface: *const IUnityInterface) -> Self {
                $s {
                    interface: interface as *const $intf,
                }
            }
        }
    };
}
