use std::collections::HashMap;
use std::os::raw::c_ulonglong;
use unity_native_plugin_sys::*;

#[derive(Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct InfKey {
    pub high: c_ulonglong,
    pub low: c_ulonglong,
}

pub trait UnityInterfaceBase {
    fn get_unity_interface(&self) -> *mut IUnityInterface;
}

pub struct UnityInterfaces {
    map: HashMap<InfKey, Box<dyn UnityInterfaceBase>>,
}

impl UnityInterfaces {
    pub fn new() -> Self {
        UnityInterfaces {
            map: HashMap::<InfKey, Box<dyn UnityInterfaceBase>>::new(),
        }
    }

    pub fn get_interface(&self, guid: UnityInterfaceGUID) -> Option<&Box<dyn UnityInterfaceBase>> {
        self.get_interface_split(guid.m_GUIDHigh, guid.m_GUIDLow)
    }

    pub fn get_interface_split(
        &self,
        high: ::std::os::raw::c_ulonglong,
        low: ::std::os::raw::c_ulonglong,
    ) -> Option<&Box<dyn UnityInterfaceBase>> {
        self.map.get(&InfKey { high, low })
    }

    pub fn register_interface(
        &mut self,
        guid: UnityInterfaceGUID,
        interface: Option<Box<dyn UnityInterfaceBase>>,
    ) {
        self.register_interface_split(guid.m_GUIDHigh, guid.m_GUIDLow, interface);
    }

    pub fn register_interface_split(
        &mut self,
        high: ::std::os::raw::c_ulonglong,
        low: ::std::os::raw::c_ulonglong,
        interface: Option<Box<dyn UnityInterfaceBase>>,
    ) {
        if let Some(i) = interface {
            self.map.insert(InfKey { high, low }, i);
        } else {
            self.map.remove(&InfKey { high, low });
        }
    }
}

static INTERFACES: IUnityInterfaces = IUnityInterfaces {
    GetInterface: Some(get_interface),
    RegisterInterface: Some(register_interface),
    GetInterfaceSplit: Some(get_interface_split),
    RegisterInterfaceSplit: Some(register_interface_split),
};

static mut UNITY_INTERFACES: Option<UnityInterfaces> = None;

extern "system" fn get_interface(guid: UnityInterfaceGUID) -> *mut IUnityInterface {
    unsafe {
        if let Some(i) = UNITY_INTERFACES.as_ref().unwrap().get_interface(guid) {
            i.as_ref().get_unity_interface()
        } else {
            std::ptr::null_mut()
        }
    }
}

extern "system" fn register_interface(_: UnityInterfaceGUID, _: *mut IUnityInterface) {}

extern "system" fn get_interface_split(
    high: ::std::os::raw::c_ulonglong,
    low: ::std::os::raw::c_ulonglong,
) -> *mut IUnityInterface {
    unsafe {
        if let Some(i) = UNITY_INTERFACES.as_ref().unwrap().get_interface_split(high, low) {
            i.as_ref().get_unity_interface()
        } else {
            std::ptr::null_mut()
        }
    }
}

extern "system" fn register_interface_split(
    _: ::std::os::raw::c_ulonglong,
    _: ::std::os::raw::c_ulonglong,
    _: *mut IUnityInterface,
) {
}

pub fn get_unity_interfaces() -> &'static IUnityInterfaces {
    &INTERFACES
}
