use ash::vk;
use unity_native_plugin::vulkan::{
    UnityGraphicsVulkan, VulkanGraphicsQueueAccess, VulkanResourceAccessMode,
};

pub fn fill_texture(unity_texture: *mut std::ffi::c_void, x: f32, y: f32, z: f32, w: f32) {
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

        // レンダーパス外であることを保証
        intf.ensure_outside_render_pass();

        // テクスチャにアクセス: レイアウトを TRANSFER_DST_OPTIMAL に遷移
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

        // コマンドバッファを取得
        let recording = match intf.command_recording_state(VulkanGraphicsQueueAccess::Allow) {
            Some(r) => r,
            None => return,
        };

        // Unity の VulkanInstance から ash::Device をロード
        let vk_instance = intf.instance();
        let get_instance_proc_addr: vk::PFN_vkGetInstanceProcAddr = std::mem::transmute(
            vk_instance.get_instance_proc_addr(c"vkGetInstanceProcAddr".as_ptr()),
        );
        let entry = ash::Entry::from_static_fn(ash::StaticFn {
            get_instance_proc_addr,
        });
        let ash_instance = ash::Instance::load(entry.static_fn(), vk_instance.instance());
        let ash_device = ash::Device::load(ash_instance.fp_v1_0(), vk_instance.device());

        // イメージをクリア
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
        ash_device.cmd_clear_color_image(
            recording.command_buffer(),
            image_info.image(),
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            &clear_value,
            &[range],
        );
    }
}
