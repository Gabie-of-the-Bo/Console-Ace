use crossterm::style::Color;

use crate::engine::console::{clear_section, move_cursor, set_color, write_str};

pub const BAIZE: Color = Color::Rgb { r: 53, g: 101, b: 77 };
pub const CREAM: Color = Color::Rgb { r: 227, g: 168, b: 105 };

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Suit {
    Hearts, Diamonds, Clubs, Spades
}

pub struct Card {
    pub suit: Suit,
    pub number: usize,
    drawn: bool
}

impl Suit {
    pub fn symbol(&self) -> &str {
        match self {
            Suit::Hearts => "♥",
            Suit::Diamonds => "♦",
            Suit::Clubs => "♣",
            Suit::Spades => "♠",
        }
    }

    pub fn color(&self) -> Color {
        match self {
            Suit::Hearts => Color::Red,
            Suit::Diamonds => Color::Red,
            Suit::Clubs => Color::Black,
            Suit::Spades => Color::Black,
        }
    }
}

impl Card {
    pub fn new(number: usize, suit: Suit) -> Self {
        Card { suit, number, drawn: false }
    }

    pub fn value(&self) -> usize {
        match self.number {
            1 => 14,
            n => n
        }
    }

    pub fn reset_draw_cache(&mut self) {
        self.drawn = false;
    }

    pub fn draw(&mut self, col: usize, row: usize, back: bool) {
        // Cache console
        if self.drawn {
            return;
        }

        self.drawn = true;

        const WIDTH: usize = 10;
        const HEIGHT: usize = 8;

        // Outer square
        set_color(Color::White, BAIZE);
        clear_section(row, col, row + HEIGHT, col + WIDTH);

        set_color(BAIZE, Color::White);
        move_cursor(row, col);
        write_str(&"▄".repeat(11));
        move_cursor(row + HEIGHT, col);
        write_str(&"▀".repeat(11));

        if back {
            set_color(Color::White, Color::DarkBlue);

            for r in row + 1..row + HEIGHT {
                for c in col + 1..col + WIDTH {
                    move_cursor(r, c);

                    if (r + c) % 2 == 1 {
                        write_str("▓");
                    
                    } else {
                        write_str("▒");
                    }
                }
            }

            return;
        }
        
        // Corner symbols
        let number = match self.number {
            1 => "A".to_string(),
            11 => "J".to_string(),
            12 => "Q".to_string(),
            13 => "K".to_string(),
            n => n.to_string()
        };

        set_color(Color::White, self.suit.color());

        move_cursor(row + 1, col + 1);
        write_str(&number);
        move_cursor(row + 2, col + 1);
        write_str(self.suit.symbol());
        
        move_cursor(row + HEIGHT - 2, col + WIDTH - 1);
        write_str(self.suit.symbol());
        move_cursor(row + HEIGHT - 1, col + WIDTH - number.len());
        write_str(&number);

        let write_suit = |row: usize, col: usize| {
            move_cursor(row, col);
            write_str(self.suit.symbol());
        };

        // Inner symbols
        match self.number {
            1 => {
                write_suit(row + HEIGHT / 2, col + WIDTH / 2);
            }

            2 => {
                write_suit(row + HEIGHT / 2 - 2, col + WIDTH / 2);
                write_suit(row + HEIGHT / 2 + 2, col + WIDTH / 2);
            }

            3 => {
                write_suit(row + HEIGHT / 2 - 2, col + WIDTH / 2);
                write_suit(row + HEIGHT / 2, col + WIDTH / 2);
                write_suit(row + HEIGHT / 2 + 2, col + WIDTH / 2);
            }

            4 => {
                write_suit(row + HEIGHT / 2 - 2, col + WIDTH / 2 - 2);
                write_suit(row + HEIGHT / 2 + 2, col + WIDTH / 2 - 2);
                write_suit(row + HEIGHT / 2 - 2, col + WIDTH / 2 + 2);
                write_suit(row + HEIGHT / 2 + 2, col + WIDTH / 2 + 2);
            }

            5 => {
                write_suit(row + HEIGHT / 2, col + WIDTH / 2);
                write_suit(row + HEIGHT / 2 - 2, col + WIDTH / 2 - 2);
                write_suit(row + HEIGHT / 2 + 2, col + WIDTH / 2 - 2);
                write_suit(row + HEIGHT / 2 - 2, col + WIDTH / 2 + 2);
                write_suit(row + HEIGHT / 2 + 2, col + WIDTH / 2 + 2);
            }

            6 => {
                write_suit(row + HEIGHT / 2 - 2, col + WIDTH / 2 - 2);
                write_suit(row + HEIGHT / 2, col + WIDTH / 2 - 2);
                write_suit(row + HEIGHT / 2 + 2, col + WIDTH / 2 - 2);
                write_suit(row + HEIGHT / 2 - 2, col + WIDTH / 2 + 2);
                write_suit(row + HEIGHT / 2, col + WIDTH / 2 + 2);
                write_suit(row + HEIGHT / 2 + 2, col + WIDTH / 2 + 2);
            }

            7 => {
                write_suit(row + HEIGHT / 2 - 2, col + WIDTH / 2 - 2);
                write_suit(row + HEIGHT / 2, col + WIDTH / 2 - 2);
                write_suit(row + HEIGHT / 2 + 2, col + WIDTH / 2 - 2);
                write_suit(row + HEIGHT / 2 - 2, col + WIDTH / 2 + 2);
                write_suit(row + HEIGHT / 2, col + WIDTH / 2 + 2);
                write_suit(row + HEIGHT / 2 + 2, col + WIDTH / 2 + 2);
                write_suit(row + HEIGHT / 2 + 1, col + WIDTH / 2);
            }

            8 => {
                write_suit(row + HEIGHT / 2 - 2, col + WIDTH / 2 - 2);
                write_suit(row + HEIGHT / 2, col + WIDTH / 2 - 2);
                write_suit(row + HEIGHT / 2 + 2, col + WIDTH / 2 - 2);
                write_suit(row + HEIGHT / 2 - 2, col + WIDTH / 2 + 2);
                write_suit(row + HEIGHT / 2, col + WIDTH / 2 + 2);
                write_suit(row + HEIGHT / 2 + 2, col + WIDTH / 2 + 2);
                write_suit(row + HEIGHT / 2 + 1, col + WIDTH / 2);
                write_suit(row + HEIGHT / 2 - 1, col + WIDTH / 2);
            }

            9 => {
                write_suit(row + HEIGHT / 2 - 2, col + WIDTH / 2 - 2);
                write_suit(row + HEIGHT / 2, col + WIDTH / 2 - 2);
                write_suit(row + HEIGHT / 2 + 2, col + WIDTH / 2 - 2);
                write_suit(row + HEIGHT / 2 - 2, col + WIDTH / 2 + 2);
                write_suit(row + HEIGHT / 2, col + WIDTH / 2 + 2);
                write_suit(row + HEIGHT / 2 + 2, col + WIDTH / 2 + 2);
                write_suit(row + HEIGHT / 2 - 2, col + WIDTH / 2);
                write_suit(row + HEIGHT / 2, col + WIDTH / 2);
                write_suit(row + HEIGHT / 2 + 2, col + WIDTH / 2);
            }

            10 => {
                write_suit(row + HEIGHT / 2 - 2, col + WIDTH / 2 - 2);
                write_suit(row + HEIGHT / 2, col + WIDTH / 2 - 2);
                write_suit(row + HEIGHT / 2 + 2, col + WIDTH / 2 - 2);
                write_suit(row + HEIGHT / 2 - 2, col + WIDTH / 2 + 2);
                write_suit(row + HEIGHT / 2, col + WIDTH / 2 + 2);
                write_suit(row + HEIGHT / 2 + 2, col + WIDTH / 2 + 2);
                write_suit(row + HEIGHT / 2 + 1, col + WIDTH / 2);
                write_suit(row + HEIGHT / 2 - 1, col + WIDTH / 2);
                write_suit(row + HEIGHT / 2 - 2, col + WIDTH / 2);
                write_suit(row + HEIGHT / 2 + 2, col + WIDTH / 2);
            }

            11 | 12 | 13 => {
                // Should draw something
                move_cursor(row + HEIGHT / 2, col + WIDTH / 2);
                write_str(&number);
            }

            _ => todo!()
        }
    }
}