use crate::define_unity_interface;
use crate::interface;
use unity_native_plugin_sys::*;

define_unity_interface!(
    UnityGraphicsVulkan,
    unity_native_plugin_sys::IUnityGraphicsVulkan,
    0x95355348d4ef4e11_u64,
    0x9789313dfcffcc87_u64
);

pub type Handle = *mut std::ffi::c_void;
pub type VoidFunction = unity_native_plugin_sys::PFN_vkVoidFunction;

#[derive(Copy, Clone)]
pub struct VulkanInstance {
    pub pipelineCache: Handle,
    pub instance: Handle,
    pub physicalDevice: Handle,
    pub device: Handle,
    pub graphicsQueue: Handle,
    getInstanceProcAddr: PFN_vkGetInstanceProcAddr,
    pub queueFamilyIndex: ::std::os::raw::c_uint,
}

impl VulkanInstance {
    pub unsafe fn get_instance_proc_addr(&self, name: &str) -> VoidFunction {
        let name = std::ffi::CString::new(name).unwrap_or(std::ffi::CString::default());
        (self.getInstanceProcAddr.expect("getInstanceProcAddr"))(
            self.instance as VkInstance,
            name.as_ptr(),
        )
    }
}

impl UnityGraphicsVulkan {
    //pub fn instance()
}
