use std::{collections::HashMap, time::Duration};

use rand::{rng, seq::IndexedRandom, Rng};

use crate::{actor::{action::Action, actor::{ActorInfo, PokerActor}}, engine::{controls::Controls, player::BIG_BLIND, timer::Timer}, poker::ai::monte_carlo_likeliness_to_win};

pub struct AdHocActor {
    started: bool,
    selected_action: Option<Action>,
    timer: Timer
}

impl AdHocActor {
    pub fn new() -> AdHocActor {
        Self { started: false, selected_action: None, timer: Timer::new(Duration::from_millis(500)) }
    }
}

fn select_weighted(options: &[Action], weights: &[f32]) -> Action {
    let mut rng = rng();
    let weights = options.iter().zip(weights).collect::<HashMap<_, _>>();

    options.choose_weighted(&mut rng, |i| weights[i]).unwrap().clone()
}

impl PokerActor for AdHocActor {
    fn start_turn(&mut self) {
        self.started = true;
        self.timer.start();
    }

    fn turn_started(&self) -> bool {
        self.started
    }

    fn done(&mut self, _forced: bool, _controls: &mut Controls, info: ActorInfo) -> bool {
        if self.selected_action.is_none() {
            let mut rng = rng();

            // Constants
            const P_EPSILON: f32 = 0.02;
            const CALL_DEFEND_FRAC: f32 = 0.075;
            const GOOD_ADVANTAGE: f32 = 1.5;
            const GREAT_ADVANTAGE: f32 = 2.0;
            const FANTASTIC_ADVANTAGE: f32 = 3.0;

            // Estimate winning probability as is
            let num_players = info.players.len();
            let equity = monte_carlo_likeliness_to_win(&info.hand, &info.community, num_players, 100000);
            
            // Calculate call metrics
            let pot = info.players.iter().map(|p| p.1.1).sum::<usize>();
            let call_amount = info.current_bet - info.players[&info.player].1;
            let break_even = call_amount as f32 / (call_amount + pot) as f32;
            let call_frac = call_amount as f32 / info.players[&info.player].0 as f32;

            // Calculate mean MDF
            let min_bet = info.players.iter().map(|p| p.1.1).min().unwrap();
            let mdf = info.players.iter()
                .map(|i| 1.0 - (i.1.1 - min_bet) as f32 / pot as f32)
                .sum::<f32>() / num_players as f32;

            // If the call is worth it
            if equity > break_even + P_EPSILON {
                let neutral = 1.0 / info.players.len() as f32;
                let advantage = (equity / (1.0 - equity)) / (neutral / (1.0 - neutral)); // Odds ratio

                // Action set
                let max_raise = info.players[&info.player].0;
                let min_raise = BIG_BLIND.max(info.last_raise).min(max_raise);
                
                let raise_small = Action::Raise(min_raise);
                let raise_double = Action::Raise((min_raise * 2).min(max_raise));
                let raise_triple = Action::Raise((min_raise * 3).min(max_raise));
                let raise_pot = Action::Raise(info.current_bet.min(max_raise));
                let raise_double_pot = Action::Raise((info.current_bet * 2).min(max_raise));

                if advantage > FANTASTIC_ADVANTAGE {
                    self.selected_action = Some(select_weighted(
                        &[raise_triple, raise_pot, raise_double_pot], 
                        &[1.0, advantage, advantage / 2.0]
                    ))

                } else if advantage > GREAT_ADVANTAGE {
                    self.selected_action = Some(select_weighted(
                        &[raise_double, raise_triple, raise_pot], 
                        &[1.0, advantage, advantage / 2.0]
                    ))

                } else if advantage > GOOD_ADVANTAGE {
                    self.selected_action = Some(select_weighted(
                        &[raise_small, raise_double], 
                        &[1.0, advantage]
                    ))

                } else {
                    self.selected_action = Some(Action::Call)
                }

            } else if call_frac < CALL_DEFEND_FRAC || rng.random_bool(mdf.into()) {
                self.selected_action = Some(Action::Call)

            } else {
                self.selected_action = Some(Action::Fold)
            }
        }

        self.timer.done()
    }

    fn get_action(&mut self) -> Action {
        self.selected_action.as_ref().cloned().unwrap()
    }

    fn end_turn(&mut self) {
        self.started = false;
        self.selected_action = None;
    }
}