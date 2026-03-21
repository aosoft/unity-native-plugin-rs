#[cfg(windows)]
mod d3d11;
#[cfg(windows)]
mod d3d12;
mod vulkan;

use unity_native_plugin::graphics::GfxRenderer;

unity_native_plugin::unity_native_plugin_entry_point! {
    fn unity_plugin_load(_interfaces: &unity_native_plugin::interface::UnityInterfaces) {
    }
    fn unity_plugin_unload() {
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
extern "system" fn FillTexture(
    unity_texture: *mut std::ffi::c_void,
    x: f32,
    y: f32,
    z: f32,
    w: f32,
) {
    let renderer = unity_native_plugin::interface::UnityInterfaces::get()
        .interface::<unity_native_plugin::graphics::UnityGraphics>()
        .map(|g| g.renderer());

    match renderer {
        #[cfg(windows)]
        Some(GfxRenderer::D3D11) => d3d11::fill_texture(unity_texture, x, y, z, w),
        #[cfg(windows)]
        Some(GfxRenderer::D3D12) => d3d12::fill_texture(unity_texture, x, y, z, w),
        Some(GfxRenderer::Vulkan) => vulkan::fill_texture(unity_texture, x, y, z, w),
        _ => {}
    }
}

#[cfg(windows)]
#[test]
fn test() {
    let instant = std::time::Instant::now();
    unity_native_plugin_tester::d3d11::test_plugin_d3d11(
        (256, 256),
        |_window, _context| {},
        |_window, context| {
            let n = (instant.elapsed().as_millis() % 1000) as f32 / 1000.0;
            FillTexture(context.back_buffer().as_raw() as _, 0.0, 0.0, n, 1.0);
            unity_native_plugin_tester::window::LoopResult::Continue
        },
        |_, _| {},
        unity_plugin_load,
        unity_plugin_unload,
    )
    .unwrap();
}
