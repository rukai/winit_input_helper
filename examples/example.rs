use winit::{EventsLoop, VirtualKeyCode};
use winit_input_helper::WinitInputHelper;

fn main() {
    let mut input = WinitInputHelper::new();

    let mut events_loop = EventsLoop::new();
    let _window = winit::WindowBuilder::new().build(&events_loop).unwrap();

    while !input.key_released(VirtualKeyCode::Q) {
        // call this once per loop
        input.update(&mut events_loop);

        // call these as many times as you want
        if input.key_pressed(VirtualKeyCode::A) {
            println!("The 'A' key was pressed on the keyboard");
        }

        let mouse_diff = input.mouse_diff();
        if mouse_diff != (0.0, 0.0) {
            println!("The mouse diff is: {:?}", mouse_diff);
            println!("The mouse position is: {:?}", input.mouse());
        }
    }
}
