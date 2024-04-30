use crate::game::Game;

mod rank;
mod game_result;
mod game;
mod player;

fn main() {
    let game_line = "   1   Player_79 (9 Dan)                          - Player_1 (7 Dan)                           1-0                              ";
    let game: Game = game_line.parse().unwrap();
    println!("{:?}", game);
}
