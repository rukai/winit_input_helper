# Winit Input Helper

[![Crates.io](https://img.shields.io/crates/v/winit_input_helper.svg)](https://crates.io/crates/winit_input_helper)
[![Docs](https://docs.rs/winit_input_helper/badge.svg)](https://docs.rs/winit_input_helper)

Processes and stores winit events, allowing input state to be queried at any time.

## How to use

Each event is passed to the `WinitInputHelper` via the `update` method.

The current input state can then be accessed via methods such as `key_pressed`, `key_released`, `key_held`, `mouse`, `mouse_diff` etc.

To see all available methods look at [docs.rs](https://docs.rs/winit_input_helper)

```rust
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

                if input.key_pressed(KeyCode::KeyW) {
                    println!("The 'W' key (US layout) was pressed on the keyboard");
                }

                if input.key_held(KeyCode::KeyR) {
                    println!("The 'R' key (US layout) key is held");
                }

                // query the change in cursor this update
                let cursor_diff = input.cursor_diff();
                if cursor_diff != (0.0, 0.0) {
                    println!("The cursor diff is: {:?}", cursor_diff);
                    println!("The cursor position is: {:?}", input.cursor());
                }

                // You are expected to control your own timing within this block.
                // Usually via rendering with vsync.
                // render();
            }
        })
        .unwrap();
}
```

## Publishing a new version

In order to avoid forcing the user to enable the default winit backends, winit_input_helper sets its winit dependency to `default-features = false`.
This complicates the publishing procedure a little because winit cannot compile without any backends enabled.

So to publish we run: `cargo publish --features winit/default`
