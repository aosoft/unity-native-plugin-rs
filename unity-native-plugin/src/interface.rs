use unity_native_plugin_sys::*;
use std::sync::OnceLock;

pub trait UnityInterface {
    fn get_interface_guid() -> UnityInterfaceGUID;
    fn new(interface: *const IUnityInterface) -> Self;
}

static UNITY_INTERFACES: OnceLock<UnityInterfaces> = OnceLock::new();

pub struct UnityInterfaces {
    interfaces: *mut unity_native_plugin_sys::IUnityInterfaces,
}

// maybe thread safety
unsafe impl Send for UnityInterfaces {}
unsafe impl Sync for UnityInterfaces {}


impl UnityInterfaces {
    pub fn get() -> &'static UnityInterfaces {
        UNITY_INTERFACES.get().unwrap()
    }

    pub fn set_native_unity_interfaces(interfaces: *mut unity_native_plugin_sys::IUnityInterfaces) {
        if !interfaces.is_null() {
            let unity_interfaces = UnityInterfaces { interfaces };
            let _ = UNITY_INTERFACES.set(unity_interfaces);
        }
    }

    pub fn interface<T: UnityInterface>(&self) -> Option<T> {
        unsafe {
            if let Some(intf) = (&*self.interfaces).GetInterfaceSplit {
                let guid = T::get_interface_guid();
                let r = intf(guid.m_GUIDHigh, guid.m_GUIDLow);
                if !r.is_null() {
                    return Some(T::new(r));
                }
            }
            None
        }
    }
}
