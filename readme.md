# Winit Input Helper
[![Build Status](https://travis-ci.org/rukai/winit_input_helper.svg?branch=master)](https://travis-ci.org/rukai/winit_input_helper) [![dependency status](https://deps.rs/repo/github/rukai/winit_input_helper/status.svg)](https://deps.rs/repo/github/rukai/winit_input_helper) [![Crates.io](https://img.shields.io/crates/v/winit_input_helper.svg)](https://crates.io/crates/winit_input_helper) [![Docs](https://docs.rs/winit_input_helper/badge.svg)](https://docs.rs/winit_input_helper)

Processes and store winit events, allowing input state to be queried at any time.

## How to use

The `WinitInputHelper` struct takes events via the `update` or `update_from_vec` methods.
The current input state can then be accessed via methods such as `key_pressed`, `key_released`, `key_held`, `mouse`, `mouse_diff` etc.

To see all available methods look at [docs.rs](https://docs.rs/winit_input_helper)

```rust
extern crate winit;
extern crate winit_input_helper;

use winit::{EventsLoop, VirtualKeyCode};
use winit_input_helper::WinitInputHelper;

fn main() {
    let mut input = WinitInputHelper::new();

    let mut events_loop = EventsLoop::new();
    let _window = winit::WindowBuilder::new().build(&events_loop).unwrap();

    loop {
        // call this once per loop
        input.update(&mut events_loop);

        // call these as many times as you want
        if input.key_pressed(VirtualKeyCode::A) {
            println!("The 'A' key was pressed on the keyboard");
        }
    }
}
```
