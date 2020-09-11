use unity_native_plugin::define_unity_interface;
use unity_native_plugin::interface::UnityInterface;
use unity_native_plugin_sys::*;

define_unity_interface!(
    UnityGraphicsVulkan,
    unity_native_plugin_sys::IUnityGraphicsVulkan,
    0x95355348d4ef4e11_u64,
    0x9789313dfcffcc87_u64
);

pub type VulkanInitCallback = UnityVulkanInitCallback;

#[repr(C)]
pub struct VulkanInstance {
    pub pipeline_cache: ash::vk::PipelineCache,
    pub instance: ash::vk::Instance,
    pub physical_device: ash::vk::PhysicalDevice,
    pub device: ash::vk::Device,
    pub graphics_queue: ash::vk::Queue,
    get_instance_proc_addr: ash::vk::PFN_vkGetInstanceProcAddr,
    pub queue_family_index: ::std::os::raw::c_uint,
    reserved: [*mut ::std::os::raw::c_void; 8usize],
}

impl VulkanInstance {
    pub unsafe fn get_instance_proc_addr(
        &self,
        name: *const std::os::raw::c_char,
    ) -> ash::vk::PFN_vkVoidFunction {
        std::mem::transmute((self.get_instance_proc_addr)(self.instance, name))
    }
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

#[repr(C)]
pub struct VulkanPluginEventConfig {
    pub render_pass_precondition: VulkanEventRenderPassPreCondition,
    pub graphics_queue_access: VulkanGraphicsQueueAccess,
    pub flags: u32,
}

#[repr(C)]
pub struct VulkanRecordingState {
    pub command_buffer: ash::vk::CommandBuffer,
    pub command_buffer_level: ash::vk::CommandBufferLevel,
    pub render_pass: ash::vk::RenderPass,
    pub framebuffer: ash::vk::Framebuffer,
    pub sub_pass_index: ::std::os::raw::c_int,
    pub current_frame_number: ::std::os::raw::c_ulonglong,
    pub safe_frame_number: ::std::os::raw::c_ulonglong,
    reserved: [*mut ::std::os::raw::c_void; 4usize],
}

#[repr(C)]
pub struct VulkanMemory {
    pub memory: ash::vk::DeviceMemory,
    pub offset: ash::vk::DeviceSize,
    pub size: ash::vk::DeviceSize,
    pub mapped: *mut ::std::os::raw::c_void,
    pub flags: ash::vk::MemoryPropertyFlags,
    pub memory_type_index: ::std::os::raw::c_uint,
    reserved: [*mut ::std::os::raw::c_void; 4usize],
}

#[repr(C)]
pub struct VulkanImage {
    pub memory: VulkanMemory,
    pub image: ash::vk::Image,
    pub layout: ash::vk::ImageLayout,
    pub aspect: ash::vk::ImageAspectFlags,
    pub usage: ash::vk::ImageUsageFlags,
    pub format: ash::vk::Format,
    pub extent: ash::vk::Extent3D,
    pub tiling: ash::vk::ImageTiling,
    pub type_: ash::vk::ImageType,
    pub samples: ash::vk::SampleCountFlags,
    pub layers: ::std::os::raw::c_int,
    pub mip_count: ::std::os::raw::c_int,
    reserved: [*mut ::std::os::raw::c_void; 4usize],
}

impl UnityGraphicsVulkan {
    pub unsafe fn intercept_initialization(
        &self,
        func: VulkanInitCallback,
        userdata: *mut ::std::os::raw::c_void,
    ) {
        self.interface()
            .InterceptInitialization
            .expect("InterceptInitialization")(func, userdata);
    }

    pub unsafe fn intercept_vulkan_api(
        &self,
        name: *const ::std::os::raw::c_char,
        func: ash::vk::PFN_vkVoidFunction,
    ) -> ash::vk::PFN_vkVoidFunction {
        std::mem::transmute(self
            .interface()
            .InterceptVulkanAPI
            .expect("InterceptVulkanAPI")(
            name, std::mem::transmute(func)
        ))
    }

    pub fn configure_event(&self, event_id: i32, plugin_event_config: &VulkanPluginEventConfig) {
        unsafe {
            self.interface().ConfigureEvent.expect("ConfigureEvent")(
                event_id,
                std::mem::transmute(plugin_event_config),
            )
        }
    }

    pub fn instance(&self) -> VulkanInstance {
        unsafe { std::mem::transmute(self.interface().Instance.expect("Instance")()) }
    }

    pub fn command_recording_state(
        &self,
        queue_access: VulkanGraphicsQueueAccess,
    ) -> Option<VulkanRecordingState> {
        unsafe {
            let mut ret = std::mem::zeroed::<VulkanRecordingState>();
            if self
                .interface()
                .CommandRecordingState
                .expect("CommandRecordingState")(
                std::mem::transmute(&mut ret),
                queue_access as UnityVulkanGraphicsQueueAccess,
            ) {
                Some(ret)
            } else {
                None
            }
        }
    }

    /*pub fn access_texture(
        nativeTexture: *mut ::std::os::raw::c_void,
        subResource: *const VkImageSubresource,
        layout: VkImageLayout,
        pipelineStageFlags: VkPipelineStageFlags,
        accessFlags: VkAccessFlags,
        accessMode: UnityVulkanResourceAccessMode,
        outImage: *mut UnityVulkanImage)*/
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn size_test() {
        assert_eq!(
            ::std::mem::size_of::<VulkanInstance>(),
            ::std::mem::size_of::<unity_native_plugin_sys::UnityVulkanInstance>()
        );
        assert_eq!(
            ::std::mem::size_of::<VulkanPluginEventConfig>(),
            ::std::mem::size_of::<unity_native_plugin_sys::UnityVulkanPluginEventConfig>()
        );
        assert_eq!(
            ::std::mem::size_of::<VulkanRecordingState>(),
            ::std::mem::size_of::<unity_native_plugin_sys::UnityVulkanRecordingState>()
        );
        assert_eq!(
            ::std::mem::size_of::<VulkanMemory>(),
            ::std::mem::size_of::<unity_native_plugin_sys::UnityVulkanMemory>()
        );
        assert_eq!(
            ::std::mem::size_of::<VulkanImage>(),
            ::std::mem::size_of::<unity_native_plugin_sys::UnityVulkanImage>()
        );
    }
}
