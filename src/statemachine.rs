use std::iter;
use std::rc::Rc;

use crate::constants::*;
use crate::definitions::*;

pub trait Machine {
    fn state(&self) -> State;
    fn offset(&self) -> usize;
    fn mindex(&self) -> usize;
    fn buf(&self) -> String;
    fn new() -> Self;
    fn next(&mut self);
    fn run(&mut self, data: &[u8]);
}

impl Machine for StateMachine {
    fn state(&self) -> State {
        self.state.0.clone()
    }

    fn offset(&self) -> usize {
        self.offset.clone()
    }

    fn mindex(&self) -> usize {
        self.mindex.clone()
    }

    fn buf(&self) -> String {
        self.buf.clone()
    }

    fn new() -> Self {
        Self {
            state: (State::Start, SubState::Header),
            offset: 0,
            mindex: 0,
            buf: String::new(),
        }
    }

    fn next(&mut self) {
        match self.state() {
            state => {
                self.state.0 = State::STATES[state as usize + 1].clone();
            }
        }
    }

    fn run(&mut self, data: &[u8]) {
        let iter = Rc::new(data);
        loop {
            match self.state.0 {
                State::Start => self.next(),
                State::Scan => match self.state.1 {
                    SubState::Header => for i in iter.clone().into_iter() {},
                    SubState::Time => {}
                    SubState::Date => {}
                    SubState::Message => {}
                },
                State::MatchFound => {}
                State::Complete => {}
            }
        }
    }
}
