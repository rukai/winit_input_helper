use winit::event_loop::EventLoop;
use winit::keyboard::{Key, KeyCode};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

fn main() {
    let mut input = WinitInputHelper::new();

    let event_loop = EventLoop::new().unwrap();
    let _window = WindowBuilder::new().build(&event_loop).unwrap();

    event_loop
        .run(move |event, elwt| {
            // Pass every event to the WindowInputHelper.
            // It will return true when the last event has been processed and it is time to run your application logic.
            if input.update(&event) {
                if input.key_released(KeyCode::KeyQ) || input.close_requested() || input.destroyed()
                {
                    elwt.exit();
                    return;
                }

                // If you are taking input for a game or similar you should use physical keys.

                if input.key_pressed(KeyCode::KeyW) {
                    println!("The 'W' key (US layout) was pressed on the keyboard");
                }

                if input.key_pressed_os(KeyCode::KeyE) {
                    println!("The 'E' key (US layout) was pressed on the keyboard (Os Repeating)");
                }

                if input.key_held(KeyCode::KeyR) {
                    println!("The 'R' key (US layout) is held");
                }

                // Logical keys are usually used for text input and rarely make sense in the way they are presented in this API.

                if input.key_pressed_logical(Key::Character("a")) {
                    println!("'a' was input by the keyboard");
                }

                if input.key_pressed_logical(Key::Character("A")) {
                    println!("'A' was input by the keyboard (detected seperately to 'a')");
                }

                if input.key_pressed_os_logical(Key::Character("s")) {
                    println!("'s' was input by the keyboard (OS repeating)");
                }

                if input.key_held_logical(Key::Character("d")) {
                    println!("`d` input is held on the keyboard");
                }

                // query the change in cursor this update
                let cursor_diff = input.cursor_diff();
                if cursor_diff != (0.0, 0.0) {
                    println!("The cursor diff is: {:?}", cursor_diff);
                    println!("The cursor position is: {:?}", input.cursor());
                }

                // query the change in mouse this update (useful for first person camera controls)
                let mouse_diff = input.mouse_diff();
                if mouse_diff != (0.0, 0.0) {
                    println!("The mouse diff is: {:?}", mouse_diff);
                }

                let scroll_diff = input.scroll_diff();
                if scroll_diff != (0.0, 0.0) {
                    println!("The scroll diff is: {:?}", scroll_diff);
                }

                // You are expected to control your own timing within this block.
                // Usually via rendering with vsync.
                // render();
            }
        })
        .unwrap();
}
