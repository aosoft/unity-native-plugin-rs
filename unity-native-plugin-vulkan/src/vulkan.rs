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

pub type VulkanInitCallback = Option<
    unsafe extern "system" fn(
        get_instance_proc_addr: ash::vk::PFN_vkGetInstanceProcAddr,
        userdata: *mut ::std::os::raw::c_void,
    ) -> Option<ash::vk::PFN_vkGetInstanceProcAddr>,
>;

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
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum VulkanEventRenderPassPreCondition {
    DontCare = UnityVulkanEventRenderPassPreCondition_kUnityVulkanRenderPass_DontCare,
    EnsureInside = UnityVulkanEventRenderPassPreCondition_kUnityVulkanRenderPass_EnsureInside,
    EnsureOutside = UnityVulkanEventRenderPassPreCondition_kUnityVulkanRenderPass_EnsureOutside,
}

#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum VulkanGraphicsQueueAccess {
    DontCare = UnityVulkanGraphicsQueueAccess_kUnityVulkanGraphicsQueueAccess_DontCare,
    Allow = UnityVulkanGraphicsQueueAccess_kUnityVulkanGraphicsQueueAccess_Allow,
}

pub struct VulkanPluginEventConfig {
    native: UnityVulkanPluginEventConfig,
}

impl VulkanPluginEventConfig {
    pub fn render_pass_precondition(&self) -> VulkanEventRenderPassPreCondition {
        unsafe { std::mem::transmute(self.native.renderPassPrecondition) }
    }

    pub fn graphics_queue_access(&self) -> VulkanGraphicsQueueAccess {
        unsafe { std::mem::transmute(self.native.graphicsQueueAccess) }
    }

    pub fn flags(&self) -> u32 {
        unsafe { std::mem::transmute(self.native.flags) }
    }
}

pub struct VulkanRecordingState {
    native: UnityVulkanRecordingState,
}

impl VulkanRecordingState {
    pub fn command_buffer(&self) -> ash::vk::CommandBuffer {
        ash::vk::CommandBuffer::from_raw(self.native.commandBuffer as u64)
    }

    pub fn command_buffer_level(&self) -> ash::vk::CommandBufferLevel {
        unsafe { std::mem::transmute(self.native.commandBufferLevel) }
    }

    pub fn render_pass(&self) -> ash::vk::RenderPass {
        ash::vk::RenderPass::from_raw(self.native.renderPass as u64)
    }

    pub fn framebuffer(&self) -> ash::vk::Framebuffer {
        ash::vk::Framebuffer::from_raw(self.native.framebuffer as u64)
    }

    pub fn sub_pass_index(&self) -> ::std::os::raw::c_int {
        self.native.subPassIndex
    }

    pub fn current_frame_number(&self) -> ::std::os::raw::c_ulonglong {
        self.native.currentFrameNumber
    }

    pub fn safe_frame_number(&self) -> ::std::os::raw::c_ulonglong {
        self.native.safeFrameNumber
    }
}

pub struct VulkanMemory<'a> {
    native: &'a UnityVulkanMemory,
}

impl VulkanMemory<'_> {
    pub fn memory(&self) -> ash::vk::DeviceMemory {
        ash::vk::DeviceMemory::from_raw(self.native.memory as u64)
    }

    pub fn offset(&self) -> ash::vk::DeviceSize {
        self.native.offset
    }

    pub fn size(&self) -> ash::vk::DeviceSize {
        self.native.size
    }

    pub fn mapped(&self) -> *mut ::std::os::raw::c_void {
        self.native.mapped
    }

    pub fn flags(&self) -> ash::vk::MemoryPropertyFlags {
        unsafe { std::mem::transmute(self.native.flags) }
    }

    pub fn memory_type_index(&self) -> ::std::os::raw::c_uint {
        self.native.memoryTypeIndex
    }
}

#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum VulkanResourceAccessMode {
    ObserveOnly = UnityVulkanResourceAccessMode_kUnityVulkanResourceAccess_ObserveOnly,
    PipelineBarrier = UnityVulkanResourceAccessMode_kUnityVulkanResourceAccess_PipelineBarrier,
    Recreate = UnityVulkanResourceAccessMode_kUnityVulkanResourceAccess_Recreate,
}

pub struct VulkanImage {
    native: UnityVulkanImage,
}

impl VulkanImage {
    pub fn memory(&self) -> VulkanMemory {
        VulkanMemory {
            native: &self.native.memory,
        }
    }

    pub fn image(&self) -> ash::vk::Image {
        ash::vk::Image::from_raw(self.native.image as u64)
    }

    pub fn layout(&self) -> ash::vk::ImageLayout {
        unsafe { ash::vk::ImageLayout::from_raw(std::mem::transmute(self.native.layout)) }
    }

    pub fn aspect(&self) -> ash::vk::ImageAspectFlags {
        unsafe { std::mem::transmute(self.native.aspect) }
    }

    pub fn usage(&self) -> ash::vk::ImageUsageFlags {
        unsafe { std::mem::transmute(self.native.usage) }
    }

    pub fn format(&self) -> ash::vk::Format {
        unsafe { std::mem::transmute(self.native.format) }
    }

    pub fn extent(&self) -> ash::vk::Extent3D {
        unsafe { std::mem::transmute(self.native.extent) }
    }

    pub fn tiling(&self) -> ash::vk::ImageTiling {
        unsafe { std::mem::transmute(self.native.tiling) }
    }

    pub fn image_type(&self) -> ash::vk::ImageType {
        unsafe { std::mem::transmute(self.native.type_) }
    }

    pub fn samples(&self) -> ash::vk::SampleCountFlags {
        unsafe { std::mem::transmute(self.native.samples) }
    }

    pub fn layers(&self) -> ::std::os::raw::c_int {
        self.native.layers
    }

    pub fn mip_count(&self) -> ::std::os::raw::c_int {
        self.native.mipCount
    }
}

#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq)]
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
            .expect("InterceptInitialization")(std::mem::transmute(func), userdata);
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
                &plugin_event_config.native,
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
            let mut ret = std::mem::zeroed::<UnityVulkanRecordingState>();
            if self
                .interface()
                .CommandRecordingState
                .expect("CommandRecordingState")(
                std::mem::transmute(&mut ret),
                queue_access as UnityVulkanGraphicsQueueAccess,
            ) {
                Some(VulkanRecordingState { native: ret })
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
        let mut ret = std::mem::zeroed::<UnityVulkanImage>();
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
            Some(VulkanImage { native: ret })
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
        let mut ret = std::mem::zeroed::<UnityVulkanImage>();
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
            Some(VulkanImage { native: ret })
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
        let mut ret = std::mem::zeroed::<UnityVulkanImage>();
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
            Some(VulkanImage { native: ret })
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
        let mut ret = std::mem::zeroed::<UnityVulkanImage>();
        if self.interface().AccessBuffer.expect("AccessTexture")(
            native_buffer,
            std::mem::transmute(pipeline_stage_flags),
            std::mem::transmute(access_flags),
            access_mode as UnityVulkanResourceAccessMode,
            std::mem::transmute(&mut ret),
        ) {
            Some(VulkanImage { native: ret })
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
        let mut ret = std::mem::zeroed::<UnityVulkanImage>();
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
            Some(VulkanImage { native: ret })
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
            ::std::mem::size_of::<VulkanSwapchainConfiguration>(),
            ::std::mem::size_of::<unity_native_plugin_sys::UnityVulkanSwapchainConfiguration>()
        );
    }
}
