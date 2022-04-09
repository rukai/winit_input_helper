use winit::event::{ElementState, MouseButton, MouseScrollDelta, VirtualKeyCode, WindowEvent};

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
    pub key_held: [bool; 255],
    pub mouse_held: [bool; 255],
    pub mouse_point: Option<(f32, f32)>,
    pub mouse_point_prev: Option<(f32, f32)>,
    pub scroll_diff: f32,
    pub text: Vec<TextChar>,
}

impl CurrentInput {
    pub fn new() -> CurrentInput {
        CurrentInput {
            mouse_actions: vec![],
            key_actions: vec![],
            key_held: [false; 255],
            mouse_held: [false; 255],
            mouse_point: None,
            mouse_point_prev: None,
            scroll_diff: 0.0,
            text: vec![],
        }
    }

    pub fn step(&mut self) {
        self.mouse_actions = vec![];
        self.key_actions = vec![];
        self.scroll_diff = 0.0;
        self.mouse_point_prev = self.mouse_point;
        self.text.clear();
    }

    pub fn handle_event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::KeyboardInput { input, .. } => {
                if let Some(keycode) = input.virtual_keycode {
                    match input.state {
                        ElementState::Pressed => {
                            if !self.key_held[keycode as usize] {
                                self.key_actions.push(KeyAction::Pressed(keycode));
                            }
                            self.key_held[keycode as usize] = true;
                            self.key_actions.push(KeyAction::PressedOs(keycode));
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
                    MouseScrollDelta::LineDelta(_, y) => {
                        self.scroll_diff += y;
                    }
                    MouseScrollDelta::PixelDelta(delta) => {
                        self.scroll_diff += (delta.y / PIXELS_PER_LINE) as f32
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
