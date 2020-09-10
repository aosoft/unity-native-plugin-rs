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

pub struct VulkanInstance {
    pub pipeline_cache: Handle,
    pub instance: Handle,
    pub physical_device: Handle,
    pub device: Handle,
    pub graphics_queue: Handle,
    get_instance_proc_addr: PFN_vkGetInstanceProcAddr,
    pub queue_family_index: ::std::os::raw::c_uint,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum VulkanEventRenderPassPreCondition {
    DontCare = UnityVulkanEventRenderPassPreCondition_kUnityVulkanRenderPass_DontCare,
    EnsureInside = UnityVulkanEventRenderPassPreCondition_kUnityVulkanRenderPass_EnsureInside,
    EnsureOutside = UnityVulkanEventRenderPassPreCondition_kUnityVulkanRenderPass_EnsureOutside,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum VulkanGraphicsQueueAccess {
    DontCare = UnityVulkanGraphicsQueueAccess_kUnityVulkanGraphicsQueueAccess_DontCare,
    Allow = UnityVulkanGraphicsQueueAccess_kUnityVulkanGraphicsQueueAccess_Allow,
}

pub struct VulkanPluginEventConfig {
    pub render_pass_precondition: VulkanEventRenderPassPreCondition,
    pub graphics_queue_access: VulkanGraphicsQueueAccess,
    pub flags: u32,
}

pub struct VulkanRecordingState {
    pub command_buffer: Handle,
    pub command_buffer_level: u32,
    pub render_pass: Handle,
    pub framebuffer: Handle,
    pub sub_pass_index: i32,
    pub current_frame_number: u64,
    pub safe_frame_number: u64,
}


impl VulkanInstance {
    pub unsafe fn get_instance_proc_addr(&self, name: &str) -> VoidFunction {
        let name = std::ffi::CString::new(name).unwrap_or(std::ffi::CString::default());
        self.get_instance_proc_addr.expect("getInstanceProcAddr")(
            self.instance as VkInstance,
            name.as_ptr(),
        )
    }
}

impl UnityGraphicsVulkan {
    //pub fn intercept_initialization()

    pub fn configure_event(&self, event_id: i32, plugin_event_config: &VulkanPluginEventConfig) {
        unsafe {
            self.interface().ConfigureEvent.expect("ConfigureEvent")(event_id, &UnityVulkanPluginEventConfig {
                renderPassPrecondition: plugin_event_config.render_pass_precondition as UnityVulkanEventRenderPassPreCondition,
                graphicsQueueAccess: plugin_event_config.graphics_queue_access as UnityVulkanGraphicsQueueAccess,
                flags: plugin_event_config.flags
            })
        }
    }

    pub fn instance(&self) -> VulkanInstance {
        unsafe {
            let instance = self.interface().Instance.expect("Instance")();
            VulkanInstance {
                pipeline_cache: instance.pipelineCache as Handle,
                instance: instance.instance as Handle,
                physical_device: instance.physicalDevice as Handle,
                device: instance.device as Handle,
                graphics_queue: instance.graphicsQueue as Handle,
                get_instance_proc_addr: instance.getInstanceProcAddr,
                queue_family_index: instance.queueFamilyIndex,
            }
        }
    }

    pub fn command_recording_state(&self, queue_access: VulkanGraphicsQueueAccess) -> Option<VulkanRecordingState> {
        let mut ret = UnityVulkanRecordingState::default();
        unsafe {
            if self.interface().CommandRecordingState.expect("CommandRecordingState")(&mut ret, queue_access as UnityVulkanGraphicsQueueAccess) {
                Some(VulkanRecordingState {
                    command_buffer: ret.commandBuffer as Handle,
                    command_buffer_level: ret.commandBufferLevel,
                    render_pass: ret.renderPass as Handle,
                    framebuffer: ret.framebuffer as Handle,
                    sub_pass_index: ret.subPassIndex,
                    current_frame_number: ret.currentFrameNumber,
                    safe_frame_number: ret.safeFrameNumber
                })
            } else {
                None
            }
        }
    }
}
