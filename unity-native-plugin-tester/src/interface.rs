use std::any::Any;
use std::collections::HashMap;
use std::os::raw::c_ulonglong;
use std::rc::Rc;
use unity_native_plugin_sys::*;

#[derive(Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct InfKey {
    pub high: c_ulonglong,
    pub low: c_ulonglong,
}

pub trait UnityInterfaceBase {
    fn as_any(&self) -> &dyn Any;
    fn get_unity_interface(&self) -> *mut IUnityInterface;
}

pub trait UnityInterfaceID {
    fn get_interface_guid() -> UnityInterfaceGUID;
}

pub struct TesterContextInterfaces {
    map: HashMap<InfKey, Rc<dyn UnityInterfaceBase>>,
    interfaces: IUnityInterfaces,
}

impl TesterContextInterfaces {
    pub fn new() -> Self {
        TesterContextInterfaces {
            map: HashMap::<InfKey, Rc<dyn UnityInterfaceBase>>::new(),
            interfaces: IUnityInterfaces {
                GetInterface: Some(get_interface),
                RegisterInterface: Some(register_interface),
                GetInterfaceSplit: Some(get_interface_split),
                RegisterInterfaceSplit: Some(register_interface_split),
            },
        }
    }

    pub fn interfaces(&self) -> *mut IUnityInterfaces {
        unsafe { std::mem::transmute::<_, _>(&self.interfaces) }
    }

    pub fn get_interface(&self, guid: UnityInterfaceGUID) -> Option<&Rc<dyn UnityInterfaceBase>> {
        self.get_interface_split(guid.m_GUIDHigh, guid.m_GUIDLow)
    }

    pub fn get_interface_split(
        &self,
        high: ::std::os::raw::c_ulonglong,
        low: ::std::os::raw::c_ulonglong,
    ) -> Option<&Rc<dyn UnityInterfaceBase>> {
        self.map.get(&InfKey { high, low })
    }

    pub fn register_interface<T: UnityInterfaceBase + UnityInterfaceID>(
        &mut self,
        interface: Option<Rc<dyn UnityInterfaceBase>>,
    ) {
        let guid = T::get_interface_guid();
        self.register_interface_split(guid.m_GUIDHigh, guid.m_GUIDLow, interface);
    }

    pub fn register_interface_split(
        &mut self,
        high: ::std::os::raw::c_ulonglong,
        low: ::std::os::raw::c_ulonglong,
        interface: Option<Rc<dyn UnityInterfaceBase>>,
    ) {
        if let Some(i) = interface {
            self.map.insert(InfKey { high, low }, i);
        } else {
            self.map.remove(&InfKey { high, low });
        }
    }
}

static mut UNITY_INTERFACES: Option<TesterContextInterfaces> = None;

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
        if let Some(i) = UNITY_INTERFACES
            .as_ref()
            .unwrap()
            .get_interface_split(high, low)
        {
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

pub unsafe fn get_unity_interfaces() -> &'static mut TesterContextInterfaces {
    UNITY_INTERFACES.as_mut().unwrap()
}

pub unsafe fn get_unity_interface<T: UnityInterfaceBase + UnityInterfaceID>() -> &'static T {
    get_unity_interfaces()
        .get_interface(T::get_interface_guid())
        .unwrap()
        .as_any()
        .downcast_ref::<T>()
        .unwrap()
}

pub fn initialize_unity_interfaces() {
    unsafe {
        UNITY_INTERFACES = Some(TesterContextInterfaces::new());
        unity_native_plugin::interface::UnityInterfaces::set_native_unity_interfaces(
            crate::interface::get_unity_interfaces().interfaces(),
        );
    }
}

pub fn finalize_unity_interfaces() {
    unsafe {
        UNITY_INTERFACES = None;
    }
}
