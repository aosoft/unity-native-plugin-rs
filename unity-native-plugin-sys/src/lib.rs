#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!("plugin_api.rs");

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
