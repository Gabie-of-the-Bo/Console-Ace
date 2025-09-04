use std::{collections::HashMap, time::Duration};

use crossterm::event::{KeyCode, MouseButton};

use super::{timer::Timer};

pub struct Mouse {
    buttons: HashMap<MouseButton, bool>,
    wheel_up: Timer,
    wheel_down: Timer,
    pub position: (usize, usize),
}

impl Mouse {
    pub fn press(&mut self, key: MouseButton) {
        *self.buttons.entry(key).or_default() = true;
    }

    pub fn wheel_up(&mut self) {
        self.wheel_up.start();
    }

    pub fn wheel_down(&mut self) {
        self.wheel_down.start();
    }

    pub fn release(&mut self, key: MouseButton) {
        *self.buttons.entry(key).or_default() = false;
    }

    pub fn is_pressed(&mut self, key: MouseButton) -> bool {
        *self.buttons.entry(key).or_insert(false)
    }

    pub fn is_wheel_up(&mut self) -> bool {
        let res = !self.wheel_up.done();

        if res {
            self.wheel_up.exhaust();
        }

        res
    }

    pub fn is_wheel_down(&mut self) -> bool {
        let res = !self.wheel_down.done();

        if res {
            self.wheel_down.exhaust();
        }

        res
    }
}

pub struct Controls {
    keys: HashMap<KeyCode, bool>,
    locks: HashMap<KeyCode, Timer>,
    pub mouse: Mouse
}

impl Controls {
    pub fn new() -> Self {
        Controls { 
            keys: HashMap::new(), 
            locks: HashMap::new(),
            mouse: Mouse { 
                buttons: HashMap::new(), 
                wheel_up: Timer::new(Duration::from_millis(10)), 
                wheel_down: Timer::new(Duration::from_millis(10)), 
                position: (0, 0), 
            }
        }
    }

    pub fn release_all(&mut self) {
        self.keys.clear();
    }

    pub fn press(&mut self, key: KeyCode) {
        if self.is_locked(key) {
            return;
        }

        *self.keys.entry(key).or_default() = true;
    }

    pub fn release(&mut self, key: KeyCode) {
        if self.is_locked(key) {
            return;
        }
        
        *self.keys.entry(key).or_default() = false;
    }

    pub fn lock(&mut self, key: KeyCode, duration: Duration) {
        self.locks.insert(key, Timer::new_started(duration));
    }

    pub fn is_locked(&mut self, key: KeyCode) -> bool {
        if let Some(t) = self.locks.get(&key) {
            return !t.done();
        }
        
        false
    }

    pub fn set_mouse(&mut self, row: usize, col: usize) {
        self.mouse.position = (row, col);
    }

    pub fn is_pressed(&mut self, key: KeyCode) -> bool {
        *self.keys.entry(key).or_insert(false)
    }
}