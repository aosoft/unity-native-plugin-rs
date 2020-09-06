#[cfg(feature = "d3d11")]
pub mod d3d11;

#[cfg(feature = "d3d12")]
pub mod d3d12;

#[cfg(feature = "vulkan")]
pub mod vulkan;

pub mod enums;
pub mod graphics;
pub mod interface;
pub type IUnityInterfaces = unity_native_plugin_sys::IUnityInterfaces;

#[macro_export]
macro_rules! unity_native_plugin_entry_point {
    {fn $method_load:ident($p:ident : $t:ty) $body_load:block
     fn $method_unload:ident() $body_unload:block} => {
        #[allow(unused_variables)]
        fn $method_load($p: $t) $body_load
        fn $method_unload() $body_unload

        #[no_mangle]
        #[allow(non_snake_case)]
        extern "system" fn UnityPluginLoad(
            interfaces: *mut unity_native_plugin::IUnityInterfaces,
        ) {
            unity_native_plugin::interface::UnityInterfaces::set_native_unity_interfaces(interfaces);
            $method_load(unity_native_plugin::interface::UnityInterfaces::get_unity_interfaces());
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        extern "system" fn UnityPluginUnload() {
            $method_unload();
            unity_native_plugin::interface::UnityInterfaces::set_native_unity_interfaces(std::ptr::null_mut());
        }
    }
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

        impl $s {
            #[allow(dead_code)]
            #[inline]
            fn get_interface(&self) -> &$intf {
                unsafe { &*self.interface }
            }
        }
    }
}
