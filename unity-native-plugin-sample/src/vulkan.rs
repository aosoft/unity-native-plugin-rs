use ash::vk;
use std::sync::OnceLock;
use unity_native_plugin::vulkan::{
    UnityGraphicsVulkan, VulkanGraphicsQueueAccess, VulkanResourceAccessMode,
};

static VK_ENTRY: OnceLock<ash::Entry> = OnceLock::new();

fn vk_entry() -> Option<&'static ash::Entry> {
    if let Some(e) = VK_ENTRY.get() {
        return Some(e);
    }
    let loaded = unsafe { ash::Entry::load() }.ok()?;
    Some(VK_ENTRY.get_or_init(|| loaded))
}

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

        // vulkan-1.dll から本物の vkGetInstanceProcAddr をロード
        // (Unity の IUnityGraphicsVulkan::Instance().getInstanceProcAddr は
        //  "vkGetInstanceProcAddr" 名で問い合わせると NULL を返すため使えない)
        let entry = match vk_entry() {
            Some(e) => e,
            None => return,
        };

        let vk_instance = intf.instance();
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
