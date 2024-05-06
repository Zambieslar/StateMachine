use crate::constants::*;
use core::slice::SlicePattern;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub enum State {
    Start,
    Scan,
    MatchFound,
    Complete,
}

impl State {
    const STATES: [State; 4] = [Self::Start, Self::Scan, Self::MatchFound, Self::Complete];
}

pub struct StateMachine {
    pub state: State,
    pub offset: usize,
    pub mindex: usize,
    pub buf: String,
}

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
        self.state.clone()
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
            state: State::Start,
            offset: 0,
            mindex: 0,
            buf: String::new(),
        }
    }

    fn next(&mut self) {
        match self.state() {
            state => {
                self.state = State::STATES[state as usize + 1].clone();
            }
        }
    }

    fn run(&mut self, data: &[u8]) {
        let iter = Rc::new(data);
        match self.state {
            State::Start => self.next(),
            State::Scan => {
                for i in iter.into_iter().clone().enumerate() {
                    self.offset = i.0;
                    if ERROR.chars().nth(0) == Some(*i.1 as char) {
                        self.next();
                    }
                }
            }
            State::MatchFound => {
                for i in iter.clone().chunks(ERROR.len()).skip(self.offset()) {
                    let collection: String = i.into_iter().map(|x| *x as char).collect();
                    match collection {
                        ERROR => {
                            self.mindex = self.offset();
                        }
                    }
                }
            }
            State::Complete => {}
        }
    }
}
