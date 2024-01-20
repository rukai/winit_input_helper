//! The simplest example, supporting only desktop applications.

use winit::event_loop::EventLoop;
use winit::keyboard::KeyCode;
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

fn main() {
    let mut input = WinitInputHelper::new();

    let event_loop = EventLoop::new().unwrap();
    let _window = WindowBuilder::new().build(&event_loop).unwrap();

    event_loop
        .run(move |event, elwt| {
            // Pass every event to the WinitInputHelper.
            // It will return true when the last event has been processed and it is time to run your application logic.
            if input.update(&event) {
                if input.key_released(KeyCode::KeyQ) || input.close_requested() || input.destroyed()
                {
                    println!("The application was requsted to close or the 'Q' key was pressed, quiting the application");
                    elwt.exit();
                    return;
                }

                if input.key_pressed(KeyCode::KeyW) {
                    println!("The 'W' key (US layout) was pressed on the keyboard");
                }


                // You are expected to control your own timing within this block.
                // Usually via rendering with vsync.
                // render();
            }
        })
        .unwrap();
}
