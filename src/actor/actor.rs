use std::time::Duration;

use crossterm::event::KeyCode;

use crate::{actor::action::Action, engine::{controls::Controls, player::BIG_BLIND, timer::Timer}};

pub trait PokerActor {
    fn start_turn(&mut self);
    fn turn_started(&self) -> bool;
    fn done(&mut self, forced: bool, game: &mut Controls, last_raise: usize, pot: usize) -> bool;
    fn get_action(&mut self) -> Action;
    fn end_turn(&mut self);
}

pub struct SimpleActor {
    started: bool,
    timer: Timer
}

impl SimpleActor {
    pub fn new() -> SimpleActor {
        Self { started: false, timer: Timer::new(Duration::from_millis(500)) }
    }
}

impl PokerActor for SimpleActor {
    fn start_turn(&mut self) {
        self.started = true;
        self.timer.start();
    }

    fn turn_started(&self) -> bool {
        self.started
    }

    fn done(&mut self, _forced: bool, _controls: &mut Controls, _last_raise: usize, _pot: usize) -> bool {
        self.timer.done()
    }

    fn get_action(&mut self) -> Action {
        return Action::Call;
    }

    fn end_turn(&mut self) {
        self.started = false;
    }
}

pub struct HumanActor {
    started: bool,
    selected_action: Option<Action>
}

impl HumanActor {
    pub fn new() -> HumanActor {
        Self { started: false, selected_action: None }
    }
}

impl PokerActor for HumanActor {
    fn start_turn(&mut self) {
        self.started = true;
    }

    fn turn_started(&self) -> bool {
        self.started
    }

    fn done(&mut self, forced: bool, controls: &mut Controls, last_raise: usize, pot: usize) -> bool {
        if forced {
            return true;
        }

        let min_raise = BIG_BLIND.max(last_raise);

        if controls.is_pressed(KeyCode::Char('f')) {
            self.selected_action = Some(Action::Fold);
        
        } else if controls.is_pressed(KeyCode::Char('c')) {
            self.selected_action = Some(Action::Call);
        
        } else if controls.is_pressed(KeyCode::Char('r')) {
            self.selected_action = Some(Action::Raise(min_raise));
        
        } else if controls.is_pressed(KeyCode::Char('p')) {
            if controls.is_pressed(KeyCode::Char('d')) {
                self.selected_action = Some(Action::Raise(pot));
            
            } else if controls.is_pressed(KeyCode::Char('t')) {
                self.selected_action = Some(Action::Raise(pot * 2));
            }

        } else {
            if controls.is_pressed(KeyCode::Char('d')) {
                self.selected_action = Some(Action::Raise(min_raise * 2));
            
            } else if controls.is_pressed(KeyCode::Char('t')) {
                self.selected_action = Some(Action::Raise(min_raise * 3));
            }
        }

        self.selected_action.is_some()
    }

    fn get_action(&mut self) -> Action {
        self.selected_action.as_ref().cloned().unwrap()
    }

    fn end_turn(&mut self) {
        self.started = false;
        self.selected_action = None;
    }
}
