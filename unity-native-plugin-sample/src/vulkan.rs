use std::ffi::c_void;
use unity_native_plugin::ash::vk;
use unity_native_plugin::vulkan::{
    UnityGraphicsVulkan, VulkanGraphicsQueueAccess, VulkanResourceAccessMode,
};
use unity_native_plugin_sys::PFN_vkVoidFunction;

unsafe fn unwrap_pfn(f: PFN_vkVoidFunction) -> Option<unsafe extern "system" fn()> {
    match f {
        PFN_vkVoidFunction::Some(f) => Some(f),
        PFN_vkVoidFunction::None => None,
    }
}

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

        // Unity の getInstanceProcAddr 経由で必要な Vulkan 関数を取得
        // (ash::Entry::load() で取った関数ポインタを使うと Unity の Vulkan loader フック層を
        //  バイパスしてしまい、内部状態の不整合でクラッシュするため、この経路で取得する)
        let vk_instance = intf.instance();

        let pfn = unwrap_pfn(vk_instance.get_instance_proc_addr(c"vkGetDeviceProcAddr".as_ptr()));
        let vk_get_device_proc_addr: vk::PFN_vkGetDeviceProcAddr = match pfn {
            Some(f) => std::mem::transmute(f),
            None => return,
        };

        let pfn = vk_get_device_proc_addr(vk_instance.device(), c"vkCmdClearColorImage".as_ptr());
        let vk_cmd_clear_color_image: vk::PFN_vkCmdClearColorImage = match pfn {
            Some(f) => std::mem::transmute(f),
            None => return,
        };

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
        vk_cmd_clear_color_image(
            recording.command_buffer(),
            image_info.image(),
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            &clear_value,
            1,
            &range,
        );
    }
}
