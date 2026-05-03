use std::ffi::c_void;
use unity_native_plugin::ash::vk;
use unity_native_plugin::ash::vk::Handle;
use unity_native_plugin::vulkan::{
    UnityGraphicsVulkan, UnityGraphicsVulkanInterface, VulkanGraphicsQueueAccess,
    VulkanResourceAccessMode,
};

pub fn fill_texture(unity_texture: *mut c_void, x: f32, y: f32, z: f32, w: f32) {
    unsafe {
        if unity_texture.is_null() {
            return;
        }

        let intf = match unity_native_plugin::interface::UnityInterfaces::get()
            .interface::<UnityGraphicsVulkan>()
        {
            Some(i) => i,
            None => return,
        };

        intf.ensure_outside_render_pass();

        let image_info = match intf.access_texture(
            unity_texture,
            None,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            vk::PipelineStageFlags::TRANSFER,
            vk::AccessFlags::TRANSFER_WRITE,
            VulkanResourceAccessMode::PipelineBarrier,
        ) {
            Some(i) => i,
            None => return,
        };

        let recording = match intf.command_recording_state(VulkanGraphicsQueueAccess::DontCare) {
            Some(r) => r,
            None => return,
        };

        // Retrieve Vulkan functions via Unity's getInstanceProcAddr.
        // Using ash::Entry::load() bypasses Unity's Vulkan loader hook layer,
        // corrupting internal state and causing a crash.
        let vk_instance = intf.instance();

        let pfn = vk_instance.get_instance_proc_addr(c"vkGetDeviceProcAddr".as_ptr());
        let vk_get_device_proc_addr: vk::PFN_vkGetDeviceProcAddr = match pfn {
            Some(f) => std::mem::transmute::<
                unsafe extern "system" fn(),
                unsafe extern "system" fn(
                    vk::Device,
                    *const std::os::raw::c_char,
                ) -> Option<unsafe extern "system" fn()>,
            >(f),
            None => return,
        };

        let pfn = vk_get_device_proc_addr(vk_instance.device(), c"vkCmdClearColorImage".as_ptr());
        let vk_cmd_clear_color_image: vk::PFN_vkCmdClearColorImage = match pfn {
            Some(f) => std::mem::transmute::<
                unsafe extern "system" fn(),
                unsafe extern "system" fn(
                    vk::CommandBuffer,
                    vk::Image,
                    vk::ImageLayout,
                    *const vk::ClearColorValue,
                    u32,
                    *const vk::ImageSubresourceRange,
                ),
            >(f),
            None => return,
        };

        let cb = recording.command_buffer();
        let img = image_info.image();
        if cb.as_raw() == 0 || img.as_raw() == 0 {
            return;
        }

        let clear_value = vk::ClearColorValue {
            float32: [x, y, z, w],
        };
        let range = vk::ImageSubresourceRange {
            aspect_mask: image_info.aspect(),
            base_mip_level: 0,
            level_count: 1,
            base_array_layer: 0,
            layer_count: image_info.layers() as u32,
        };

        vk_cmd_clear_color_image(
            cb,
            img,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            &clear_value,
            1,
            &range,
        );
    }
}
