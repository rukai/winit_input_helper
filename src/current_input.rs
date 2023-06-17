use winit::event::{
    ElementState, MouseButton, MouseScrollDelta, ScanCode, VirtualKeyCode, WindowEvent,
};

/// Stores a character or a backspace.
///
/// TODO: Either:
///  *   remove this struct and just use backspace character instead
///  *   move keypresses like Home, End, Left, Right, Up, Down, Return to this enum
///  (advantage of using this struct is it retains sub-frame keypress ordering)
#[derive(Clone)]
pub enum TextChar {
    Char(char),
    Back,
}

#[derive(Clone)]
pub struct CurrentInput {
    pub mouse_actions: Vec<MouseAction>,
    pub key_actions: Vec<KeyAction>,
    pub scancode_actions: Vec<ScanCodeAction>,
    pub key_held: [bool; 255],
    pub scancode_held: Vec<ScanCode>, // some scan codes are higher than 255 so using an array may be dangerous
    pub mouse_held: [bool; 255],
    pub mouse_point: Option<(f32, f32)>,
    pub mouse_point_prev: Option<(f32, f32)>,
    pub y_scroll_diff: f32,
    pub x_scroll_diff: f32,
    pub text: Vec<TextChar>,
}

impl CurrentInput {
    pub fn new() -> CurrentInput {
        CurrentInput {
            mouse_actions: vec![],
            key_actions: vec![],
            scancode_actions: vec![],
            key_held: [false; 255],
            scancode_held: vec![],
            mouse_held: [false; 255],
            mouse_point: None,
            mouse_point_prev: None,
            y_scroll_diff: 0.0,
            x_scroll_diff: 0.0,
            text: vec![],
        }
    }

    pub fn step(&mut self) {
        self.mouse_actions.clear();
        self.key_actions.clear();
        self.scancode_actions.clear();
        self.y_scroll_diff = 0.0;
        self.x_scroll_diff = 0.0;
        self.mouse_point_prev = self.mouse_point;
        self.text.clear();
    }

    pub fn handle_event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::KeyboardInput { input, .. } => match input.state {
                ElementState::Pressed => {
                    if let Some(keycode) = input.virtual_keycode {
                        if !self.key_held[keycode as usize] {
                            self.key_actions.push(KeyAction::Pressed(keycode));
                        }

                        self.key_held[keycode as usize] = true;
                        self.key_actions.push(KeyAction::PressedOs(keycode));
                        if let VirtualKeyCode::Back = keycode {
                            self.text.push(TextChar::Back);
                        }
                    }

                    let scancode = input.scancode;

                    if !self.scancode_held.contains(&scancode) {
                        self.scancode_actions
                            .push(ScanCodeAction::Pressed(scancode));
                        self.scancode_held.push(scancode);
                    }

                    self.scancode_actions
                        .push(ScanCodeAction::PressedOs(scancode));
                }
                ElementState::Released => {
                    if let Some(keycode) = input.virtual_keycode {
                        self.key_held[keycode as usize] = false;
                        self.key_actions.push(KeyAction::Released(keycode));
                    }

                    let scancode = input.scancode;
                    self.scancode_held.retain(|&x| x != scancode);
                    self.scancode_actions
                        .push(ScanCodeAction::Released(scancode));
                }
            },
            WindowEvent::ReceivedCharacter(c) => {
                let c = *c;
                if c != '\x08' && c != '\r' && c != '\n' {
                    self.text.push(TextChar::Char(c));
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.mouse_point = Some((position.x as f32, position.y as f32));
            }
            WindowEvent::MouseInput {
                state: ElementState::Pressed,
                button,
                ..
            } => {
                let button = mouse_button_to_int(button);
                self.mouse_held[button] = true;
                self.mouse_actions.push(MouseAction::Pressed(button));
            }
            WindowEvent::MouseInput {
                state: ElementState::Released,
                button,
                ..
            } => {
                let button = mouse_button_to_int(button);
                self.mouse_held[button] = false;
                self.mouse_actions.push(MouseAction::Released(button));
            }
            WindowEvent::MouseWheel { delta, .. } => {
                // I just took this from three-rs, no idea why this magic number was chosen ¯\_(ツ)_/¯
                const PIXELS_PER_LINE: f64 = 38.0;

                match delta {
                    MouseScrollDelta::LineDelta(x, y) => {
                        self.x_scroll_diff += x;
                        self.y_scroll_diff += y;
                    }
                    MouseScrollDelta::PixelDelta(delta) => {
                        self.y_scroll_diff += (delta.y / PIXELS_PER_LINE) as f32;
                        self.x_scroll_diff += (delta.x / PIXELS_PER_LINE) as f32;
                    }
                }
            }
            _ => {}
        }
    }
}

#[derive(Clone)]
pub enum KeyAction {
    Pressed(VirtualKeyCode),
    PressedOs(VirtualKeyCode),
    Released(VirtualKeyCode),
}

#[derive(Clone, PartialEq)]
pub enum ScanCodeAction {
    Pressed(ScanCode),
    PressedOs(ScanCode),
    Released(ScanCode),
}

#[derive(Clone)]
pub enum MouseAction {
    Pressed(usize),
    Released(usize),
}

fn mouse_button_to_int(button: &MouseButton) -> usize {
    match button {
        MouseButton::Left => 0,
        MouseButton::Right => 1,
        MouseButton::Middle => 2,
        MouseButton::Other(byte) => *byte as usize,
    }
}
