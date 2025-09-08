use std::time::{Duration, Instant};

use crossterm::event::KeyCode;

use crate::{engine::game::Game};

pub mod engine {
    pub mod console;
    pub mod controls;
    pub mod timer;
    pub mod game;
    pub mod state;
    pub mod player;
}

pub mod poker {
    pub mod card;
    pub mod deck;
    pub mod play;
}

pub mod actor {
    pub mod action;
    pub mod actor;
}

fn main() {
    const TARGET_FPS: u64 = 15;
    let frame_duration: Duration = Duration::from_secs_f64(1.0 / TARGET_FPS as f64);
    
    let mut game = Game::new();

    game.startup();

    loop {
        let frame_start = Instant::now();

        game.poll_inputs();

        if game.controls.is_pressed(KeyCode::Esc) {
            break;
        }

        if game.update() {
            break;
        }

        game.render();

        let frame_time = Instant::now() - frame_start;
        if frame_time < frame_duration {
            spin_sleep::sleep(frame_duration - frame_time);
        }
    }
    
    game.finalize();
}
