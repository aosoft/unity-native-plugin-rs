#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::all)]
#![allow(clippy::pedantic)]
#![allow(unnecessary_transmutes)]
#![allow(unsafe_op_in_unsafe_fn)]

include!("plugin_api_core.rs");

#[cfg(windows)]
include!("plugin_api_windows.rs");

#[cfg(target_vendor = "apple")]
include!("plugin_api_apple.rs");

#[cfg(feature = "vulkan")]
include!("plugin_api_vulkan.rs");

impl UnityInterfaceGUID {
    pub fn new(
        high: ::std::os::raw::c_ulonglong,
        low: ::std::os::raw::c_ulonglong,
    ) -> UnityInterfaceGUID {
        UnityInterfaceGUID {
            m_GUIDHigh: high,
            m_GUIDLow: low,
        }
    }
}
