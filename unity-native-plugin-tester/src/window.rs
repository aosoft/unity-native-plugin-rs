use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

use std::ops::Deref;
use winit::platform::desktop::EventLoopExtDesktop;
#[cfg(target_os = "windows")]
use winit::platform::windows::EventLoopExtWindows;

pub enum LoopResult {
    Continue,
    Exit,
}

pub fn run_window_app<
    Context: 'static + crate::interface::UnityInterfaceBase + crate::interface::UnityInterfaceID,
    FnInit: FnOnce(&Window) -> Context,
    FnMain: FnMut(&Window, &Context) -> LoopResult,
    FnFinalize: FnOnce(&Window, &Context),
>(
    fn_initialize: FnInit,
    mut fn_main: FnMain,
    fn_finalize: FnFinalize,
    fn_unity_plugin_load: fn(interfaces: &unity_native_plugin::interface::UnityInterfaces),
    fn_unity_plugin_unload: fn(),
) {
    let mut event_loop = EventLoop::<u32>::new_any_thread();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let context = std::rc::Rc::new(fn_initialize(&window));
    unsafe {
        crate::interface::get_unity_interfaces()
            .register_interface::<Context>(Some(context.clone()));
    }

    fn_unity_plugin_load(unity_native_plugin::interface::UnityInterfaces::get());

    event_loop.run_return(|event, _, control_flow| {
        match event {
            Event::WindowEvent { window_id, event} => {
                if window_id == window.id() {
                    match event {
                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        _ => {
                            *control_flow = match fn_main(&window, context.deref()) {
                                LoopResult::Continue => ControlFlow::Wait,
                                _ => ControlFlow::Exit,
                            };
                        }
                    }
                }
            }
            _ => (),
        }
    });

    fn_unity_plugin_unload();
    fn_finalize(&window, context.deref());
}
