use unity_native_plugin_sys::*;

pub trait UnityInterface {
    fn get_interface_guid() -> UnityInterfaceGUID;
    fn new(interface: *const IUnityInterface) -> Self;
}

pub struct UnityInterfaces {
    interfaces: *const IUnityInterfaces,
}

impl UnityInterfaces {
    fn get_interface<T: UnityInterface>(&self) -> Option<T> {
        unsafe {
            if let Some(intf) = &(*self.interfaces).GetInterface {
                let r = intf(T::get_interface_guid());
                if !r.is_null() {
                    return Some(T::new(r));
                }
            }
            None
        }
    }
}
