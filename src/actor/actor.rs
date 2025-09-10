use std::{collections::HashMap, time::Duration};

use crate::{actor::action::Action, engine::{controls::Controls, timer::Timer}, poker::card::Card};

pub struct ActorInfo {
    pub player: usize,
    pub last_raise: usize, 
    pub current_bet: usize,
    pub hand: Vec<Card>,
    pub community: Vec<Card>,
    pub players: HashMap<usize, (usize, usize, bool)>, // Idx -> (chips, bet, folded)
}

pub trait PokerActor {
    fn start_turn(&mut self);
    fn turn_started(&self) -> bool;
    fn done(&mut self, forced: bool, controls: &mut Controls, info: ActorInfo) -> bool;
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

    fn done(&mut self, _forced: bool, _controls: &mut Controls, _info: ActorInfo) -> bool {
        self.timer.done()
    }

    fn get_action(&mut self) -> Action {
        return Action::Call;
    }

    fn end_turn(&mut self) {
        self.started = false;
    }
}
