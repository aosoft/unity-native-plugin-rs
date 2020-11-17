use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

#[cfg(target_os = "windows")]
use winit::platform::windows::EventLoopExtWindows;
use winit::platform::desktop::EventLoopExtDesktop;

pub enum LoopResult {
    Continue,
    Exit,
}

pub fn run_window_app<
    Context: 'static + crate::interface::UnityInterfaceBase + crate::interface::UnityInterfaceID,
    FnInit: FnOnce(&Window) -> Context,
    FnMain: FnMut(&Window) -> LoopResult,
    FnFinalize: FnOnce(&Window),
>(
    fn_initialize: FnInit,
    mut fn_main: FnMain,
    fn_finalize: FnFinalize,
    fn_unity_plugin_load: fn(interfaces: &unity_native_plugin::interface::UnityInterfaces),
    fn_unity_plugin_unload: fn(),
) {
    let mut event_loop = EventLoop::<u32>::new_any_thread();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    unsafe {
        crate::interface::get_unity_interfaces().register_interface::<Context>(Some(Box::new(fn_initialize(&window))));
    }

    fn_unity_plugin_load(unity_native_plugin::interface::UnityInterfaces::get());

    event_loop.run_return(|event, _, control_flow| {
        *control_flow = match fn_main(&window) {
            LoopResult::Continue => ControlFlow::WaitUntil(
                std::time::Instant::now() + std::time::Duration::from_millis(50)),
            _ => ControlFlow::Exit
        };

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });

    fn_unity_plugin_unload();
    fn_finalize(&window);
}

