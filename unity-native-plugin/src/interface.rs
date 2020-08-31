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
    pub fn get_unity_interfaces() -> &'static UnityInterfaces {
        unsafe { UNITY_INTERFACES.as_ref().unwrap() }
    }

    pub(crate) fn set_native_unity_interfaces(
        interfaces: *mut unity_native_plugin_sys::IUnityInterfaces,
    ) {
        unsafe {
            UNITY_INTERFACES = if !interfaces.is_null() {
                Some(UnityInterfaces {
                    interfaces: interfaces,
                })
            } else {
                None
            }
        }
    }

    pub fn get_interface<T: UnityInterface>(&self) -> Option<T> {
        unsafe {
            if let Some(intf) = (&*self.interfaces).GetInterface {
                let r = intf(T::get_interface_guid());
                if !r.is_null() {
                    return Some(T::new(r));
                }
            }
            None
        }
    }
}

pub type RenderBuffer = unity_native_plugin_sys::UnityRenderBuffer;
pub type TextureID = unity_native_plugin_sys::UnityTextureID;
