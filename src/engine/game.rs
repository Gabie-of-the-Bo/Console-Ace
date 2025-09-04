use std::time::Duration;

use crossterm::{event::{self, Event, KeyCode, MouseEventKind}, style::Color, terminal::{disable_raw_mode, enable_raw_mode}};

use crate::{engine::{console::{clear, clear_section, disable_mouse_capture, enable_mouse_capture, enter_alternate_screen, hide_cursor, leave_alternate_screen, move_cursor, resize, set_color, show_cursor}, controls::Controls, player::Player, state::GameState}, poker::{card::{Card, BAIZE}, deck::Deck}};

pub struct Game {
    pub controls: Controls,
    pub deck: Deck,
    pub state: GameState,
    pub players: Vec<Player>,
    pub board: Vec<Card>
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
            board: vec!()
        }
    }

    pub fn startup(&mut self) {
        resize(40, 125);

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

                self.state = GameState::Round(3);

                // Draw green baize
                set_color(BAIZE, Color::Black);
                clear_section(0, 0, 40, 125);
            },

            GameState::Round(num_flipped) => {
                if self.controls.is_pressed(KeyCode::Enter) {
                    if num_flipped < 5 {
                        self.board[num_flipped].reset_draw_cache();
                        self.state = GameState::Round(num_flipped + 1);

                    } else {
                        self.players.iter_mut().flat_map(|p| &mut p.hand).for_each(Card::reset_draw_cache);
                        self.state = GameState::Resolving;
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
                if self.controls.is_pressed(KeyCode::Enter) {
                    self.state = GameState::Collecting;
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
            
            GameState::Round(num_flipped) => {
                // Center cards
                for (i, card) in self.board.iter_mut().enumerate() {
                    card.draw(27 + i * 15, 15, i >= num_flipped);
                }

                // Players
                for (i, card) in self.players[3].hand.iter_mut().enumerate() {
                    card.draw(109, 10 + i * 10, true);
                }

                for (i, card) in self.players[2].hand.iter_mut().enumerate() {
                    card.draw(25 + 24 + i * 16, 2, true);
                }

                for (i, card) in self.players[1].hand.iter_mut().enumerate() {
                    card.draw(5, 10 + i * 10, true);
                }

                for (i, card) in self.players[0].hand.iter_mut().enumerate() {
                    card.draw(25 + 24 + i * 16, 29, false);
                }
            },

            GameState::Resolving => {
                for (i, card) in self.players[3].hand.iter_mut().enumerate() {
                    card.draw(109, 10 + i * 10, false);
                }

                for (i, card) in self.players[2].hand.iter_mut().enumerate() {
                    card.draw(25 + 24 + i * 16, 2, false);
                }

                for (i, card) in self.players[1].hand.iter_mut().enumerate() {
                    card.draw(5, 10 + i * 10, false);
                }

                for (i, card) in self.players[0].hand.iter_mut().enumerate() {
                    card.draw(25 + 24 + i * 16, 29, false);
                }
            },
        }
    }
}