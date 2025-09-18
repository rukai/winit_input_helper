//! A complex example demonstrating use of every API feature, runs on both desktop and web.
//! To run on desktop: `cargo run --example example`
//! To run on web: `cargo run-wasm --example example`
use winit::application::ApplicationHandler;
use winit::event::{DeviceEvent, DeviceId, MouseButton, StartCause, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{Key, KeyCode};
use winit::window::{Window, WindowId};
use winit_input_helper::WinitInputHelper;

struct App {
    input: WinitInputHelper,
    window: Option<Window>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // This method is called when the app is resumed, including when it
        //  is first started. If we do not have a window, we have to create one.
        if self.window.is_none() {
            self.window = Some(platform::create_window(event_loop));
        }
    }

    fn window_event(&mut self, _: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        // Pass every event to the WinitInputHelper.
        // It will return true if it receives a RequestedRedraw event: you should then render.
        if self.input.process_window_event(&event) {
            // render();

            // If you want to render every frame, remember to call window.request_redraw() in ApplicationHandler.about_to_wait().
        }
    }

    fn device_event(&mut self, _: &ActiveEventLoop, _: DeviceId, event: DeviceEvent) {
        self.input.process_device_event(&event);
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        self.input.end_step();

        // AFTER calling process_about_to_wait(), run your application logic here.
        // We do not call window.request_redraw() here because we have nothing to render anyways

        if self.input.key_released(KeyCode::KeyQ)
            || self.input.close_requested()
            || self.input.destroyed()
        {
            log::info!("The application was requsted to close or the 'Q' key was pressed, quiting the application");
            event_loop.exit();
            return;
        }

        // If you are taking input for a game or similar you should use physical keys.

        if self.input.key_pressed(KeyCode::KeyW) {
            log::info!("The 'W' key (US layout) was pressed on the keyboard");
        }

        if self.input.key_pressed_os(KeyCode::KeyE) {
            log::info!("The 'E' key (US layout) was pressed on the keyboard (Os Repeating)");
        }

        if self.input.key_held(KeyCode::KeyR) {
            log::info!("The 'R' key (US layout) is held");
        }

        // Logical keys are usually used for text input and rarely make sense in the way they are presented in this API.

        if self.input.key_pressed_logical(Key::Character("a")) {
            log::info!("'a' was input by the keyboard");
        }

        if self.input.key_pressed_logical(Key::Character("A")) {
            log::info!("'A' was input by the keyboard (detected seperately to 'a')");
        }

        if self.input.key_pressed_os_logical(Key::Character("s")) {
            log::info!("'s' was input by the keyboard (OS repeating)");
        }

        if self.input.key_held_logical(Key::Character("d")) {
            log::info!("`d` input is held on the keyboard");
        }

        // query the change in cursor this update
        let cursor_diff = self.input.cursor_diff();
        if cursor_diff != (0.0, 0.0) {
            log::info!("The cursor diff is: {:?}", cursor_diff);
            log::info!("The cursor position is: {:?}", self.input.cursor());
        }

        // query the change in mouse this update (useful for first person camera controls)
        let mouse_diff = self.input.mouse_diff();
        if mouse_diff != (0.0, 0.0) {
            log::info!("The mouse diff is: {:?}", mouse_diff);
        }

        let scroll_diff = self.input.scroll_diff();
        if scroll_diff != (0.0, 0.0) {
            log::info!("The scroll diff is: {:?}", scroll_diff);
        }

        for button in [MouseButton::Left, MouseButton::Right, MouseButton::Middle] {
            if self.input.mouse_pressed(button) {
                log::info!("The {:?} mouse button was pressed", button);
            }

            if self.input.mouse_held(button) {
                log::info!("The {:?} mouse button is being held", button);
            }

            if self.input.mouse_released(button) {
                log::info!("The {:?} mouse button was released", button);
            }
        }
    }

    fn new_events(&mut self, _: &ActiveEventLoop, _: StartCause) {
        self.input.step();
    }
}

fn main() {
    platform::init();

    // Create the event loop; the window will be created inside App when ApplicationHandler.resumed() is called.
    // We set ControlFlow to Poll because we want to print continuously in some cases.
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    // Run the app
    event_loop
        .run_app(&mut App {
            input: WinitInputHelper::new(),
            window: None,
        })
        .unwrap();
}

#[cfg(target_arch = "wasm32")]
mod platform {
    use winit::event_loop::ActiveEventLoop;
    use winit::platform::web::WindowAttributesExtWebSys;
    use winit::platform::web::WindowExtWebSys;
    use winit::window::Window;

    pub fn create_window(event_loop: &ActiveEventLoop) -> Window {
        let window_attrs = Window::default_attributes().with_append(true);
        let window = event_loop.create_window(window_attrs).unwrap();

        // Set a background color for the canvas to make it easier to tell the where the canvas is for debugging purposes.
        let canvas = window.canvas().unwrap();
        canvas.style().set_css_text(
            "display: block; background-color: crimson; margin: auto; width: 50%; aspect-ratio: 4 / 2;",
        );
        window
    }

    pub fn init() {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init_with_level(log::Level::Info).expect("could not initialize logger");
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod platform {
    use winit::event_loop::ActiveEventLoop;
    use winit::window::Window;

    pub fn create_window(event_loop: &ActiveEventLoop) -> Window {
        event_loop
            .create_window(Window::default_attributes())
            .unwrap()
    }

    pub fn init() {
        env_logger::init_from_env(env_logger::Env::default().filter_or("RUST_LOG", "info"));
    }
}
