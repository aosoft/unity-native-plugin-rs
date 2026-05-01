use std::ops::Deref;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

#[cfg(target_os = "windows")]
use winit::platform::windows::EventLoopBuilderExtWindows;

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

struct App<Context, FnInit, FnMain, FnFinalize>
where
    Context: 'static + crate::interface::UnityInterfaceBase + crate::interface::UnityInterfaceID,
    FnInit: FnOnce(&Window) -> Context,
    FnMain: FnMut(&Window, &Context) -> LoopResult,
    FnFinalize: FnOnce(&Window, &Context),
{
    client_size: (u32, u32),
    fn_initialize: Option<FnInit>,
    fn_main: FnMain,
    fn_finalize: Option<FnFinalize>,
    fn_unity_plugin_load: fn(interfaces: &unity_native_plugin::interface::UnityInterfaces),
    window: Option<Window>,
    context: Option<std::rc::Rc<Context>>,
    last_result: LoopResult,
}

impl<Context, FnInit, FnMain, FnFinalize> App<Context, FnInit, FnMain, FnFinalize>
where
    Context: 'static + crate::interface::UnityInterfaceBase + crate::interface::UnityInterfaceID,
    FnInit: FnOnce(&Window) -> Context,
    FnMain: FnMut(&Window, &Context) -> LoopResult,
    FnFinalize: FnOnce(&Window, &Context),
{
    fn update_control_flow(&self, event_loop: &ActiveEventLoop) {
        match self.last_result {
            LoopResult::Continue => {
                event_loop.set_control_flow(ControlFlow::WaitUntil(
                    std::time::Instant::now() + std::time::Duration::from_millis(50),
                ));
            }
            LoopResult::ContinueOnWindowEvent => {
                event_loop.set_control_flow(ControlFlow::Wait);
            }
            LoopResult::Exit => {
                event_loop.exit();
            }
        }
    }
}

impl<Context, FnInit, FnMain, FnFinalize> ApplicationHandler<u32>
    for App<Context, FnInit, FnMain, FnFinalize>
where
    Context: 'static + crate::interface::UnityInterfaceBase + crate::interface::UnityInterfaceID,
    FnInit: FnOnce(&Window) -> Context,
    FnMain: FnMut(&Window, &Context) -> LoopResult,
    FnFinalize: FnOnce(&Window, &Context),
{
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }
        let attrs = Window::default_attributes()
            .with_inner_size(winit::dpi::PhysicalSize::<u32>::from(self.client_size));
        let window = event_loop.create_window(attrs).unwrap();

        let fn_init = self.fn_initialize.take().unwrap();
        let context = std::rc::Rc::new(fn_init(&window));
        unsafe {
            crate::interface::get_unity_interfaces()
                .register_interface::<Context>(Some(context.clone()));
        }
        (self.fn_unity_plugin_load)(unity_native_plugin::interface::UnityInterfaces::get());

        self.window = Some(window);
        self.context = Some(context);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let Some(window) = self.window.as_ref() else {
            return;
        };
        if window_id != window.id() {
            return;
        }
        let context = self.context.as_ref().unwrap();
        match event {
            WindowEvent::CloseRequested => {
                self.last_result = LoopResult::Exit;
            }
            _ => {
                self.last_result = (self.fn_main)(window, context.deref());
            }
        }
        self.update_control_flow(event_loop);
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        let (Some(window), Some(context)) = (self.window.as_ref(), self.context.as_ref()) else {
            return;
        };
        if self.last_result == LoopResult::Continue {
            self.last_result = (self.fn_main)(window, context.deref());
        }
        self.update_control_flow(event_loop);
    }
}

pub fn run_window_app<
    Context: 'static + crate::interface::UnityInterfaceBase + crate::interface::UnityInterfaceID,
    FnInit: FnOnce(&Window) -> Context,
    FnMain: FnMut(&Window, &Context) -> LoopResult,
    FnFinalize: FnOnce(&Window, &Context),
>(
    client_size: (u32, u32),
    fn_initialize: FnInit,
    fn_main: FnMain,
    fn_finalize: FnFinalize,
    fn_unity_plugin_load: fn(interfaces: &unity_native_plugin::interface::UnityInterfaces),
    fn_unity_plugin_unload: fn(),
) {
    let mut builder = EventLoop::<u32>::with_user_event();
    #[cfg(target_os = "windows")]
    builder.with_any_thread(true);
    let event_loop = builder.build().unwrap();

    let mut app = App {
        client_size,
        fn_initialize: Some(fn_initialize),
        fn_main,
        fn_finalize: Some(fn_finalize),
        fn_unity_plugin_load,
        window: None,
        context: None,
        last_result: LoopResult::Continue,
    };

    event_loop.run_app(&mut app).unwrap();

    fn_unity_plugin_unload();
    if let (Some(fn_finalize), Some(window), Some(context)) = (
        app.fn_finalize.take(),
        app.window.as_ref(),
        app.context.as_ref(),
    ) {
        fn_finalize(window, context.deref());
    }
}
