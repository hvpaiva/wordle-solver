use wordle::{Guess, Guesser};

pub struct Naive {}

impl Guesser for Naive {
    fn guess(&mut self, previous: &[Guess]) -> String {
        "abcde".to_string()
    }
}

impl Naive {
    pub fn new() -> Naive {
        Naive {}
    }
}
