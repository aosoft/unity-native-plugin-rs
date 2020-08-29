use unity_native_plugin_sys::*;

pub trait UnityInterface {
    fn get_interface_guid() -> UnityInterfaceGUID;
    fn new(interface: *const IUnityInterface) -> Self;
}

static mut UNITY_INTERFACES: Option<UnityInterfaces> = None;

pub struct UnityInterfaces {
    interfaces: *const IUnityInterfaces,
}

impl UnityInterfaces {
    pub fn get_unity_interfaces() -> &'static UnityInterfaces {
        unsafe {
            UNITY_INTERFACES.as_ref().unwrap()
        }
    }

    pub(crate) fn set_native_unity_interfaces(
        interfaces: Option<*const unity_native_plugin_sys::IUnityInterfaces>,
    ) {
        unsafe {
            UNITY_INTERFACES = match interfaces {
                Some(interfaces) => Some(UnityInterfaces {
                    interfaces: interfaces,
                }),
                None => None,
            }
        }
    }

    pub fn get_interface<T: UnityInterface>(&self) -> Option<T> {
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

pub type RenderBuffer = unity_native_plugin_sys::UnityRenderBuffer;
pub type TextureID = unity_native_plugin_sys::UnityTextureID;
