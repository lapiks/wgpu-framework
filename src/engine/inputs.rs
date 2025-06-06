use std::collections::HashMap;

use glam::Vec2;
use winit::{event::MouseButton, keyboard::{KeyCode, ModifiersState}};

#[derive(Clone, Copy, Hash, Eq, PartialEq, Default)]
pub struct Modifiers {
    pub alt: bool,
    pub ctrl: bool,
    pub shift: bool,
}

/// Gather and store user inputs
pub struct Inputs {
    mouse: HashMap<MouseButton, bool>,
    keys: HashMap<KeyCode, bool>,
    modifiers: Modifiers,
    mouse_delta: Vec2,
    mouse_wheel_delta: f32,
}

impl Default for Inputs {
    fn default() -> Self {
        Self {
            mouse: HashMap::default(),
            keys: HashMap::default(),
            modifiers: Modifiers::default(),
            mouse_delta: Vec2::ZERO,
            mouse_wheel_delta: 0.0,
        }
    }
}

impl Inputs {
    pub fn new() -> Self {
        Default::default()
    } 

    pub fn reset(&mut self) {
        self.mouse_delta = Vec2::ZERO;
        self.mouse_wheel_delta = 0.0;
    }

    pub fn on_mouse_move(&mut self, position: Vec2) {
        self.mouse_delta += position;
    }

    pub fn on_mouse_wheel(&mut self, delta: f32) {
        self.mouse_wheel_delta += delta;
    }

    pub fn on_mouse_button_down(&mut self, button: MouseButton) {
        self.mouse.insert(button, true);
    }

    pub fn on_mouse_button_up(&mut self, button: MouseButton) {
        self.mouse.insert(button, false);
    }

    pub fn on_key_down(&mut self, keycode: KeyCode) {
        self.keys.insert(keycode, true);
    }

    pub fn on_key_up(&mut self, keycode: KeyCode) {
        self.keys.insert(keycode, false);
    }

    pub fn set_modifiers(&mut self, mods: ModifiersState) {
        self.modifiers.alt = mods.alt_key();
        self.modifiers.ctrl = mods.control_key();
        self.modifiers.shift = mods.shift_key();
    }

    pub fn get_key_down(&self, keycode: KeyCode) -> bool {
        *self.keys.get(&keycode).unwrap_or(&false)
    }

    pub fn get_modifiers(&self) -> &Modifiers {
        &self.modifiers
    }

    pub fn get_button_down(&self, button: MouseButton) -> bool {
        *self.mouse.get(&button).unwrap_or(&false)
    }

    pub fn get_mouse_delta(&self) -> Vec2 {
        self.mouse_delta
    }

    pub fn get_mouse_wheel_delta(&self) -> f32 {
        self.mouse_wheel_delta
    }
}