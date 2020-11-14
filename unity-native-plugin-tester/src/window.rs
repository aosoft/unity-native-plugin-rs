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
    Context,
    FnInit: FnOnce(&Window) -> Context,
    FnMain: FnMut(&Window, &mut Context) -> LoopResult,
    FnFinalize: FnOnce(&Window, &mut Context),
>(
    fn_initialize: FnInit,
    mut fn_main: FnMain,
    fn_finalize: FnFinalize,
) {
    let mut event_loop = EventLoop::<u32>::new_any_thread();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut context = fn_initialize(&window);

    event_loop.run_return(|event, _, control_flow| {
        *control_flow = match fn_main(&window, &mut context) {
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

    fn_finalize(&window, &mut context);
}

