use crate::define_unity_interface;
use crate::interface;
use unity_native_plugin_sys::*;

define_unity_interface!(
    UnityGraphicsD3D12,
    IUnityGraphicsD3D12,
    0xEF4CEC88A45F4C4C_u64,
    0xBD295B6F2A38D9DE_u64
);

impl UnityGraphicsD3D12 {
    pub unsafe fn get_device(&self) -> *mut std::ffi::c_void {
        match (&*self.interface).GetDevice {
            Some(intf) => intf() as *mut std::ffi::c_void,
            None => std::ptr::null_mut(),
        }
    }
}

define_unity_interface!(
    UnityGraphicsD3D12v2,
    IUnityGraphicsD3D12v2,
    0xEC39D2F18446C745_u64,
    0xB1A2626641D6B11F_u64
);

define_unity_interface!(
    UnityGraphicsD3D12v3,
    IUnityGraphicsD3D12v3,
    0x57C3FAFE59E5E843_u64,
    0xBF4F5998474BB600_u64
);

define_unity_interface!(
    UnityGraphicsD3D12v4,
    IUnityGraphicsD3D12v4,
    0x498FFCC13EC94006_u64,
    0xB18F8B0FF67778C8_u64
);

define_unity_interface!(
    UnityGraphicsD3D12v5,
    IUnityGraphicsD3D12v5,
    0xF5C8D8A37D37BC42_u64,
    0xB02DFE93B5064A27_u64
);
