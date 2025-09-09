use crossterm::event::KeyCode;

use crate::{actor::{action::Action, actor::PokerActor}, engine::{controls::Controls, player::BIG_BLIND}};

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