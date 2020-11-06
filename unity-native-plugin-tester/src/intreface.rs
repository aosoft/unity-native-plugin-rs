use std::collections::HashMap;
use std::os::raw::c_ulonglong;
use unity_native_plugin_sys::*;

#[derive(Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct InfKey {
    pub high: c_ulonglong,
    pub low: c_ulonglong,
}

type InfMap = HashMap<InfKey, *mut IUnityInterface>;

static mut INTERFACES: Option<InfMap> = None;

extern "system" fn get_interface(guid: UnityInterfaceGUID) -> *mut IUnityInterface {
    get_interface_split(guid.m_GUIDHigh, guid.m_GUIDLow)
}

extern "system" fn register_interface(guid: UnityInterfaceGUID, ptr: *mut IUnityInterface) {
    register_interface_split(guid.m_GUIDHigh, guid.m_GUIDLow, ptr);
}

extern "system" fn get_interface_split(
    high: ::std::os::raw::c_ulonglong,
    low: ::std::os::raw::c_ulonglong,
) -> *mut IUnityInterface {
    unsafe {
        if let Some(infs) = &INTERFACES {
            *infs.get(&InfKey { high, low }).unwrap_or(&std::ptr::null_mut())
        } else {
            std::ptr::null_mut()
        }
    }
}

extern "system" fn register_interface_split(
    high: ::std::os::raw::c_ulonglong,
    low: ::std::os::raw::c_ulonglong,
    ptr: *mut IUnityInterface,
) {
    unsafe {
        if let Some(infs) = &mut INTERFACES {
            if ptr.is_null() {
                infs.remove(&InfKey { high, low });
            } else {
                infs.insert(InfKey { high, low }, ptr);
            }
        }
    }
}

pub fn get_unity_interfaces() -> IUnityInterfaces {
    unsafe {
        if INTERFACES.is_none() {
            INTERFACES = Some(InfMap::new());
        }
    }

    IUnityInterfaces {
        GetInterface: Some(get_interface),
        RegisterInterface: Some(register_interface),
        GetInterfaceSplit: Some(get_interface_split),
        RegisterInterfaceSplit: Some(register_interface_split),
    }
}
