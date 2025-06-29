use std::any::Any;
use std::collections::HashMap;
use std::fmt::Debug;
use std::os::raw::c_ulonglong;
use std::rc::Rc;
use std::sync::{Mutex, OnceLock};
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
    map: Mutex<HashMap<InfKey, Rc<dyn UnityInterfaceBase>>>,
    interfaces: IUnityInterfaces,
}

impl Debug for TesterContextInterfaces {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TesterContextInterfaces").finish()
    }
}


// maybe thread safety
unsafe impl Send for TesterContextInterfaces {}
unsafe impl Sync for TesterContextInterfaces {}

impl TesterContextInterfaces {
    pub fn new() -> Self {
        TesterContextInterfaces {
            map: Mutex::new(HashMap::<InfKey, Rc<dyn UnityInterfaceBase>>::new()),
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

    pub fn get_interface(&self, guid: UnityInterfaceGUID) -> Option<Rc<dyn UnityInterfaceBase>> {
        self.get_interface_split(guid.m_GUIDHigh, guid.m_GUIDLow)
    }

    pub fn get_interface_split(
        &self,
        high: ::std::os::raw::c_ulonglong,
        low: ::std::os::raw::c_ulonglong,
    ) -> Option<Rc<dyn UnityInterfaceBase>> {
        self.map.lock().unwrap().get(&InfKey { high, low }).cloned()
    }

    pub fn register_interface<T: UnityInterfaceBase + UnityInterfaceID>(
        &self,
        interface: Option<Rc<dyn UnityInterfaceBase>>,
    ) {
        let guid = T::get_interface_guid();
        self.register_interface_split(guid.m_GUIDHigh, guid.m_GUIDLow, interface);
    }

    pub fn register_interface_split(
        &self,
        high: ::std::os::raw::c_ulonglong,
        low: ::std::os::raw::c_ulonglong,
        interface: Option<Rc<dyn UnityInterfaceBase>>,
    ) {
        if let Some(i) = interface {
            self.map.lock().unwrap().insert(InfKey { high, low }, i);
        } else {
            self.map.lock().unwrap().remove(&InfKey { high, low });
        }
    }
}

static UNITY_INTERFACES: OnceLock<TesterContextInterfaces> = OnceLock::new();

extern "system" fn get_interface(guid: UnityInterfaceGUID) -> *mut IUnityInterface {
    if let Some(i) = UNITY_INTERFACES.get().unwrap().get_interface(guid) {
        i.as_ref().get_unity_interface()
    } else {
        std::ptr::null_mut()
    }
}

extern "system" fn register_interface(_: UnityInterfaceGUID, _: *mut IUnityInterface) {}

extern "system" fn get_interface_split(
    high: ::std::os::raw::c_ulonglong,
    low: ::std::os::raw::c_ulonglong,
) -> *mut IUnityInterface {
    if let Some(i) = UNITY_INTERFACES
        .get()
        .unwrap()
        .get_interface_split(high, low)
    {
        i.as_ref().get_unity_interface()
    } else {
        std::ptr::null_mut()
    }
}

extern "system" fn register_interface_split(
    _: ::std::os::raw::c_ulonglong,
    _: ::std::os::raw::c_ulonglong,
    _: *mut IUnityInterface,
) {
}

pub unsafe fn get_unity_interfaces() -> &'static TesterContextInterfaces {
    UNITY_INTERFACES.get().unwrap()
}

pub unsafe fn get_unity_interface<T: UnityInterfaceBase + UnityInterfaceID + 'static>() -> Rc<T>
{
    unsafe {
        let interface_rc = get_unity_interfaces()
            .get_interface(T::get_interface_guid()).unwrap();

        // Rcの中身をダウンキャストして新しいRcを作成
        let any_ref = interface_rc.as_any();
        if let Some(_) = any_ref.downcast_ref::<T>() {
            // Use Rc::clone to safely create an Rc<T>
            // First, get a raw pointer from the original Rc
            let ptr = Rc::as_ptr(&interface_rc);
            // Successfully downcasted, so cast it safely as type T
            let concrete_ptr = ptr as *const T;
            // Create a new Rc<T> (clone the original Rc to increase the reference count)
            std::mem::forget(interface_rc.clone()); // Increase reference count
            Rc::from_raw(concrete_ptr)
        } else {
            panic!("interface is not T");
        }
    }
}

pub fn initialize_unity_interfaces() {
    unsafe {
        UNITY_INTERFACES.set(TesterContextInterfaces::new()).unwrap();
        unity_native_plugin::interface::UnityInterfaces::set_native_unity_interfaces(
            crate::interface::get_unity_interfaces().interfaces(),
        );
    }
}

pub fn finalize_unity_interfaces() {
    UNITY_INTERFACES.get().unwrap().map.lock().unwrap().clear();
}
