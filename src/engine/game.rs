use std::{collections::HashSet, time::Duration};

use crossterm::{event::{self, Event, KeyCode, MouseEventKind}, style::Color, terminal::{disable_raw_mode, enable_raw_mode}};

use crate::{actor::{action::Action, actor::ActorInfo, adhoc::AdHocActor, human::HumanActor}, engine::{console::{clear, clear_section, disable_mouse_capture, enable_mouse_capture, enter_alternate_screen, hide_cursor, leave_alternate_screen, move_cursor, resize, set_color, show_cursor, write_str}, controls::Controls, player::{Player, BIG_BLIND, SMALL_BLIND}, state::GameState}, poker::{card::{Card, BAIZE, CREAM, DBLUE}, deck::Deck, play::{analyze_play, Play}}};

pub struct Game {
    pub controls: Controls,
    pub deck: Deck,
    pub state: GameState,
    pub players: Vec<Player>,
    pub board: Vec<Card>,
    pub dealer: usize,
    pub current_bet: usize,
    pub last_raise: usize
}

impl Game {
    pub fn new() -> Self {
        let players = vec!(
            Player::new("Player 1".into(), 100, Box::new(HumanActor::new())),
            Player::new("Player 2".into(), 100, Box::new(AdHocActor::new())),
            Player::new("Player 3".into(), 100, Box::new(AdHocActor::new())),
            Player::new("Player 4".into(), 100, Box::new(AdHocActor::new())),
        );

        Game { 
            controls: Controls::new(),
            deck: Deck::new(),
            state: GameState::Dealing,
            players,
            board: vec!(),
            dealer: 0,
            current_bet: 0,
            last_raise: 0
        }
    }

    pub fn startup(&mut self) {
        resize(41, 125);

        enable_raw_mode().expect("Unable to start raw mode");
        enter_alternate_screen();

        enable_mouse_capture();
        
        clear();
        hide_cursor();

        self.draw_ui();
    }

    pub fn finalize(&self) {
        move_cursor(0, 0);
        show_cursor();

        disable_mouse_capture();

        leave_alternate_screen();
        disable_raw_mode().expect("Unable to disable raw mode");
    }

    pub fn poll_inputs(&mut self) -> bool {
        while event::poll(Duration::ZERO).unwrap() {
            match event::read().unwrap() {
                Event::Key(key) => {
                    match key.kind {
                        event::KeyEventKind::Press => self.controls.press(key.code),
                        event::KeyEventKind::Release => self.controls.release(key.code),
                        _ => {}
                    }    
                }

                Event::Mouse(me) => {
                    self.controls.set_mouse(me.row as usize, me.column as usize);

                    match me.kind {
                        MouseEventKind::Down(b) => self.controls.mouse.press(b),
                        MouseEventKind::Up(b) => self.controls.mouse.release(b),
                        MouseEventKind::ScrollUp => self.controls.mouse.wheel_up(),
                        MouseEventKind::ScrollDown => self.controls.mouse.wheel_down(),
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        false
    }

    pub fn draw_ui(&self) {

    }

    pub fn draw_single_player_play(&self, col: usize, row: usize, play: String) {
        let name = format!(" {} ", play);
        let h = name.len() / 2;

        set_color(DBLUE, Color::White);
        move_cursor(row, col - h);
        write_str(&name);

        set_color(BAIZE, DBLUE);
        move_cursor(row - 1, col - h);
        write_str(&"▄".repeat(name.len()));
        move_cursor(row + 1, col - h);
        write_str(&"▀".repeat(name.len()));
    }

    pub fn draw_single_player_play_left(&self, col: usize, row: usize, play: String) {
        let name = format!(" {} ", play);

        set_color(DBLUE, Color::White);
        move_cursor(row, col);
        write_str(&name);

        set_color(BAIZE, DBLUE);
        move_cursor(row - 1, col);
        write_str(&"▄".repeat(name.len()));
        move_cursor(row + 1, col);
        write_str(&"▀".repeat(name.len()));
    }

    pub fn draw_single_player_play_right(&self, col: usize, row: usize, play: String) {
        let name = format!(" {} ", play);

        set_color(DBLUE, Color::White);
        move_cursor(row, col - name.len());
        write_str(&name);

        set_color(BAIZE, DBLUE);
        move_cursor(row - 1, col - name.len());
        write_str(&"▄".repeat(name.len()));
        move_cursor(row + 1, col - name.len());
        write_str(&"▀".repeat(name.len()));
    }

    pub fn draw_player_plays(&self, plays: &Vec<Play>, winners: &HashSet<usize>, valid_players: &HashSet<usize>) {
        let play_str = |i: usize| {
            if winners.contains(&i) {
                format!(">>> {} <<<", plays[i].name())

            } else {
                plays[i].name()
            }
        }; 

        if valid_players.contains(&0) {
            self.draw_single_player_play(62, 27, play_str(0));
        }

        if valid_players.contains(&1) {
            self.draw_single_player_play_left(5, 9, play_str(1));
        }

        if valid_players.contains(&2) {
            self.draw_single_player_play(62, 13, play_str(2));
        }

        if valid_players.contains(&3) {
            self.draw_single_player_play_right(120, 31, play_str(3));
        }
    }

    pub fn draw_single_player_chips(&self, col: usize, row: usize, player: &Player) {
        set_color(CREAM, Color::Black);
        clear_section(row, col, row, col + 10);

        move_cursor(row, col + 1);
        write_str(&format!("Chips {:>3}", player.money));

        set_color(BAIZE, CREAM);
        move_cursor(row - 1, col);
        write_str(&"▄".repeat(11));
        move_cursor(row + 1, col);
        write_str(&"▀".repeat(11));
    }

    pub fn draw_player_chips(&self) {
        self.draw_single_player_chips(79, 37, &self.players[0]);

        if !self.players[1].lost() {
            self.draw_single_player_chips(5, 31, &self.players[1]);
        }

        if !self.players[2].lost() {
            self.draw_single_player_chips(35, 3, &self.players[2]);
        }

        if !self.players[3].lost() {
            self.draw_single_player_chips(109, 9, &self.players[3]);
        }
    }

    pub fn draw_single_player_bet(&self, col: usize, row: usize, player: &Player) {
        set_color(CREAM, Color::Black);
        clear_section(row, col, row, col + 8);

        move_cursor(row, col + 1);
        write_str(&format!("Bet {:>3}", player.bet));

        set_color(BAIZE, CREAM);
        move_cursor(row - 1, col);
        write_str(&"▄".repeat(9));
        move_cursor(row + 1, col);
        write_str(&"▀".repeat(9));
    }

    pub fn draw_player_bets(&self) {
        self.draw_single_player_bet(79, 34, &self.players[0]);

        if !self.players[1].lost() {
            self.draw_single_player_bet(5, 34, &self.players[1]);
        }

        if !self.players[2].lost() {
            self.draw_single_player_bet(35, 6, &self.players[2]);
        }

        if !self.players[3].lost() {
            self.draw_single_player_bet(109, 6, &self.players[3]);
        }
    }

    pub fn draw_dealer_chip_at(&self, row: usize, col: usize) {
        set_color(DBLUE, Color::White);
        move_cursor(row, col);
        write_str(" D ");

        set_color(DBLUE, BAIZE);
        move_cursor(row - 1, col);
        write_str(&"▀".repeat(3));
        move_cursor(row + 1, col);
        write_str(&"▄".repeat(3));
    }

    pub fn draw_turn_chip_at(&self, row: usize, col: usize) {
        set_color(Color::Cyan, Color::Black);
        move_cursor(row, col);
        write_str(" T ");

        set_color(Color::Cyan, BAIZE);
        move_cursor(row - 1, col);
        write_str(&"▀".repeat(3));
        move_cursor(row + 1, col);
        write_str(&"▄".repeat(3));
    }

    pub fn draw_dealer_chip(&self) {
        set_color(BAIZE, Color::Black);
        clear_section(30, 43, 32, 46);
        clear_section(8, 4, 10, 7);
        clear_section(2, 77, 4, 80);
        clear_section(30, 108, 32, 111);

        match self.dealer {
            0 => self.draw_dealer_chip_at(31, 44),
            1 => self.draw_dealer_chip_at(9, 5),
            2 => self.draw_dealer_chip_at(3, 78),
            3 => self.draw_dealer_chip_at(31, 109),
            _ => unreachable!()
        }
    }

    pub fn draw_turn_chip(&self, turn: usize) {
        set_color(BAIZE, Color::Black);
        clear_section(33, 43, 35, 46);
        clear_section(8, 9, 10, 12);
        clear_section(2, 82, 4, 85);
        clear_section(30, 113, 32, 116);

        match turn {
            0 => self.draw_turn_chip_at(34, 44),
            1 => self.draw_turn_chip_at(9, 10),
            2 => self.draw_turn_chip_at(3, 83),
            3 => self.draw_turn_chip_at(31, 114),
            _ => unreachable!()
        }
    }

    pub fn bet(&mut self, turn: usize, chips: usize) {
        self.players[turn].bet_chips(chips);

        if self.current_bet < self.players[turn].bet {
            self.current_bet = self.players[turn].bet;
        }

        self.draw_player_chips();
        self.draw_player_bets();
    }

    pub fn print_msg(&mut self, msg: String) {
        set_color(Color::Black, Color::White);
        move_cursor(0, 0);
        write_str(&msg);
    }

    pub fn draw_info_at(&self, row: usize, col: usize, lines: Vec<String>) {
        let width = lines.iter().map(String::len).max().unwrap();

        set_color(DBLUE, Color::White);

        for (i, line) in lines.iter().enumerate() {
            move_cursor(row + i, col);
            write_str(&format!(" {}{} ", line, " ".repeat(width - line.len())));
        }

        set_color(BAIZE, DBLUE);
        move_cursor(row - 1, col);
        write_str(&"▄".repeat(width + 2));
        move_cursor(row + lines.len(), col);
        write_str(&"▀".repeat(width + 2));
    }

    pub fn clear_info(&self) {
        set_color(BAIZE, DBLUE);
        clear_section(30, 23, 41, 40);
    }

    pub fn next_turn(&mut self, turn: usize) -> usize {
        let mut res = (turn + 1) % 4;

        while self.players[res].lost() {
            res = (res + 1) % 4;
        }

        res
    }

    pub fn perform_action(&mut self, action: Action, turn: usize) {
        match action {
            Action::Fold => self.players[turn].fold(),
            
            Action::Call => {
                let call_amount = self.current_bet - self.players[turn].bet;
                let player_money = self.players[turn].money;

                self.bet(turn, player_money.min(call_amount))
            },

            Action::Raise(c) => {
                let call_amount = self.current_bet - self.players[turn].bet;
                let player_money = self.players[turn].money;

                self.last_raise = c;
                self.bet(turn, player_money.min(call_amount + c));
            },
        }
    }

    pub fn solve_pots(&mut self, plays: &Vec<Play>) -> HashSet<usize> {
        // Players that won something
        let mut winners = HashSet::new();

        // Initial contributions
        let mut contributions = self.players.iter().map(|p| p.bet).collect::<Vec<_>>();
        let mut total = contributions.iter().sum::<usize>();
        let valid_plays = plays.iter()
            .enumerate()
            .filter(|(i, _)| !self.players[*i].folded)
            .filter(|(i, _)| !self.players[*i].lost())
            .collect::<Vec<_>>();

        // Reset player bets
        self.players.iter_mut().for_each(Player::lose_bet);

        // Pot winning algorithm
        while total > 0 {
            // Calculate layers
            let mut layers = contributions.clone();
            layers.sort();
            layers.dedup();

            // Compute incremental tiers
            for i in (1..layers.len()).rev() {
                layers[i] -= layers[i - 1]; 
            }

            // Solve layers from lowest amount to highest
            for layer in layers {
                // Get players of the layer (non-folded and active)
                let mut layer_players = valid_plays.iter()
                    .filter(|(p, _)| contributions[*p] >= layer)
                    .collect::<Vec<_>>();

                layer_players.sort_by(|a, b| a.1.cmp(&b.1).then_with(|| a.0.cmp(&b.0)));

                // Get tied best players of the layer
                let (_, best_play) = layer_players.last().unwrap();
                let tied_best_players = layer_players.iter()
                    .filter(|(_, p)| p == best_play)
                    .map(|i| i.0)
                    .collect::<Vec<_>>();

                // Subtract layer value from contributions to get remaining contributions
                let mut layer_amount = 0;

                for c in contributions.iter_mut() {
                    let amount = layer.min(*c);
                    *c -= amount;
                    layer_amount += amount;
                }

                total -= layer_amount;

                // Distribute the amount got from this layer
                let base_amount = layer_amount / tied_best_players.len();
                let mut remainder = layer_amount % tied_best_players.len();

                for p in tied_best_players.iter() {
                    let won_amount = base_amount + 1.min(remainder);
                    self.players[*p].win(won_amount);

                    if remainder > 0 {
                        remainder -= 1; // Remainder is distributed in seating order
                    }

                    if won_amount > 0 {
                        winners.insert(*p);
                    }
                }
            }
        }

        winners
    }

    pub fn update(&mut self) -> bool {
        match self.state {
            GameState::MainMenu => todo!(),
            
            GameState::Dealing => {
                // Prepare players
                self.players.iter_mut().for_each(Player::unfold);

                // Prepare cards
                self.deck.shuffle();

                for player in &mut self.players {
                    if !player.lost() {
                        for _ in 0..2 {
                            player.give_card(self.deck.pop().expect("No more cards"));
                        }
                    }
                }

                for _ in 0..5 {
                    self.board.push(self.deck.pop().expect("No more cards"));
                }

                self.state = GameState::Round(0, self.next_turn(self.dealer), false, false, false);

                // Draw green baize
                set_color(BAIZE, Color::Black);
                clear_section(0, 0, 40, 125);

                self.draw_player_chips();
                self.draw_player_bets();
                self.draw_dealer_chip();
            },

            GameState::Round(num_flipped, turn, sb, bb, mut initial) => {                
                if !self.players[turn].actor.turn_started() {
                    self.players[turn].actor.start_turn();
                }

                // Information for the actors to decide
                let actor_info = ActorInfo {
                    player: turn,
                    last_raise: self.last_raise,
                    current_bet: self.current_bet,
                    hand: self.players[turn].hand.clone(),
                    community: self.board[..num_flipped].to_vec(),
                    players: self.players.iter().enumerate()
                        .filter(|p| !p.1.lost())
                        .map(|(i, p)| (i, (p.money, p.bet, p.folded)))
                        .collect(),
                };

                if !sb && !bb { // Small blind
                    if self.players[turn].actor.done(true, &mut self.controls, actor_info) {
                        self.perform_action(Action::Raise(SMALL_BLIND), turn);
                        self.players[turn].actor.end_turn();
    
                        self.state = GameState::Round(num_flipped, self.next_turn(turn), true, bb, true);
                    }

                } else if sb && !bb { // Big blind
                    if self.players[turn].actor.done(true, &mut self.controls, actor_info) {
                        self.perform_action(Action::Raise(BIG_BLIND - SMALL_BLIND), turn);
                        self.players[turn].actor.end_turn();
    
                        self.state = GameState::Round(num_flipped, self.next_turn(turn), true, true, true);
                        self.players[0].hand.iter_mut().for_each(Card::reset_draw_cache);
                        self.last_raise = BIG_BLIND;
                    }                    

                } else {
                    // Normal turn
                    let only_one_left = self.players.iter().map(|i| !i.folded && !i.lost()).count() == 1;

                    if !only_one_left && !self.players[turn].folded && !self.players[turn].is_all_in() && (!initial || self.players[turn].bet < self.current_bet) {
                        if turn == 0 {
                            let player_money = self.players[turn].money;
                            let call_amount = self.current_bet - self.players[turn].bet;
                            let max_raise = player_money - call_amount.min(player_money);
                            let min_raise = BIG_BLIND.max(self.last_raise).min(max_raise);

                            let raise_bet = if initial { "Raise" } else { "Bet" };
                            let raise_all_in = |c: usize| {
                                if (c + call_amount) >= player_money {
                                    "All-in".into()
                                } else {
                                    format!("{raise_bet} {c}")
                                }
                            };

                            self.draw_info_at(
                                31, 23, 
                                vec!(
                                    if self.players[turn].bet == self.current_bet {
                                        format!("[C]   Check")
                                    
                                    } else if call_amount <= player_money {
                                        format!("[C]   Call {}", self.current_bet)

                                    } else {
                                        format!("[C]   All-in")
                                    },
                                    format!("[R]   {}", raise_all_in(min_raise)),
                                    format!("[D]   {}", raise_all_in((min_raise * 2).min(max_raise))),
                                    format!("[T]   {}", raise_all_in((min_raise * 3).min(max_raise))),
                                    format!("[B+D] {}", raise_all_in(self.current_bet.min(max_raise))),
                                    format!("[B+T] {}", raise_all_in((self.current_bet * 2).min(max_raise))),
                                    format!("[F]   Fold")
                                )
                            );
                        }

                        if self.players[turn].actor.done(false, &mut self.controls, actor_info) {
                            let action = self.players[turn].actor.get_action();

                            if matches!(action, Action::Raise(_)) {
                                initial = true;
                            }

                            self.perform_action(action, turn);
                            
                            self.players[turn].actor.end_turn();

                            if turn == 0 {
                                self.clear_info();
                            }
    
                        } else {
                            return false; // Wait for the actor to be done
                        }

                    } else {
                        if turn == 0 {
                            self.clear_info();
                        }

                        // End turn inmediately if no action is possible
                        self.players[turn].actor.end_turn();
                    }

                    let balanced_bet = self.players.iter()
                        .filter(|p| !p.folded)
                        .filter(|p| !p.lost())
                        .all(|i| i.bet == self.current_bet);

                    // Pass stage
                    if turn == self.dealer && balanced_bet {
                        if num_flipped < 5 {
                            // Pre-flop
                            if num_flipped == 0 {
                                self.board[0].reset_draw_cache();
                                self.board[1].reset_draw_cache();
                                self.board[2].reset_draw_cache();
                                self.state = GameState::Round(3, self.next_turn(turn), true, true, false);

                            } else {
                                self.board[num_flipped].reset_draw_cache();
                                self.state = GameState::Round(num_flipped + 1, self.next_turn(self.dealer), true, true, false);
                            }
                            
                            self.last_raise = 0;

                        } else {
                            // Calculate winner and draw plays
                            let plays = self.players.iter()
                                .map(|p| analyze_play(&p.hand, &self.board))
                                .collect();

                            let valid_players = self.players.iter()
                                .enumerate()
                                .filter(|p| !p.1.folded && !p.1.lost())
                                .map(|p| p.0)
                                .collect();

                            let winners = self.solve_pots(&plays);

                            self.draw_player_plays(&plays, &winners, &valid_players);

                            // Reset draw cache and proceed
                            self.players.iter_mut().flat_map(|p| &mut p.hand).for_each(Card::reset_draw_cache);
                            self.state = GameState::Resolving;
                        }

                    } else {
                        self.state = GameState::Round(num_flipped, self.next_turn(turn), sb, bb, initial);
                    }
                }
            },

            GameState::Collecting => {
                for p in &mut self.players {
                    while !p.hand.is_empty() {
                        self.deck.push(p.hand.pop().unwrap());
                    }
                }

                while !self.board.is_empty() {
                    self.deck.push(self.board.pop().unwrap());
                }

                self.deck.reset_draw_cache();
                self.deck.shuffle();

                self.state = GameState::Dealing;
            },

            GameState::Resolving => {
                if self.controls.is_pressed(KeyCode::Enter) && !self.controls.is_locked(KeyCode::Enter) {
                    self.controls.lock(KeyCode::Enter, Duration::from_millis(500));

                    self.state = GameState::Collecting;
                    self.dealer = self.next_turn(self.dealer);
                    self.current_bet = 0;
                }
            },
        };

        return false;
    }

    pub fn render(&mut self) {
        match self.state {
            GameState::MainMenu => todo!(),

            GameState::Dealing |
            GameState::Collecting => {},
            
            GameState::Round(num_flipped, turn, sb, bb, _) => {
                self.draw_turn_chip(turn);

                // Center cards
                for (i, card) in self.board.iter_mut().enumerate() {
                    card.draw(27 + i * 15, 16, i >= num_flipped);
                }

                // Players
                for (i, card) in self.players[3].hand.iter_mut().enumerate() {
                    card.draw(109, 11 + i * 10, true);
                }

                for (i, card) in self.players[2].hand.iter_mut().enumerate() {
                    card.draw(25 + 24 + i * 16, 2, true);
                }

                for (i, card) in self.players[1].hand.iter_mut().enumerate() {
                    card.draw(5, 11 + i * 10, true);
                }

                for (i, card) in self.players[0].hand.iter_mut().enumerate() {
                    card.draw(25 + 24 + i * 16, 30, !sb || !bb);
                }
            },

            GameState::Resolving => {
                for (i, card) in self.players[3].hand.iter_mut().enumerate() {
                    card.draw(109, 11 + i * 10, false);
                }

                for (i, card) in self.players[2].hand.iter_mut().enumerate() {
                    card.draw(25 + 24 + i * 16, 2, false);
                }

                for (i, card) in self.players[1].hand.iter_mut().enumerate() {
                    card.draw(5, 11 + i * 10, false);
                }

                for (i, card) in self.players[0].hand.iter_mut().enumerate() {
                    card.draw(25 + 24 + i * 16, 30, false);
                }
            },
        }
    }
}