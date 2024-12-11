#![allow(dead_code, unused_variables)]

mod naive;

use naive::Naive;
use wordle::Wordle;

const GAME: &str = include_str!("../answers.txt");

fn main() {
    let game = Wordle::new();
    for answer in GAME.split_whitespace() {
        let guesser = Naive::new();
        game.play(answer, guesser);
    }
    println!("Hello, world!");
}
