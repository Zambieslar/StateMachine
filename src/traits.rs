use crate::{constants::*, definitions::*, statemachine::*};

pub trait Machine {
    fn state(&self) -> State;
    fn offset(&self) -> usize;
    fn mindex(&self) -> usize;
    fn buf(&self) -> String;
    fn new() -> Self;
    fn next(&mut self);
    fn reverse(&mut self);
    fn run(&mut self, data: &[u8]);
    fn evaluate(comparator: String, key: Option<String>);
}
