use winit::event::VirtualKeyCode;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

fn main() {
    let mut input = WinitInputHelper::new();

    let event_loop = EventLoop::new();
    let _window = WindowBuilder::new().build(&event_loop).unwrap();

    event_loop.run(move |event, _, control_flow| {
        // Pass every event to the WindowInputHelper.
        // It will return true when the last event has been processed and it is time to run your application logic.
        if input.update(&event) {
            // query keypresses this update
            if input.key_pressed_os(VirtualKeyCode::A) {
                println!("The 'A' key was pressed on the keyboard (OS repeating)");
            }

            if input.key_pressed(VirtualKeyCode::A) {
                println!("The 'A' key was pressed on the keyboard");
            }

            if input.key_released(VirtualKeyCode::Q) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // query the change in cursor this update
            let cursor_diff = input.cursor_diff();
            if cursor_diff != (0.0, 0.0) {
                println!("The cursor diff is: {:?}", cursor_diff);
                println!("The cursor position is: {:?}", input.cursor());
            }

            //query the change in mouse this update (useful for camera)
            let mouse_diff = input.mouse_diff();
            println!("The mouse diff is: {:?}", mouse_diff);

            // You are expected to control your own timing within this block.
            // Usually via rendering with vsync.
            // render();
        }
    });
}
