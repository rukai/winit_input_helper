# Winit Input Helper
[![Build Status](https://travis-ci.org/rukai/winit_input_helper.svg?branch=master)](https://travis-ci.org/rukai/winit_input_helper) [![Crates.io](https://img.shields.io/crates/v/winit_input_helper.svg)](https://crates.io/crates/winit_input_helper) [![Docs](https://docs.rs/winit_input_helper/badge.svg)](https://docs.rs/winit_input_helper)

Processes and stores winit events, allowing input state to be queried at any time.

## How to use

Each event is passed to the `WinitInputHelper` via the `update` method.

The current input state can then be accessed via methods such as `key_pressed`, `key_released`, `key_held`, `mouse`, `mouse_diff` etc.

To see all available methods look at [docs.rs](https://docs.rs/winit_input_helper)

```rust
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
            if input.key_pressed(VirtualKeyCode::A) {
                println!("The 'A' key was pressed on the keyboard");
            }

            if input.key_released(VirtualKeyCode::Q) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // query the change in mouse this update
            let mouse_diff = input.mouse_diff();
            if mouse_diff != (0.0, 0.0) {
                println!("The mouse diff is: {:?}", mouse_diff);
                println!("The mouse position is: {:?}", input.mouse());
            }

            // You are expected to control your own timing within this block.
            // Usually via rendering with vsync.
            // render();
        }
    });
}
```
