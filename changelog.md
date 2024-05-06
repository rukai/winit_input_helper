# Changelog

This changelog is written with the goal of helping you through breaking changes rather than being a complete documentation of every change in the release.

## 0.17

Upgraded to winit 0.30. Overhauled the API to be an `ApplicationHandler` wrapper.

```rust
// Old
let mut input = WinitInputHelper::new();
event_loop
        .run(move |event, elwt| {
            if input.update(&event) {
              ...
// New
let mut winit_input = WinitInputApplicationHandler::new(your_application_handler);
event_loop.run_app(&mut winit_input).unwrap();

impl WinitInputUpdate for State {
    fn update(&mut self, event_loop: &ActiveEventLoop, input: &WinitInputHelper) {
       ...
    }
}
```

## 0.16

* `WinitInputHelper::quit` is removed, instead use `input.close_requested() || input.destroyed()` for an equivalent check
* Mouse APIs now use <https://docs.rs/winit/latest/winit/event/enum.MouseButton.html> instead of a usize
* `WinitInputHeler::text` now returns <https://docs.rs/winit/latest/winit/keyboard/enum.Key.html> instead of the now deleted `TextChar` type

## 0.15

Upgraded to winit 0.29.
Winit 0.29 completely overhauled its keyboard API, which meant that I had to also overhaul our keyboard API.

Previously winit_input_helper favored logical keys over physical keys (previously called scancodes).
But this was a mistake, driven by winit's poor support for physical keys and mistaken simplification of logical keys.
Winit has now fixed these mistakes and as a result winit_input_helper is now swapping to favor physical keys.

A direct translation of the previous API to the new API:

```rust

// old
input.key_pressed_scancode(17); // US scan code for W
// new
input.key_presed(winit::keyboard::KeyCode::KeyW);

// old
input.key_pressed(winit::event::VirtualKeyCode::KeyW);
// new
input.key_presed_logical(winit::keyboard::Key::Character("w")); // WARNING: this likely wont actually do what you want, this will never return true while shift is held since that is considered as `W` instead of `w`

// ... other keyboard methods follow the same pattern
```

However, I actually suggest you move to physical keys:

```rust
// old
input.key_pressed_scancode(17); // US scan code for W
// new
input.key_presed(winit::keyboard::KeyCode::KeyW);

// old
input.key_pressed(winit::event::VirtualKeyCode::W);
// new
input.key_presed(winit::keyboard::KeyCode::KeyW);
```
