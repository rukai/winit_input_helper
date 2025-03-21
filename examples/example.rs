//! A complex example demonstrating use of every API feature, runs on both desktop and web.
//! To run on desktop: `cargo run --example example`
//! To run on web: `cargo run-wasm --example example`
use winit::application::ApplicationHandler;
use winit::event::MouseButton;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::keyboard::{Key, KeyCode};
use winit::window::Window;
use winit_input_helper::WinitInputHelper;

struct App {
    winit_input_helper: WinitInputHelper,
    window: Option<Window>,
}
impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        // This method is called when the app is resumed, including when it
        //  is first started. If we do not have a window, we have to create one.
        if let None = self.window {
            self.window = Some(platform::create_window(event_loop));
        }
    }
    fn window_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        // Pass every event to the WinitInputHelper.
        // It will return true if it receives a RequestedRedraw event: you should then render.
        if self.winit_input_helper.process_window_event(&event) {
            // render();

            // If you want to render every frame, remember to call window.request_redraw() in ApplicationHandler.about_to_wait().
        }
    }
    fn device_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        event: winit::event::DeviceEvent,
    ) {
        self.winit_input_helper.process_device_event(&event);
    }
    fn about_to_wait(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.winit_input_helper.end_step();

        // AFTER calling process_about_to_wait(), run your application logic here.
        // We do not call window.request_redraw() here because we have nothing to render anyways

        if self.winit_input_helper.key_released(KeyCode::KeyQ)
            || self.winit_input_helper.close_requested()
            || self.winit_input_helper.destroyed()
        {
            log::info!("The application was requsted to close or the 'Q' key was pressed, quiting the application");
            event_loop.exit();
            return;
        }

        // If you are taking input for a game or similar you should use physical keys.

        if self.winit_input_helper.key_pressed(KeyCode::KeyW) {
            log::info!("The 'W' key (US layout) was pressed on the keyboard");
        }

        if self.winit_input_helper.key_pressed_os(KeyCode::KeyE) {
            log::info!("The 'E' key (US layout) was pressed on the keyboard (Os Repeating)");
        }

        if self.winit_input_helper.key_held(KeyCode::KeyR) {
            log::info!("The 'R' key (US layout) is held");
        }

        // Logical keys are usually used for text input and rarely make sense in the way they are presented in this API.

        if self
            .winit_input_helper
            .key_pressed_logical(Key::Character("a"))
        {
            log::info!("'a' was input by the keyboard");
        }

        if self
            .winit_input_helper
            .key_pressed_logical(Key::Character("A"))
        {
            log::info!("'A' was input by the keyboard (detected seperately to 'a')");
        }

        if self
            .winit_input_helper
            .key_pressed_os_logical(Key::Character("s"))
        {
            log::info!("'s' was input by the keyboard (OS repeating)");
        }

        if self
            .winit_input_helper
            .key_held_logical(Key::Character("d"))
        {
            log::info!("`d` input is held on the keyboard");
        }

        // query the change in cursor this update
        let cursor_diff = self.winit_input_helper.cursor_diff();
        if cursor_diff != (0.0, 0.0) {
            log::info!("The cursor diff is: {:?}", cursor_diff);
            log::info!(
                "The cursor position is: {:?}",
                self.winit_input_helper.cursor()
            );
        }

        // query the change in mouse this update (useful for first person camera controls)
        let mouse_diff = self.winit_input_helper.mouse_diff();
        if mouse_diff != (0.0, 0.0) {
            log::info!("The mouse diff is: {:?}", mouse_diff);
        }

        let scroll_diff = self.winit_input_helper.scroll_diff();
        if scroll_diff != (0.0, 0.0) {
            log::info!("The scroll diff is: {:?}", scroll_diff);
        }

        for button in [MouseButton::Left, MouseButton::Right, MouseButton::Middle] {
            if self.winit_input_helper.mouse_pressed(button) {
                log::info!("The {:?} mouse button was pressed", button);
            }

            if self.winit_input_helper.mouse_held(button) {
                log::info!("The {:?} mouse button is being held", button);
            }

            if self.winit_input_helper.mouse_released(button) {
                log::info!("The {:?} mouse button was released", button);
            }
        }
    }
    fn new_events(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _cause: winit::event::StartCause,
    ) {
        self.winit_input_helper.step();
    }
}

fn main() {
    platform::init();

    // Initialize the WinitInputHelper at the beginning
    let winit_input_helper = WinitInputHelper::new();

    // Create the event loop; the window will be created inside App
    //  when ApplicationHandler.resumed() is called.
    // We set ControlFlow to Poll because we want to print continuously in some cases.
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    // Run the app
    event_loop
        .run_app(&mut App {
            winit_input_helper,
            window: None,
        })
        .unwrap();
}

// This example has not been updated to work on the web in the new winit 0.30 update,
// because the person writing this comment was unable to find the documentation for the platform-specific web module.
// Pull requests are welcome.
/*
#[cfg(target_arch = "wasm32")]
mod platform {
    use winit::event_loop::EventLoop;
    use winit::platform::web::WindowBuilderExtWebSys;
    use winit::platform::web::WindowExtWebSys;
    use winit::window::{Window, WindowBuilder};

    pub fn create_window(event_loop: &ActiveEventLoop<()>) -> Window {
        let window = WindowBuilder::new()
            .with_append(true)
            .build(event_loop)
            .unwrap();

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
*/

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
