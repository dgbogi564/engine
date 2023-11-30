use ash::Entry;
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

pub(crate) struct Engine {
    window: Window,
    event_loop: EventLoop<()>,
}

/// https://vulkan-tutorial.com/Development_environment
/// https://docs.rs/ash/latest/ash/
/// https://docs.rs/winit/latest/winit/
impl Engine {
    pub(crate) fn new(
        width: Option<u32>,
        height: Option<u32>,
        window_name: Option<&str>,
    ) -> Engine {
        let (window, event_loop) = Engine::init_window(
            width.unwrap_or(800),
            height.unwrap_or(600),
            window_name.unwrap_or("engine window"),
        );

        Engine {
            window,
            event_loop,
        }
    }

    fn init_window(width: u32, height: u32, name: &str) -> (Window, EventLoop<()>) {
        let event_loop = EventLoop::new().unwrap();
        let window = WindowBuilder::new()
            .with_title(name)
            .with_inner_size(LogicalSize::new(width, height))
            .with_resizable(false)
            .build(&event_loop)
            .unwrap();

        event_loop.set_control_flow(ControlFlow::Poll);

        return (window, event_loop);
    }

    pub(crate) fn run(self) {
        let entry = Entry::linked();

        // Print number of supported extensions
        let extensions = entry.enumerate_instance_extension_properties(None).unwrap();
        let extension_count = extensions.len();
        print!("{extension_count} extensions supported\n");

        // Run window event loop
        let _ = self.event_loop.run(move |event, elwt| {
            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        elwt.exit();
                    }
                    WindowEvent::KeyboardInput { .. } => {}
                    _ => {}
                },
                Event::UserEvent(_) => {}
                Event::AboutToWait => {}
                _ => {}
            };
        });
    }
}
