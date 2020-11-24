use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

use std::ops::Deref;
use winit::platform::desktop::EventLoopExtDesktop;
#[cfg(target_os = "windows")]
use winit::platform::windows::EventLoopExtWindows;

#[derive(PartialEq, Eq)]
pub enum LoopResult {
    /// Continue the loop. The next process is executed immediately.
    Continue,

    /// Continue the loop. The next process is executed when a window event occurs.
    /// (e.g., moving the mouse over a window)
    /// Use when the process to be implemented is high load.
    ContinueOnWindowEvent,

    /// End the loop.
    Exit,
}

pub fn run_window_app<
    Context: 'static + crate::interface::UnityInterfaceBase + crate::interface::UnityInterfaceID,
    FnInit: FnOnce(&Window) -> Context,
    FnMain: FnMut(&Window, &Context) -> LoopResult,
    FnFinalize: FnOnce(&Window, &Context),
>(
    client_size: (u32, u32),
    fn_initialize: FnInit,
    mut fn_main: FnMain,
    fn_finalize: FnFinalize,
    fn_unity_plugin_load: fn(interfaces: &unity_native_plugin::interface::UnityInterfaces),
    fn_unity_plugin_unload: fn(),
) {
    let mut event_loop = EventLoop::<u32>::new_any_thread();
    let window = WindowBuilder::new()
        .with_inner_size(winit::dpi::Size::from(
            winit::dpi::PhysicalSize::<u32>::from(client_size),
        ))
        .build(&event_loop)
        .unwrap();

    let context = std::rc::Rc::new(fn_initialize(&window));
    unsafe {
        crate::interface::get_unity_interfaces()
            .register_interface::<Context>(Some(context.clone()));
    }

    fn_unity_plugin_load(unity_native_plugin::interface::UnityInterfaces::get());

    let mut last_result = LoopResult::Continue;
    event_loop.run_return(|event, _, control_flow| {
        let instant = std::time::Instant::now();
        match event {
            Event::WindowEvent { window_id, event } => {
                if window_id == window.id() {
                    match event {
                        WindowEvent::CloseRequested => last_result = LoopResult::Exit,
                        _ => {
                            last_result = fn_main(&window, context.deref());
                        }
                    }
                }
            }
            _ => {
                if last_result == LoopResult::Continue {
                    last_result = fn_main(&window, context.deref());
                }
            },
        }
        *control_flow = match last_result {
            LoopResult::Continue => ControlFlow::WaitUntil(instant + std::time::Duration::from_millis(50)),
            LoopResult::ContinueOnWindowEvent => ControlFlow::Wait,
            _ => ControlFlow::Exit,
        };
    });

    fn_unity_plugin_unload();
    fn_finalize(&window, context.deref());
}
