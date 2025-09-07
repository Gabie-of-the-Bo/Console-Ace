use std::time::Duration;

use crossterm::{event::{self, Event, KeyCode, MouseEventKind}, style::Color, terminal::{disable_raw_mode, enable_raw_mode}};

use crate::{engine::{console::{clear, clear_section, disable_mouse_capture, enable_mouse_capture, enter_alternate_screen, hide_cursor, leave_alternate_screen, move_cursor, resize, set_color, show_cursor, write_str}, controls::Controls, player::{Player, BIG_BLIND, SMALL_BLIND}, state::GameState}, poker::{card::{Card, BAIZE, CREAM, DBLUE}, deck::Deck, play::{analyze_play, Play}}};

pub struct Game {
    pub controls: Controls,
    pub deck: Deck,
    pub state: GameState,
    pub players: Vec<Player>,
    pub board: Vec<Card>,
    pub dealer: usize,
    pub current_bet: usize,
}

impl Game {
    pub fn new(num_players: usize) -> Self {
        let players = (0..num_players).into_iter()
            .map(|i| Player::new(i.to_string(), 100))
            .collect();

        Game { 
            controls: Controls::new(),
            deck: Deck::new(),
            state: GameState::Dealing,
            players,
            board: vec!(),
            dealer: 0,
            current_bet: 0,
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

    pub fn draw_player_plays(&self, plays: &Vec<Play>) {
        let winner = plays.iter().enumerate().max_by_key(|(_, i)| *i).unwrap().0;

        let play_str = |i: usize| {
            if i == winner {
                format!(">>> {} <<<", plays[i].name())
                
            } else {
                plays[i].name()
            }
        }; 

        self.draw_single_player_play(62, 27, play_str(0));
        self.draw_single_player_play_left(5, 9, play_str(1));
        self.draw_single_player_play(62, 13, play_str(2));
        self.draw_single_player_play_right(120, 31, play_str(3));
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
        self.draw_single_player_chips(5, 31, &self.players[1]);
        self.draw_single_player_chips(35, 3, &self.players[2]);
        self.draw_single_player_chips(109, 9, &self.players[3]);
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
        self.draw_single_player_bet(5, 34, &self.players[1]);
        self.draw_single_player_bet(35, 6, &self.players[2]);
        self.draw_single_player_bet(109, 6, &self.players[3]);
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

        if self.current_bet < chips {
            self.current_bet = chips;
        }

        self.draw_player_chips();
        self.draw_player_bets();
    }

    pub fn update(&mut self) -> bool {
        match self.state {
            GameState::MainMenu => todo!(),
            
            GameState::Dealing => {
                // Prepare cards
                self.deck.shuffle();

                for player in &mut self.players {
                    for _ in 0..2 {
                        player.give_card(self.deck.pop().expect("No more cards"));
                    }
                }

                for _ in 0..5 {
                    self.board.push(self.deck.pop().expect("No more cards"));
                }

                self.state = GameState::Round(0, (self.dealer + 1) % 4, false, false);

                // Draw green baize
                set_color(BAIZE, Color::Black);
                clear_section(0, 0, 40, 125);

                self.draw_player_chips();
                self.draw_player_bets();
                self.draw_dealer_chip();
            },

            GameState::Round(num_flipped, turn, sb, bb) => {
                if self.controls.is_pressed(KeyCode::Enter) && !self.controls.is_locked(KeyCode::Enter) {
                    self.controls.lock(KeyCode::Enter, Duration::from_millis(500));

                    if !sb && !bb { // Small blind
                        self.bet(turn, SMALL_BLIND);

                        self.state = GameState::Round(num_flipped, (turn + 1) % 4, true, bb);

                    } else if sb && !bb { // Big blind
                        self.bet(turn, BIG_BLIND);
                        
                        self.state = GameState::Round(num_flipped, (turn + 1) % 4, true, true);

                        self.players[0].hand.iter_mut().for_each(Card::reset_draw_cache);

                    } else { // Normal turn
                        if self.players[turn].bet < self.current_bet {
                            self.bet(turn, self.current_bet - self.players[turn].bet);
                        }

                        let balanced_bet = self.players.iter().all(|i| i.bet == self.current_bet);

                        if turn == self.dealer && balanced_bet {
                            if num_flipped < 5 {
                                self.board[num_flipped].reset_draw_cache();

                                // Pre-flop
                                if num_flipped == 0 {
                                    self.board[0].reset_draw_cache();
                                    self.board[1].reset_draw_cache();
                                    self.board[2].reset_draw_cache();
                                    self.state = GameState::Round(3, (turn + 1) % 4, true, true);

                                } else {
                                    self.state = GameState::Round(num_flipped + 1, (self.dealer + 1) % 4, sb, bb);
                                }

                            } else {
                                // TODO: win bets
                                let plays = self.players.iter().map(|p| analyze_play(&p.hand, &self.board)).collect::<Vec<_>>();

                                self.draw_player_plays(&plays);

                                self.players.iter_mut().for_each(Player::take_bet);
                                self.draw_player_bets();
                                self.draw_player_chips();

                                // Reset draw cache and proceed
                                self.players.iter_mut().flat_map(|p| &mut p.hand).for_each(Card::reset_draw_cache);
                                self.state = GameState::Resolving;
                            }

                        } else {
                            self.state = GameState::Round(num_flipped, (turn + 1) % 4, sb, bb);
                        }
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
                    self.dealer = (self.dealer + 1) % 4;
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
            
            GameState::Round(num_flipped, turn, sb, bb) => {
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