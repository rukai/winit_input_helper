//! A complex example demonstrating use of every API feature, runs on both desktop and web.
//! To run on desktop: `cargo run --example example`
//! To run on web: `cargo run-wasm --example example`
use winit::event::MouseButton;
use winit::event_loop::EventLoop;
use winit::keyboard::{Key, KeyCode};
use winit_input_helper::WinitInputHelper;

fn main() {
    platform::init();

    let mut input = WinitInputHelper::new();

    let event_loop = EventLoop::new().unwrap();

    let _window = platform::create_window(&event_loop);

    event_loop
        .run(move |event, elwt| {
            // Pass every event to the WinitInputHelper.
            // It will return true when the last event has been processed and it is time to run your application logic.
            if input.update(&event) {
                if input.key_released(KeyCode::KeyQ) || input.close_requested() || input.destroyed()
                {
                    log::info!("The application was requsted to close or the 'Q' key was pressed, quiting the application");
                    elwt.exit();
                    return;
                }

                // If you are taking input for a game or similar you should use physical keys.

                if input.key_pressed(KeyCode::KeyW) {
                    log::info!("The 'W' key (US layout) was pressed on the keyboard");
                }

                if input.key_pressed_os(KeyCode::KeyE) {
                    log::info!(
                        "The 'E' key (US layout) was pressed on the keyboard (Os Repeating)"
                    );
                }

                if input.key_held(KeyCode::KeyR) {
                    log::info!("The 'R' key (US layout) is held");
                }

                // Logical keys are usually used for text input and rarely make sense in the way they are presented in this API.

                if input.key_pressed_logical(Key::Character("a")) {
                    log::info!("'a' was input by the keyboard");
                }

                if input.key_pressed_logical(Key::Character("A")) {
                    log::info!("'A' was input by the keyboard (detected seperately to 'a')");
                }

                if input.key_pressed_os_logical(Key::Character("s")) {
                    log::info!("'s' was input by the keyboard (OS repeating)");
                }

                if input.key_held_logical(Key::Character("d")) {
                    log::info!("`d` input is held on the keyboard");
                }

                // query the change in cursor this update
                let cursor_diff = input.cursor_diff();
                if cursor_diff != (0.0, 0.0) {
                    log::info!("The cursor diff is: {:?}", cursor_diff);
                    log::info!("The cursor position is: {:?}", input.cursor());
                }

                // query the change in mouse this update (useful for first person camera controls)
                let mouse_diff = input.mouse_diff();
                if mouse_diff != (0.0, 0.0) {
                    log::info!("The mouse diff is: {:?}", mouse_diff);
                }

                let scroll_diff = input.scroll_diff();
                if scroll_diff != (0.0, 0.0) {
                    log::info!("The scroll diff is: {:?}", scroll_diff);
                }


                for button in [MouseButton::Left, MouseButton::Right, MouseButton::Middle] {
                    if input.mouse_pressed(button) {
                        log::info!("The {:?} mouse button was pressed", button);
                    }

                    if input.mouse_held(button) {
                        log::info!("The {:?} mouse button is being held", button);
                    }

                    if input.mouse_released(button) {
                        log::info!("The {:?} mouse button was released", button);
                    }
                }


                // You are expected to control your own timing within this block.
                // Usually via rendering with vsync.
                // render();
            }
        })
        .unwrap();
}

#[cfg(target_arch = "wasm32")]
mod platform {
    use winit::event_loop::EventLoop;
    use winit::platform::web::WindowBuilderExtWebSys;
    use winit::platform::web::WindowExtWebSys;
    use winit::window::{Window, WindowBuilder};

    pub fn create_window(event_loop: &EventLoop<()>) -> Window {
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

#[cfg(not(target_arch = "wasm32"))]
mod platform {
    use winit::event_loop::EventLoop;
    use winit::window::{Window, WindowBuilder};

    pub fn create_window(event_loop: &EventLoop<()>) -> Window {
        WindowBuilder::new().build(event_loop).unwrap()
    }

    pub fn init() {
        env_logger::init_from_env(env_logger::Env::default().filter_or("RUST_LOG", "info"));
    }
}
