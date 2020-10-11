use ash::vk::Handle;
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

#[allow(non_camel_case_types)]
pub type PFN_vkGetInstanceProcAddr = unity_native_plugin_sys::PFN_vkGetInstanceProcAddr;

pub struct VulkanInstance {
    native: UnityVulkanInstance,
}

impl VulkanInstance {
    pub fn pipeline_cache(&self) -> ash::vk::PipelineCache {
        ash::vk::PipelineCache::from_raw(self.native.pipelineCache as u64)
    }

    pub fn instance(&self) -> ash::vk::Instance {
        ash::vk::Instance::from_raw(self.native.instance as u64)
    }

    pub fn physical_device(&self) -> ash::vk::PhysicalDevice {
        ash::vk::PhysicalDevice::from_raw(self.native.physicalDevice as u64)
    }

    pub fn device(&self) -> ash::vk::Device {
        ash::vk::Device::from_raw(self.native.device as u64)
    }

    pub fn graphics_queue(&self) -> ash::vk::Queue {
        ash::vk::Queue::from_raw(self.native.graphicsQueue as u64)
    }

    pub fn queue_family_index(&self) -> ::std::os::raw::c_uint {
        self.native.queueFamilyIndex
    }

    pub unsafe fn get_instance_proc_addr(
        &self,
        name: *const std::os::raw::c_char,
    ) -> PFN_vkVoidFunction {
        if let Some(f) = self.native.getInstanceProcAddr {
            (f)(self.native.instance, name)
        } else {
            PFN_vkVoidFunction::None
        }
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

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum VulkanResourceAccessMode {
    ObserveOnly = UnityVulkanResourceAccessMode_kUnityVulkanResourceAccess_ObserveOnly,
    PipelineBarrier = UnityVulkanResourceAccessMode_kUnityVulkanResourceAccess_PipelineBarrier,
    Recreate = UnityVulkanResourceAccessMode_kUnityVulkanResourceAccess_Recreate,
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

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum VulkanSwapchainMode {
    Default = UnityVulkanSwapchainMode_kUnityVulkanSwapchainMode_Default,
    Offscreen = UnityVulkanSwapchainMode_kUnityVulkanSwapchainMode_Offscreen,
}

#[repr(C)]
pub struct VulkanSwapchainConfiguration {
    pub mode: VulkanSwapchainMode,
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
        unsafe {
            VulkanInstance {
                native: self.interface().Instance.expect("Instance")(),
            }
        }
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

    pub unsafe fn access_texture(
        &self,
        native_texture: *mut ::std::os::raw::c_void,
        sub_resource: Option<&ash::vk::ImageSubresource>,
        layout: ash::vk::ImageLayout,
        pipeline_stage_flags: ash::vk::PipelineStageFlags,
        access_flags: ash::vk::AccessFlags,
        access_mode: VulkanResourceAccessMode,
    ) -> Option<VulkanImage> {
        let mut ret = std::mem::zeroed::<VulkanImage>();
        if self.interface().AccessTexture.expect("AccessTexture")(
            native_texture,
            match sub_resource {
                Some(t) => std::mem::transmute(t),
                None => std::ptr::null(),
            },
            std::mem::transmute(layout),
            std::mem::transmute(pipeline_stage_flags),
            std::mem::transmute(access_flags),
            access_mode as UnityVulkanResourceAccessMode,
            std::mem::transmute(&mut ret),
        ) {
            Some(ret)
        } else {
            None
        }
    }

    pub unsafe fn access_render_buffer_texture(
        &self,
        native_render_buffer: unity_native_plugin::graphics::RenderBuffer,
        sub_resource: Option<&ash::vk::ImageSubresource>,
        layout: ash::vk::ImageLayout,
        pipeline_stage_flags: ash::vk::PipelineStageFlags,
        access_flags: ash::vk::AccessFlags,
        access_mode: VulkanResourceAccessMode,
    ) -> Option<VulkanImage> {
        let mut ret = std::mem::zeroed::<VulkanImage>();
        if self
            .interface()
            .AccessRenderBufferTexture
            .expect("AccessRenderBufferTexture")(
            native_render_buffer,
            match sub_resource {
                Some(t) => std::mem::transmute(t),
                None => std::ptr::null(),
            },
            std::mem::transmute(layout),
            std::mem::transmute(pipeline_stage_flags),
            std::mem::transmute(access_flags),
            access_mode as UnityVulkanResourceAccessMode,
            std::mem::transmute(&mut ret),
        ) {
            Some(ret)
        } else {
            None
        }
    }

    pub unsafe fn access_render_buffer_resolve_texture(
        &self,
        native_render_buffer: unity_native_plugin::graphics::RenderBuffer,
        sub_resource: Option<&ash::vk::ImageSubresource>,
        layout: ash::vk::ImageLayout,
        pipeline_stage_flags: ash::vk::PipelineStageFlags,
        access_flags: ash::vk::AccessFlags,
        access_mode: VulkanResourceAccessMode,
    ) -> Option<VulkanImage> {
        let mut ret = std::mem::zeroed::<VulkanImage>();
        if self
            .interface()
            .AccessRenderBufferResolveTexture
            .expect("AccessRenderBufferResolveTexture")(
            native_render_buffer,
            match sub_resource {
                Some(t) => std::mem::transmute(t),
                None => std::ptr::null(),
            },
            std::mem::transmute(layout),
            std::mem::transmute(pipeline_stage_flags),
            std::mem::transmute(access_flags),
            access_mode as UnityVulkanResourceAccessMode,
            std::mem::transmute(&mut ret),
        ) {
            Some(ret)
        } else {
            None
        }
    }

    pub unsafe fn access_buffer(
        &self,
        native_buffer: *mut ::std::os::raw::c_void,
        pipeline_stage_flags: ash::vk::PipelineStageFlags,
        access_flags: ash::vk::AccessFlags,
        access_mode: VulkanResourceAccessMode,
    ) -> Option<VulkanImage> {
        let mut ret = std::mem::zeroed::<VulkanImage>();
        if self.interface().AccessBuffer.expect("AccessTexture")(
            native_buffer,
            std::mem::transmute(pipeline_stage_flags),
            std::mem::transmute(access_flags),
            access_mode as UnityVulkanResourceAccessMode,
            std::mem::transmute(&mut ret),
        ) {
            Some(ret)
        } else {
            None
        }
    }

    pub fn ensure_outside_render_pass(&self) {
        unsafe {
            self.interface()
                .EnsureOutsideRenderPass
                .expect("EnsureOutsideRenderPass")()
        }
    }

    pub fn ensure_inside_render_pass(&self) {
        unsafe {
            self.interface()
                .EnsureInsideRenderPass
                .expect("EnsureInsideRenderPass")()
        }
    }

    pub unsafe fn access_queue(
        &self,
        callback: UnityRenderingEventAndData,
        event_id: ::std::os::raw::c_int,
        user_data: *mut ::std::os::raw::c_void,
        flush: bool,
    ) {
        self.interface().AccessQueue.expect("AccessQueue")(callback, event_id, user_data, flush);
    }

    pub fn configure_swapchain(&self, swapchain_config: &VulkanSwapchainConfiguration) -> bool {
        unsafe {
            self.interface()
                .ConfigureSwapchain
                .expect("ConfigureSwapchain")(std::mem::transmute(swapchain_config))
        }
    }

    pub unsafe fn access_texture_by_id(
        &self,
        texture_id: unity_native_plugin::graphics::TextureID,
        sub_resource: Option<&ash::vk::ImageSubresource>,
        layout: ash::vk::ImageLayout,
        pipeline_stage_flags: ash::vk::PipelineStageFlags,
        access_flags: ash::vk::AccessFlags,
        access_mode: VulkanResourceAccessMode,
    ) -> Option<VulkanImage> {
        let mut ret = std::mem::zeroed::<VulkanImage>();
        if self
            .interface()
            .AccessTextureByID
            .expect("AccessTextureByID")(
            texture_id,
            match sub_resource {
                Some(t) => std::mem::transmute(t),
                None => std::ptr::null(),
            },
            std::mem::transmute(layout),
            std::mem::transmute(pipeline_stage_flags),
            std::mem::transmute(access_flags),
            access_mode as UnityVulkanResourceAccessMode,
            std::mem::transmute(&mut ret),
        ) {
            Some(ret)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn size_test() {
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
        assert_eq!(
            ::std::mem::size_of::<VulkanSwapchainConfiguration>(),
            ::std::mem::size_of::<unity_native_plugin_sys::UnityVulkanSwapchainConfiguration>()
        );
    }
}
