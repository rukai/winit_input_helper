use winit::event::{Event, WindowEvent, VirtualKeyCode, ElementState, MouseButton, MouseScrollDelta};
use winit::dpi::PhysicalSize;

use std::path::PathBuf;

/// The main struct of the API.
///
/// Create with `WinitInputHelper::new`.
/// Call `update` for every `winit::event::Event` you receive from winit.
/// Run your application logic when `update` returns true, callng any of the accessor methods you need.
#[derive(Clone)]
pub struct WinitInputHelper {
    current:               Option<CurrentInput>,
    dropped_file:          Option<PathBuf>,
    window_resized:        Option<PhysicalSize<u32>>,
    scale_factor_changed:  Option<f64>,
    quit:                  bool,
}

impl WinitInputHelper {
    pub fn new() -> WinitInputHelper {
        WinitInputHelper {
            current:              Some(CurrentInput::new()),
            dropped_file:         None,
            window_resized:       None,
            scale_factor_changed: None,
            quit:                 false,
        }
    }

    /// Pass every winit event to this function and run your application logic when it returns true.
    ///
    /// The following winit events are handled:
    /// *   `Event::NewEvents` clears all internal state.
    /// *   `Event::MainEventsCleared` causes this function to return true.
    /// *   `Event::WindowEvent` updates internal state, this will affect the result of accessor methods immediately.
    pub fn update<T>(&mut self, event: Event<T>) -> bool {
        match &event {
            Event::NewEvents (_) => {
                self.dropped_file = None;
                self.window_resized = None;
                self.scale_factor_changed = None;
                if let Some(ref mut current) = self.current {
                    current.step();
                }

                false
            }
            Event::MainEventsCleared => {
                true
            }
            _ => {
                if let Event::WindowEvent { event, .. } = event {
                    match event {
                        WindowEvent::CloseRequested |
                        WindowEvent::Destroyed                               => { self.quit = true }
                        WindowEvent::Focused (false)                         => { self.current = None }
                        WindowEvent::Focused (true)                          => { self.current = Some(CurrentInput::new()) }
                        WindowEvent::DroppedFile (ref path)                  => { self.dropped_file = Some(path.clone()) }
                        WindowEvent::Resized (ref size)                      => { self.window_resized = Some(size.clone()) }
                        WindowEvent::ScaleFactorChanged { scale_factor, .. } => { self.scale_factor_changed = Some(scale_factor) }
                        _ => { }
                    }
                    if let Some(ref mut current) = self.current {
                        current.handle_event(event);
                    }
                }

                false
            }
        }
    }

    /// Returns true when the specified keyboard key goes from "not pressed" to "pressed"
    /// Otherwise returns false
    pub fn key_pressed(&self, check_key_code: VirtualKeyCode) -> bool {
        if let Some(ref current) = self.current {
            for action in &current.key_actions {
                if let &KeyAction::Pressed(key_code) = action {
                    if key_code == check_key_code {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// Returns true when the specified mouse button goes from "not pressed" to "pressed"
    /// Otherwise returns false
    ///
    /// Left   => 0
    /// Right  => 1
    /// Middle => 2
    /// Other  => 3..255
    pub fn mouse_pressed(&self, check_mouse_button: usize) -> bool {
        // TODO: Take MouseButton instead of usize
        if let Some(ref current) = self.current {
            for action in &current.mouse_actions {
                if let &MouseAction::Pressed(key_code) = action {
                    if key_code == check_mouse_button {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// Returns true when the specified keyboard key goes from "pressed" to "not pressed"
    /// Otherwise returns false
    pub fn key_released(&self, check_key_code: VirtualKeyCode) -> bool {
        if let Some(ref current) = self.current {
            for action in &current.key_actions {
                if let &KeyAction::Released(key_code) = action {
                    if key_code == check_key_code {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// Returns true when the specified mouse button goes from "pressed" to "not pressed"
    /// Otherwise returns false
    ///
    /// Left   => 0
    /// Right  => 1
    /// Middle => 2
    /// Other  => 3..255
    pub fn mouse_released(&self, check_mouse_button: usize) -> bool {
        // TODO: Take MouseButton instead of usize
        if let Some(ref current) = self.current {
            for action in &current.mouse_actions {
                if let &MouseAction::Released(key_code) = action {
                    if key_code == check_mouse_button {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// Returns true while the specified keyboard key remains "pressed"
    /// Otherwise returns false
    pub fn key_held(&self, key_code: VirtualKeyCode) -> bool {
        match self.current {
            Some (ref current) => current.key_held[key_code as usize],
            None               => false
        }
    }

    /// Returns true while the specified mouse button remains "pressed"
    /// Otherwise returns false
    ///
    /// Left   => 0
    /// Right  => 1
    /// Middle => 2
    /// Other  => 3..255
    pub fn mouse_held(&self, mouse_button: usize) -> bool {
        // TODO: Take MouseButton instead of usize
        match self.current {
            Some (ref current) => current.mouse_held[mouse_button as usize],
            None               => false
        }
    }

    /// Returns true while any shift key is held on the keyboard
    /// Otherwise returns false
    pub fn held_shift(&self) -> bool {
        return self.key_held(VirtualKeyCode::LShift) || self.key_held(VirtualKeyCode::RShift);
    }

    /// Returns true while any control key is held on the keyboard
    /// Otherwise returns false
    pub fn held_control(&self) -> bool {
        return self.key_held(VirtualKeyCode::LControl) || self.key_held(VirtualKeyCode::RControl);
    }

    /// Returns true while any alt key is held on the keyboard
    /// Otherwise returns false
    pub fn held_alt(&self) -> bool {
        return self.key_held(VirtualKeyCode::LAlt) || self.key_held(VirtualKeyCode::RAlt);
    }

    /// Returns `0.0` if the mouse is outside of the window.
    /// Otherwise returns the amount scrolled by the mouse in between the last two `update*()` calls
    pub fn scroll_diff(&self) -> f32 {
        match self.current {
            Some(ref current) => current.scroll_diff,
            None              => 0.0
        }
    }

    /// Returns `None` when the mouse is outside of the window.
    /// Otherwise returns the mouse coordinates in pixels
    pub fn mouse(&self) -> Option<(f32, f32)> {
        match self.current {
            Some(ref current) => current.mouse_point,
            None              => None
        }
    }

    /// Returns the difference in mouse coordinates between the last two `update*()` calls
    /// Returns `(0.0, 0.0)` if the mouse is outside of the window.
    pub fn mouse_diff(&self) -> (f32, f32) {
        if let Some(ref current_input) = self.current {
            if let Some(cur) = current_input.mouse_point {
                if let Some(prev) = current_input.mouse_point_prev {
                    return (cur.0 - prev.0, cur.1 - prev.1);
                }
            }
        }
        (0.0, 0.0)
    }

    /// Returns `None` when the mouse is outside of the window.
    /// Otherwise returns the resolution of the window.
    pub fn resolution(&self) -> Option<(u32, u32)> {
        match self.current {
            Some(ref current) => Some(current.resolution),
            None              => None
        }
    }

    /// Returns the characters pressed since the last `update*()`.
    /// The earlier the character was pressed, the lower the index in the Vec.
    pub fn text(&self) -> Vec<TextChar> {
        match self.current {
            Some(ref current) => current.text.clone(),
            None              => vec!()
        }
    }

    /// Returns the path to a file that has been drag-and-dropped onto the window.
    pub fn dropped_file(&self) -> Option<PathBuf> {
        self.dropped_file.clone()
    }

    /// Returns the current window size if it was resized between the last two `update*()` calls.
    /// Otherwise returns `None`
    pub fn window_resized(&self) -> Option<PhysicalSize<u32>> {
        self.window_resized.clone()
    }

    /// Returns the current window size if it was resized between the last two `update*()` calls.
    /// Otherwise returns `None`
    pub fn scale_factor_changed(&self) -> Option<f64> {
        self.scale_factor_changed
    }

    /// Returns true if the OS has requested the application to quit.
    /// Otherwise returns false.
    pub fn quit(&self) -> bool {
        self.quit
    }
}

/// Stores a character or a backspace.
///
/// TODO: Either:
///  *   remove this struct and just use backspace character instead
///  *   move keypresses like Home, End, Left, Right, Up, Down, Return to this enum
///  (advantage of using this struct is it retains sub-frame keypress ordering)
#[derive(Clone)]
pub enum TextChar {
    Char (char),
    Back,
}

#[derive(Clone)]
struct CurrentInput {
    pub mouse_actions:    Vec<MouseAction>,
    pub key_actions:      Vec<KeyAction>,
    pub key_held:         [bool; 255],
    pub mouse_held:       [bool; 255],
    pub mouse_point:      Option<(f32, f32)>,
    pub mouse_point_prev: Option<(f32, f32)>,
    pub scroll_diff:      f32,
    pub scale_factor:     f64,
    pub resolution:       (u32, u32),
    pub text:             Vec<TextChar>,
}

impl CurrentInput {
    pub fn new() -> CurrentInput {
        CurrentInput {
            mouse_actions:    vec!(),
            key_actions:      vec!(),
            key_held:         [false; 255],
            mouse_held:       [false; 255],
            mouse_point:      None,
            mouse_point_prev: None,
            scroll_diff:      0.0,
            scale_factor:     1.0,
            resolution:       (1, 1),
            text:             vec!(),
        }
    }

    pub fn step(&mut self) {
        self.mouse_actions    = vec!();
        self.key_actions      = vec!();
        self.scroll_diff      = 0.0;
        self.mouse_point_prev = self.mouse_point;
        self.text.clear();
    }

    pub fn handle_event(&mut self, event: WindowEvent) {
        match event {
            WindowEvent::KeyboardInput { input, .. } => {
                if let Some(keycode) = input.virtual_keycode {
                    match input.state {
                        ElementState::Pressed => {
                            self.key_held[keycode as usize] = true;
                            self.key_actions.push(KeyAction::Pressed(keycode));
                            if let VirtualKeyCode::Back = keycode {
                                self.text.push(TextChar::Back);
                            }
                        }
                        ElementState::Released => {
                            self.key_held[keycode as usize] = false;
                            self.key_actions.push(KeyAction::Released(keycode));
                        }
                    }
                }
            }
            WindowEvent::ReceivedCharacter (c) => {
                if c != '\x08' && c != '\r' && c != '\n' {
                    self.text.push(TextChar::Char(c));
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.mouse_point = Some((position.x as f32, position.y as f32));
            }
            WindowEvent::MouseInput { state: ElementState::Pressed, button, .. } => {
                let button = mouse_button_to_int(button);
                self.mouse_held[button] = true;
                self.mouse_actions.push(MouseAction::Pressed(button));
            }
            WindowEvent::MouseInput { state: ElementState::Released, button, .. } => {
                let button = mouse_button_to_int(button);
                self.mouse_held[button] = false;
                self.mouse_actions.push(MouseAction::Released(button));
            }
            WindowEvent::MouseWheel { delta, .. } => {
                // I just took this from three-rs, no idea why this magic number was chosen ¯\_(ツ)_/¯
                const PIXELS_PER_LINE: f64 = 38.0;

                match delta {
                    MouseScrollDelta::LineDelta  (_, y) => { self.scroll_diff += y; }
                    MouseScrollDelta::PixelDelta (delta) => { self.scroll_diff += (delta.y / PIXELS_PER_LINE) as f32 }
                }
            }
            WindowEvent::Resized (resolution) => {
                self.resolution = resolution.into();
            }
            WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                self.scale_factor = scale_factor;
            }
            _ => {}
        }
    }
}

#[derive(Clone)]
enum KeyAction {
    Pressed  (VirtualKeyCode),
    Released (VirtualKeyCode),
}

#[derive(Clone)]
enum MouseAction {
    Pressed (usize),
    Released (usize),
}

fn mouse_button_to_int(button: MouseButton) -> usize {
    match button {
        MouseButton::Left        => 0,
        MouseButton::Right       => 1,
        MouseButton::Middle      => 2,
        MouseButton::Other(byte) => byte as usize
    }
}
