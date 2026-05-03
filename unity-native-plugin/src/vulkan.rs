use crate::interface::UnityInterface;
use ash::vk::Handle;
use unity_native_plugin_sys::*;

define_unity_interface!(
    UnityGraphicsVulkan,
    IUnityGraphicsVulkan,
    0x95355348d4ef4e11_u64,
    0x9789313dfcffcc87_u64
);

pub type VulkanInitCallback = Option<
    unsafe extern "system" fn(
        get_instance_proc_addr: ash::vk::PFN_vkGetInstanceProcAddr,
        user_data: *mut ::std::os::raw::c_void,
    ) -> ash::vk::PFN_vkGetInstanceProcAddr,
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
    ) -> ash::vk::PFN_vkVoidFunction {
        if let Some(f) = self.native.getInstanceProcAddr {
            unsafe { (f)(self.native.instance, name) }
        } else {
            None
        }
    }
}

#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum VulkanEventRenderPassPreCondition {
    DontCare = UnityVulkanEventRenderPassPreCondition_kUnityVulkanRenderPass_DontCare,
    EnsureInside = UnityVulkanEventRenderPassPreCondition_kUnityVulkanRenderPass_EnsureInside,
    EnsureOutside = UnityVulkanEventRenderPassPreCondition_kUnityVulkanRenderPass_EnsureOutside,
}

#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum VulkanGraphicsQueueAccess {
    DontCare = UnityVulkanGraphicsQueueAccess_kUnityVulkanGraphicsQueueAccess_DontCare,
    Allow = UnityVulkanGraphicsQueueAccess_kUnityVulkanGraphicsQueueAccess_Allow,
}

pub struct VulkanPluginEventConfig {
    native: UnityVulkanPluginEventConfig,
}

impl VulkanPluginEventConfig {
    pub fn new(
        render_pass_precondition: VulkanEventRenderPassPreCondition,
        graphics_queue_access: VulkanGraphicsQueueAccess,
        flags: u32,
    ) -> Self {
        Self {
            native: UnityVulkanPluginEventConfig {
                renderPassPrecondition: render_pass_precondition
                    as UnityVulkanEventRenderPassPreCondition,
                graphicsQueueAccess: graphics_queue_access as UnityVulkanGraphicsQueueAccess,
                flags,
            },
        }
    }

    pub fn render_pass_precondition(&self) -> VulkanEventRenderPassPreCondition {
        unsafe {
            std::mem::transmute::<
                UnityVulkanEventRenderPassPreCondition,
                VulkanEventRenderPassPreCondition,
            >(self.native.renderPassPrecondition)
        }
    }

    pub fn graphics_queue_access(&self) -> VulkanGraphicsQueueAccess {
        unsafe {
            std::mem::transmute::<UnityVulkanGraphicsQueueAccess, VulkanGraphicsQueueAccess>(
                self.native.graphicsQueueAccess,
            )
        }
    }

    pub fn flags(&self) -> u32 {
        self.native.flags
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
        ash::vk::CommandBufferLevel::from_raw(self.native.commandBufferLevel as i32)
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
        ash::vk::MemoryPropertyFlags::from_raw(self.native.flags)
    }

    pub fn memory_type_index(&self) -> ::std::os::raw::c_uint {
        self.native.memoryTypeIndex
    }
}

#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum VulkanResourceAccessMode {
    ObserveOnly = UnityVulkanResourceAccessMode_kUnityVulkanResourceAccess_ObserveOnly,
    PipelineBarrier = UnityVulkanResourceAccessMode_kUnityVulkanResourceAccess_PipelineBarrier,
    Recreate = UnityVulkanResourceAccessMode_kUnityVulkanResourceAccess_Recreate,
}

pub struct VulkanImage {
    native: UnityVulkanImage,
}

impl VulkanImage {
    pub fn memory(&self) -> VulkanMemory<'_> {
        VulkanMemory {
            native: &self.native.memory,
        }
    }

    pub fn image(&self) -> ash::vk::Image {
        ash::vk::Image::from_raw(self.native.image as u64)
    }

    pub fn layout(&self) -> ash::vk::ImageLayout {
        ash::vk::ImageLayout::from_raw(self.native.layout as i32)
    }

    pub fn aspect(&self) -> ash::vk::ImageAspectFlags {
        ash::vk::ImageAspectFlags::from_raw(self.native.aspect)
    }

    pub fn usage(&self) -> ash::vk::ImageUsageFlags {
        ash::vk::ImageUsageFlags::from_raw(self.native.usage)
    }

    pub fn format(&self) -> ash::vk::Format {
        ash::vk::Format::from_raw(self.native.format as i32)
    }

    pub fn extent(&self) -> ash::vk::Extent3D {
        ash::vk::Extent3D {
            width: self.native.extent.width,
            height: self.native.extent.height,
            depth: self.native.extent.depth,
        }
    }

    pub fn tiling(&self) -> ash::vk::ImageTiling {
        ash::vk::ImageTiling::from_raw(self.native.tiling as i32)
    }

    pub fn image_type(&self) -> ash::vk::ImageType {
        ash::vk::ImageType::from_raw(self.native.type_ as i32)
    }

    pub fn samples(&self) -> ash::vk::SampleCountFlags {
        ash::vk::SampleCountFlags::from_raw(self.native.samples)
    }

    pub fn layers(&self) -> ::std::os::raw::c_int {
        self.native.layers
    }

    pub fn mip_count(&self) -> ::std::os::raw::c_int {
        self.native.mipCount
    }
}

#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum VulkanSwapchainMode {
    Default = UnityVulkanSwapchainMode_kUnityVulkanSwapchainMode_Default,
    Offscreen = UnityVulkanSwapchainMode_kUnityVulkanSwapchainMode_Offscreen,
}

#[repr(C)]
pub struct VulkanSwapchainConfiguration {
    pub mode: VulkanSwapchainMode,
}

pub trait UnityGraphicsVulkanExt {
    unsafe fn intercept_initialization(
        &self,
        func: VulkanInitCallback,
        user_data: *mut ::std::os::raw::c_void,
    );
    unsafe fn intercept_vulkan_api(
        &self,
        name: *const ::std::os::raw::c_char,
        func: ash::vk::PFN_vkVoidFunction,
    ) -> ash::vk::PFN_vkVoidFunction;
    fn configure_event(&self, event_id: i32, plugin_event_config: &VulkanPluginEventConfig);
    fn instance(&self) -> VulkanInstance;
    fn command_recording_state(
        &self,
        queue_access: VulkanGraphicsQueueAccess,
    ) -> Option<VulkanRecordingState>;
    unsafe fn access_texture(
        &self,
        native_texture: *mut ::std::os::raw::c_void,
        sub_resource: Option<&ash::vk::ImageSubresource>,
        layout: ash::vk::ImageLayout,
        pipeline_stage_flags: ash::vk::PipelineStageFlags,
        access_flags: ash::vk::AccessFlags,
        access_mode: VulkanResourceAccessMode,
    ) -> Option<VulkanImage>;
    unsafe fn access_render_buffer_texture(
        &self,
        native_render_buffer: crate::graphics::RenderBuffer,
        sub_resource: Option<&ash::vk::ImageSubresource>,
        layout: ash::vk::ImageLayout,
        pipeline_stage_flags: ash::vk::PipelineStageFlags,
        access_flags: ash::vk::AccessFlags,
        access_mode: VulkanResourceAccessMode,
    ) -> Option<VulkanImage>;
    unsafe fn access_render_buffer_resolve_texture(
        &self,
        native_render_buffer: crate::graphics::RenderBuffer,
        sub_resource: Option<&ash::vk::ImageSubresource>,
        layout: ash::vk::ImageLayout,
        pipeline_stage_flags: ash::vk::PipelineStageFlags,
        access_flags: ash::vk::AccessFlags,
        access_mode: VulkanResourceAccessMode,
    ) -> Option<VulkanImage>;
    unsafe fn access_buffer(
        &self,
        native_buffer: *mut ::std::os::raw::c_void,
        pipeline_stage_flags: ash::vk::PipelineStageFlags,
        access_flags: ash::vk::AccessFlags,
        access_mode: VulkanResourceAccessMode,
    ) -> Option<VulkanImage>;
    fn ensure_outside_render_pass(&self);
    fn ensure_inside_render_pass(&self);
    unsafe fn access_queue(
        &self,
        callback: UnityRenderingEventAndData,
        event_id: ::std::os::raw::c_int,
        user_data: *mut ::std::os::raw::c_void,
        flush: bool,
    );
    fn configure_swapchain(&self, swapchain_config: &VulkanSwapchainConfiguration) -> bool;
    unsafe fn access_texture_by_id(
        &self,
        texture_id: crate::graphics::TextureID,
        sub_resource: Option<&ash::vk::ImageSubresource>,
        layout: ash::vk::ImageLayout,
        pipeline_stage_flags: ash::vk::PipelineStageFlags,
        access_flags: ash::vk::AccessFlags,
        access_mode: VulkanResourceAccessMode,
    ) -> Option<VulkanImage>;
}

macro_rules! impl_vulkan {
    ($intf:ty) => {
        impl UnityGraphicsVulkanExt for $intf {
            unsafe fn intercept_initialization(
                &self,
                func: VulkanInitCallback,
                user_data: *mut ::std::os::raw::c_void,
            ) {
                unsafe {
                    self.interface()
                        .InterceptInitialization
                        .expect("InterceptInitialization")(
                        std::mem::transmute::<VulkanInitCallback, UnityVulkanInitCallback>(func),
                        user_data,
                    );
                }
            }

            unsafe fn intercept_vulkan_api(
                &self,
                name: *const ::std::os::raw::c_char,
                func: ash::vk::PFN_vkVoidFunction,
            ) -> ash::vk::PFN_vkVoidFunction {
                unsafe {
                    self.interface()
                        .InterceptVulkanAPI
                        .expect("InterceptVulkanAPI")(name, func)
                }
            }

            fn configure_event(
                &self,
                event_id: i32,
                plugin_event_config: &VulkanPluginEventConfig,
            ) {
                unsafe {
                    self.interface().ConfigureEvent.expect("ConfigureEvent")(
                        event_id,
                        &plugin_event_config.native,
                    )
                }
            }

            fn instance(&self) -> VulkanInstance {
                unsafe {
                    VulkanInstance {
                        native: self.interface().Instance.expect("Instance")(),
                    }
                }
            }

            fn command_recording_state(
                &self,
                queue_access: VulkanGraphicsQueueAccess,
            ) -> Option<VulkanRecordingState> {
                unsafe {
                    let mut ret = std::mem::zeroed::<UnityVulkanRecordingState>();
                    if self
                        .interface()
                        .CommandRecordingState
                        .expect("CommandRecordingState")(
                        &mut ret,
                        queue_access as UnityVulkanGraphicsQueueAccess,
                    ) {
                        Some(VulkanRecordingState { native: ret })
                    } else {
                        None
                    }
                }
            }

            unsafe fn access_texture(
                &self,
                native_texture: *mut ::std::os::raw::c_void,
                sub_resource: Option<&ash::vk::ImageSubresource>,
                layout: ash::vk::ImageLayout,
                pipeline_stage_flags: ash::vk::PipelineStageFlags,
                access_flags: ash::vk::AccessFlags,
                access_mode: VulkanResourceAccessMode,
            ) -> Option<VulkanImage> {
                unsafe {
                    let mut ret = std::mem::zeroed::<UnityVulkanImage>();
                    if self.interface().AccessTexture.expect("AccessTexture")(
                        native_texture,
                        match sub_resource {
                            Some(t) => {
                                t as *const ash::vk::ImageSubresource as *const VkImageSubresource
                            }
                            None => std::ptr::null(),
                        },
                        layout.as_raw() as VkImageLayout,
                        pipeline_stage_flags.as_raw(),
                        access_flags.as_raw(),
                        access_mode as UnityVulkanResourceAccessMode,
                        &mut ret,
                    ) {
                        Some(VulkanImage { native: ret })
                    } else {
                        None
                    }
                }
            }

            unsafe fn access_render_buffer_texture(
                &self,
                native_render_buffer: crate::graphics::RenderBuffer,
                sub_resource: Option<&ash::vk::ImageSubresource>,
                layout: ash::vk::ImageLayout,
                pipeline_stage_flags: ash::vk::PipelineStageFlags,
                access_flags: ash::vk::AccessFlags,
                access_mode: VulkanResourceAccessMode,
            ) -> Option<VulkanImage> {
                unsafe {
                    let mut ret = std::mem::zeroed::<UnityVulkanImage>();
                    if self
                        .interface()
                        .AccessRenderBufferTexture
                        .expect("AccessRenderBufferTexture")(
                        native_render_buffer,
                        match sub_resource {
                            Some(t) => {
                                t as *const ash::vk::ImageSubresource as *const VkImageSubresource
                            }
                            None => std::ptr::null(),
                        },
                        layout.as_raw() as VkImageLayout,
                        pipeline_stage_flags.as_raw(),
                        access_flags.as_raw(),
                        access_mode as UnityVulkanResourceAccessMode,
                        &mut ret,
                    ) {
                        Some(VulkanImage { native: ret })
                    } else {
                        None
                    }
                }
            }

            unsafe fn access_render_buffer_resolve_texture(
                &self,
                native_render_buffer: crate::graphics::RenderBuffer,
                sub_resource: Option<&ash::vk::ImageSubresource>,
                layout: ash::vk::ImageLayout,
                pipeline_stage_flags: ash::vk::PipelineStageFlags,
                access_flags: ash::vk::AccessFlags,
                access_mode: VulkanResourceAccessMode,
            ) -> Option<VulkanImage> {
                unsafe {
                    let mut ret = std::mem::zeroed::<UnityVulkanImage>();
                    if self
                        .interface()
                        .AccessRenderBufferResolveTexture
                        .expect("AccessRenderBufferResolveTexture")(
                        native_render_buffer,
                        match sub_resource {
                            Some(t) => {
                                t as *const ash::vk::ImageSubresource as *const VkImageSubresource
                            }
                            None => std::ptr::null(),
                        },
                        layout.as_raw() as VkImageLayout,
                        pipeline_stage_flags.as_raw(),
                        access_flags.as_raw(),
                        access_mode as UnityVulkanResourceAccessMode,
                        &mut ret,
                    ) {
                        Some(VulkanImage { native: ret })
                    } else {
                        None
                    }
                }
            }

            unsafe fn access_buffer(
                &self,
                native_buffer: *mut ::std::os::raw::c_void,
                pipeline_stage_flags: ash::vk::PipelineStageFlags,
                access_flags: ash::vk::AccessFlags,
                access_mode: VulkanResourceAccessMode,
            ) -> Option<VulkanImage> {
                unsafe {
                    let mut ret = std::mem::zeroed::<UnityVulkanImage>();
                    if self.interface().AccessBuffer.expect("AccessBuffer")(
                        native_buffer,
                        pipeline_stage_flags.as_raw(),
                        access_flags.as_raw(),
                        access_mode as UnityVulkanResourceAccessMode,
                        &mut ret as *mut UnityVulkanImage as *mut UnityVulkanBuffer,
                    ) {
                        Some(VulkanImage { native: ret })
                    } else {
                        None
                    }
                }
            }

            fn ensure_outside_render_pass(&self) {
                unsafe {
                    self.interface()
                        .EnsureOutsideRenderPass
                        .expect("EnsureOutsideRenderPass")()
                }
            }

            fn ensure_inside_render_pass(&self) {
                unsafe {
                    self.interface()
                        .EnsureInsideRenderPass
                        .expect("EnsureInsideRenderPass")()
                }
            }

            unsafe fn access_queue(
                &self,
                callback: UnityRenderingEventAndData,
                event_id: ::std::os::raw::c_int,
                user_data: *mut ::std::os::raw::c_void,
                flush: bool,
            ) {
                unsafe {
                    self.interface().AccessQueue.expect("AccessQueue")(
                        callback, event_id, user_data, flush,
                    );
                }
            }

            fn configure_swapchain(&self, swapchain_config: &VulkanSwapchainConfiguration) -> bool {
                unsafe {
                    self.interface()
                        .ConfigureSwapchain
                        .expect("ConfigureSwapchain")(
                        swapchain_config as *const VulkanSwapchainConfiguration
                            as *const UnityVulkanSwapchainConfiguration,
                    )
                }
            }

            unsafe fn access_texture_by_id(
                &self,
                texture_id: crate::graphics::TextureID,
                sub_resource: Option<&ash::vk::ImageSubresource>,
                layout: ash::vk::ImageLayout,
                pipeline_stage_flags: ash::vk::PipelineStageFlags,
                access_flags: ash::vk::AccessFlags,
                access_mode: VulkanResourceAccessMode,
            ) -> Option<VulkanImage> {
                unsafe {
                    let mut ret = std::mem::zeroed::<UnityVulkanImage>();
                    if self
                        .interface()
                        .AccessTextureByID
                        .expect("AccessTextureByID")(
                        texture_id,
                        match sub_resource {
                            Some(t) => {
                                t as *const ash::vk::ImageSubresource as *const VkImageSubresource
                            }
                            None => std::ptr::null(),
                        },
                        layout.as_raw() as VkImageLayout,
                        pipeline_stage_flags.as_raw(),
                        access_flags.as_raw(),
                        access_mode as UnityVulkanResourceAccessMode,
                        &mut ret,
                    ) {
                        Some(VulkanImage { native: ret })
                    } else {
                        None
                    }
                }
            }
        }
    };
}

impl_vulkan!(UnityGraphicsVulkan);

define_unity_interface!(
    UnityGraphicsVulkanV2,
    IUnityGraphicsVulkanV2,
    0x329334c09dca4787_u64,
    0xb347dd92a0097ffc_u64
);

pub trait UnityGraphicsVulkanV2Ext: UnityGraphicsVulkanExt {
    unsafe fn add_intercept_initialization(
        &self,
        func: VulkanInitCallback,
        user_data: *mut ::std::os::raw::c_void,
        priority: i32,
    ) -> bool;
    unsafe fn remove_intercept_initialization(&self, func: VulkanInitCallback) -> bool;
}

macro_rules! impl_vulkan_v2 {
    ($intf:ty) => {
        impl_vulkan!($intf);

        impl UnityGraphicsVulkanV2Ext for $intf {
            unsafe fn add_intercept_initialization(
                &self,
                func: VulkanInitCallback,
                user_data: *mut ::std::os::raw::c_void,
                priority: i32,
            ) -> bool {
                unsafe {
                    self.interface()
                        .AddInterceptInitialization
                        .expect("AddInterceptInitialization")(
                        std::mem::transmute::<VulkanInitCallback, UnityVulkanInitCallback>(func),
                        user_data,
                        priority,
                    )
                }
            }

            unsafe fn remove_intercept_initialization(&self, func: VulkanInitCallback) -> bool {
                unsafe {
                    self.interface()
                        .RemoveInterceptInitialization
                        .expect("RemoveInterceptInitialization")(std::mem::transmute::<
                        VulkanInitCallback,
                        UnityVulkanInitCallback,
                    >(func))
                }
            }
        }
    };
}

impl_vulkan_v2!(UnityGraphicsVulkanV2);

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
