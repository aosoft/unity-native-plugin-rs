use crate::define_unity_interface;
use crate::interface;
use unity_native_plugin_sys::*;
use std::ffi::c_void;

define_unity_interface!(
    UnityGraphicsVulkan,
    IUnityGraphicsVulkan,
    0x95355348d4ef4e11_u64,
    0x9789313dfcffcc87_u64
);

impl UnityGraphicsVulkan {

}