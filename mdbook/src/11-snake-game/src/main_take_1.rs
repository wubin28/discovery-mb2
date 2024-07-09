#![no_main]
#![no_std]

mod game;

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use microbit::{
    Board,
    hal::{prelude::*, Rng, Timer},
    display::blocking::Display
};
use rtt_target::rtt_init_print;
use panic_rtt_target as _;
use crate::game::{Game, GameStatus, Turn};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let mut board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut rng = Rng::new(board.RNG);
    let mut game = Game::new(rng.random_u32());
    let mut display = Display::new(board.display_pins);

    loop {
        loop {  // Game loop
            let image = game.game_matrix(9, 9, 9);
            // The brightness values are meaningless at the moment as we haven't yet implemented a display capable of
            // displaying different brightnesses
            display.show(&mut timer, image, game.step_len_ms());
            match game.status {
                GameStatus::Ongoing => game.step(Turn::None), // Placeholder as we haven't implemented controls yet
                _ => {
                    for _ in 0..3 {
                        display.clear();
                        timer.delay_ms(200u32);
                        display.show(&mut timer, image, 200);
                    }
                    display.clear();
                    display.show(&mut timer, game.score_matrix(), 1000);
                    break
                }
            }
        }
        game.reset();
    }
}
