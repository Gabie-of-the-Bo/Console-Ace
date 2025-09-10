use crossterm::event::KeyCode;

use crate::{actor::{action::Action, actor::{ActorInfo, PokerActor}}, engine::{controls::Controls, player::BIG_BLIND}};

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

    fn done(&mut self, forced: bool, controls: &mut Controls, info: ActorInfo) -> bool {
        if forced {
            return true;
        }
        
        let player_money = info.players[&info.player].0;
        let call_amount = info.current_bet - info.players[&info.player].1;
        let max_raise = player_money - call_amount.min(player_money);
        let min_raise = BIG_BLIND.max(info.last_raise).min(max_raise);

        if controls.is_pressed(KeyCode::Char('f')) {
            self.selected_action = Some(Action::Fold);
        
        } else if controls.is_pressed(KeyCode::Char('c')) {
            self.selected_action = Some(Action::Call);
        
        } else if controls.is_pressed(KeyCode::Char('r')) {
            self.selected_action = Some(Action::Raise(min_raise));
        
        } else if controls.is_pressed(KeyCode::Char('b')) {
            if controls.is_pressed(KeyCode::Char('d')) {
                self.selected_action = Some(Action::Raise(info.current_bet.min(max_raise)));
            
            } else if controls.is_pressed(KeyCode::Char('t')) {
                self.selected_action = Some(Action::Raise((info.current_bet * 2).min(max_raise)));
            }

        } else {
            if controls.is_pressed(KeyCode::Char('d')) {
                self.selected_action = Some(Action::Raise((min_raise * 2).min(max_raise)));
            
            } else if controls.is_pressed(KeyCode::Char('t')) {
                self.selected_action = Some(Action::Raise((min_raise * 3).min(max_raise)));
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