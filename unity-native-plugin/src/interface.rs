use unity_native_plugin_sys::*;

pub trait UnityInterface {
    fn get_interface_guid() -> UnityInterfaceGUID;
    fn new(interface: *const IUnityInterface) -> Self;
}

static mut UNITY_INTERFACES: Option<UnityInterfaces> = None;

pub struct UnityInterfaces {
    interfaces: *mut unity_native_plugin_sys::IUnityInterfaces,
}

impl UnityInterfaces {
    pub fn get() -> &'static UnityInterfaces {
        unsafe { UNITY_INTERFACES.as_ref().unwrap() }
    }

    pub fn set_native_unity_interfaces(interfaces: *mut unity_native_plugin_sys::IUnityInterfaces) {
        unsafe {
            UNITY_INTERFACES = if !interfaces.is_null() {
                Some(UnityInterfaces { interfaces })
            } else {
                None
            }
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
