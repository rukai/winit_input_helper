use winit::dpi::PhysicalSize;
use winit::event::{Event, VirtualKeyCode, WindowEvent};

use crate::current_input::{CurrentInput, KeyAction, MouseAction, TextChar};
use std::path::PathBuf;

/// The main struct of the API.
///
/// Create with `WinitInputHelper::new`.
/// Call `WinitInputHelper::update` for every `winit::event::Event` you receive from winit.
/// `WinitInputHelper::update` returning true indicates a step has occured.
/// You should now run your application logic, calling any of the accessor methods you need.
///
/// An alternative API is provided via `WinitInputHelper::step_with_window_events`,
/// call this method instead of `WinitInputHelper::update` if you need to manually control when a new step begins.
/// A step occurs every time this method is called.
///
/// Do not mix usages of `WinitInputHelper::update` and `WinitInputHelper::step_with_window_events`.
/// You should stick to one or the other.
#[derive(Clone)]
pub struct WinitInputHelper {
    current: Option<CurrentInput>,
    dropped_file: Option<PathBuf>,
    window_resized: Option<PhysicalSize<u32>>,
    window_size: Option<(u32, u32)>,
    scale_factor_changed: Option<f64>,
    scale_factor: Option<f64>,
    destroyed: bool,
    close_requested: bool,
}

impl Default for WinitInputHelper {
    fn default() -> Self {
        Self::new()
    }
}

impl WinitInputHelper {
    pub fn new() -> WinitInputHelper {
        WinitInputHelper {
            current: Some(CurrentInput::new()),
            dropped_file: None,
            window_resized: None,
            window_size: None,
            scale_factor_changed: None,
            scale_factor: None,
            destroyed: false,
            close_requested: false,
        }
    }

    /// Pass every winit event to this function and run your application logic when it returns true.
    ///
    /// The following winit events are handled:
    /// *   `Event::NewEvents` clears all internal state.
    /// *   `Event::MainEventsCleared` causes this function to return true, signifying a "step" has completed.
    /// *   `Event::WindowEvent` updates internal state, this will affect the result of accessor methods immediately.
    pub fn update<T>(&mut self, event: &Event<T>) -> bool {
        match &event {
            Event::NewEvents(_) => {
                self.step();
                false
            }
            Event::WindowEvent { event, .. } => {
                self.process_window_event(event);
                false
            }
            Event::MainEventsCleared => true,
            _ => false,
        }
    }

    /// Pass a slice containing every winit event that occured within the step to this function.
    /// Ensure this method is only called once per application main loop.
    /// Ensure every event since the last `WinitInputHelper::step_with_window_events` call is included in the `events` argument.
    ///
    /// `WinitInputHelper::Update` is easier to use.
    /// But this method is useful when your application logic steps dont line up with winit's event loop.
    /// e.g. you have a seperate thread for application logic using WinitInputHelper that constantly
    /// runs regardless of winit's event loop and you need to send events to it directly.
    pub fn step_with_window_events(&mut self, events: &[WindowEvent]) {
        self.step();
        for event in events {
            self.process_window_event(event);
        }
    }

    fn step(&mut self) {
        self.dropped_file = None;
        self.window_resized = None;
        self.scale_factor_changed = None;
        self.close_requested = false;
        if let Some(current) = &mut self.current {
            current.step();
        }
    }

    fn process_window_event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::CloseRequested => self.close_requested = true,
            WindowEvent::Destroyed => self.destroyed = true,
            WindowEvent::Focused(false) => self.current = None,
            WindowEvent::Focused(true) => {
                if self.current.is_none() {
                    self.current = Some(CurrentInput::new())
                }
            }
            WindowEvent::DroppedFile(path) => self.dropped_file = Some(path.clone()),
            WindowEvent::Resized(size) => {
                self.window_resized = Some(*size);
                self.window_size = Some((*size).into());
            }
            WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                self.scale_factor_changed = Some(*scale_factor);
                self.scale_factor = Some(*scale_factor);
            }
            _ => {}
        }
        if let Some(current) = &mut self.current {
            current.handle_event(event);
        }
    }

    /// Returns true when the specified keyboard key goes from "not pressed" to "pressed".
    /// Otherwise returns false.
    ///
    /// This is suitable for game controls.
    pub fn key_pressed(&self, check_key_code: VirtualKeyCode) -> bool {
        if let Some(current) = &self.current {
            for action in &current.key_actions {
                if let KeyAction::Pressed(key_code) = *action {
                    if key_code == check_key_code {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// Returns true when the specified keyboard key goes from "not pressed" to "pressed".
    /// Otherwise returns false.
    ///
    /// Will repeat key presses while held down according to the OS's key repeat configuration
    /// This is suitable for UI.
    pub fn key_pressed_os(&self, check_key_code: VirtualKeyCode) -> bool {
        if let Some(current) = &self.current {
            for action in &current.key_actions {
                if let KeyAction::PressedOs(key_code) = *action {
                    if key_code == check_key_code {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// Returns true when the specified keyboard key goes from "pressed" to "not pressed".
    /// Otherwise returns false.
    pub fn key_released(&self, check_key_code: VirtualKeyCode) -> bool {
        if let Some(current) = &self.current {
            for action in &current.key_actions {
                if let KeyAction::Released(key_code) = *action {
                    if key_code == check_key_code {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// Returns true while the specified keyboard key remains "pressed".
    /// Otherwise returns false.
    pub fn key_held(&self, key_code: VirtualKeyCode) -> bool {
        match &self.current {
            Some(current) => current.key_held[key_code as usize],
            None => false,
        }
    }

    /// Returns true while any shift key is held on the keyboard.
    /// Otherwise returns false.
    pub fn held_shift(&self) -> bool {
        self.key_held(VirtualKeyCode::LShift) || self.key_held(VirtualKeyCode::RShift)
    }

    /// Returns true while any control key is held on the keyboard.
    /// Otherwise returns false.
    pub fn held_control(&self) -> bool {
        self.key_held(VirtualKeyCode::LControl) || self.key_held(VirtualKeyCode::RControl)
    }

    /// Returns true while any alt key is held on the keyboard.
    /// Otherwise returns false.
    pub fn held_alt(&self) -> bool {
        self.key_held(VirtualKeyCode::LAlt) || self.key_held(VirtualKeyCode::RAlt)
    }

    /// Returns true when the specified mouse button goes from "not pressed" to "pressed".
    /// Otherwise returns false.
    ///
    /// Left   => 0
    /// Right  => 1
    /// Middle => 2
    /// Other  => 3..255
    pub fn mouse_pressed(&self, check_mouse_button: usize) -> bool {
        // TODO: Take MouseButton instead of usize
        if let Some(current) = &self.current {
            for action in &current.mouse_actions {
                if let MouseAction::Pressed(key_code) = *action {
                    if key_code == check_mouse_button {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// Returns true when the specified mouse button goes from "pressed" to "not pressed".
    /// Otherwise returns false.
    ///
    /// Left   => 0
    /// Right  => 1
    /// Middle => 2
    /// Other  => 3..255
    pub fn mouse_released(&self, check_mouse_button: usize) -> bool {
        // TODO: Take MouseButton instead of usize
        if let Some(current) = &self.current {
            for action in &current.mouse_actions {
                if let MouseAction::Released(key_code) = *action {
                    if key_code == check_mouse_button {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// Returns true while the specified mouse button remains "pressed".
    /// Otherwise returns false.
    ///
    /// Left   => 0
    /// Right  => 1
    /// Middle => 2
    /// Other  => 3..255
    pub fn mouse_held(&self, mouse_button: usize) -> bool {
        // TODO: Take MouseButton instead of usize
        match &self.current {
            Some(current) => current.mouse_held[mouse_button],
            None => false,
        }
    }

    /// Returns `(0.0, 0.0)` if the mouse is outside of the window.
    /// Otherwise returns the amount scrolled by the mouse during the last step.
    /// Returns (horizontally, vertically)
    pub fn scroll_diff(&self) -> (f32, f32) {
        match &self.current {
            Some(current) => (current.x_scroll_diff, current.y_scroll_diff),
            None => (0.0, 0.0),
        }
    }

    /// Returns `None` when the mouse is outside of the window.
    /// Otherwise returns the mouse coordinates in pixels.
    pub fn mouse(&self) -> Option<(f32, f32)> {
        match &self.current {
            Some(current) => current.mouse_point,
            None => None,
        }
    }

    /// Returns the change in mouse coordinates that occured during the last step.
    /// Returns `(0.0, 0.0)` if the window loses focus.
    pub fn mouse_diff(&self) -> (f32, f32) {
        if let Some(current_input) = &self.current {
            if let Some(cur) = current_input.mouse_point {
                if let Some(prev) = current_input.mouse_point_prev {
                    return (cur.0 - prev.0, cur.1 - prev.1);
                }
            }
        }
        (0.0, 0.0)
    }

    /// Returns the characters pressed during the last step.
    /// The earlier the character was pressed, the lower the index in the Vec.
    pub fn text(&self) -> Vec<TextChar> {
        match &self.current {
            Some(current) => current.text.clone(),
            None => vec![],
        }
    }

    /// Returns the path to a file that has been drag-and-dropped onto the window.
    pub fn dropped_file(&self) -> Option<PathBuf> {
        self.dropped_file.clone()
    }

    /// Returns the current window size if it was resized during the last step.
    /// Otherwise returns `None`.
    pub fn window_resized(&self) -> Option<PhysicalSize<u32>> {
        self.window_resized
    }

    /// Returns `None` when no `WindowEvent::Resized` have been received yet.
    /// After one has been received it returns the current resolution of the window.
    pub fn resolution(&self) -> Option<(u32, u32)> {
        self.window_size
    }

    /// Returns the current scale factor if it was changed during the last step.
    /// Otherwise returns `None`.
    pub fn scale_factor_changed(&self) -> Option<f64> {
        self.scale_factor_changed
    }

    /// Returns `None` when no `WindowEvent::ScaleFactorChanged` have been received yet.
    /// After one has been received it returns the current scale_factor of the window.
    pub fn scale_factor(&self) -> Option<f64> {
        self.scale_factor
    }

    /// Returns true if the window has been destroyed
    /// Otherwise returns false.
    /// Once this method has returned true once all following calls to this method will also return true.
    pub fn destroyed(&self) -> bool {
        self.destroyed
    }

    /// Returns true if the OS has requested the application to close during this step.
    /// Otherwise returns false.
    pub fn close_requested(&self) -> bool {
        self.close_requested
    }

    /// Deprecated
    #[deprecated(note = "Instead use `input.close_requested() || input.destroyed()`")]
    pub fn quit(&self) -> bool {
        self.close_requested || self.destroyed
    }
}
