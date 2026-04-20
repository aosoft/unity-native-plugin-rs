#[cfg(windows)]
mod d3d11;
#[cfg(windows)]
mod d3d12;
mod vulkan;

use std::ffi::c_void;
use std::os::raw::c_int;

use unity_native_plugin::graphics::GfxRenderer;

unity_native_plugin::unity_native_plugin_entry_point! {
    fn unity_plugin_load(_interfaces: &unity_native_plugin::interface::UnityInterfaces) {
    }
    fn unity_plugin_unload() {
    }
}

#[repr(C)]
pub struct FillTextureParams {
    pub texture: *mut c_void,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

fn dispatch_fill_texture(texture: *mut c_void, x: f32, y: f32, z: f32, w: f32) {
    let renderer = unity_native_plugin::interface::UnityInterfaces::get()
        .interface::<unity_native_plugin::graphics::UnityGraphics>()
        .map(|g| g.renderer());

    match renderer {
        #[cfg(windows)]
        Some(GfxRenderer::D3D11) => d3d11::fill_texture(texture, x, y, z, w),
        #[cfg(windows)]
        Some(GfxRenderer::D3D12) => d3d12::fill_texture(texture, x, y, z, w),
        Some(GfxRenderer::Vulkan) => vulkan::fill_texture(texture, x, y, z, w),
        _ => {}
    }
}

extern "system" fn on_render_event_and_data(_event_id: c_int, data: *mut c_void) {
    if data.is_null() {
        return;
    }
    let p = unsafe { &*(data as *const FillTextureParams) };
    dispatch_fill_texture(p.texture, p.x, p.y, p.z, p.w);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "system" fn GetFillTextureCallback() -> extern "system" fn(c_int, *mut c_void) {
    on_render_event_and_data
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
            dispatch_fill_texture(context.back_buffer().as_raw() as _, 0.0, 0.0, n, 1.0);
            unity_native_plugin_tester::window::LoopResult::Continue
        },
        |_, _| {},
        unity_plugin_load,
        unity_plugin_unload,
    )
    .unwrap();
}
