use std::{iter, rc::Rc};

use crate::{constants::*, definitions::*, traits::*};

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
            line: 0,
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

    fn reverse(&mut self) {
        match self.state() {
            state => {
                self.state.0 = State::STATES[state as usize - 1].clone();
            }
        }
    }

    fn run(&mut self, data: &[u8]) {
        let iter = Rc::new(data);
        loop {
            match self.state.0 {
                State::Start => self.next(),
                State::Scan => match self.state.1 {
                    SubState::Header => {
                        let iter = data.into_iter();
                        for byte in iter.enumerate() {
                            match *byte.1 {
                                DELIMITER => {
                                    let token = self.buf.chars().as_str();
                                    match token {
                                        _ => {}
                                    }
                                }
                                NEWLINE => {
                                    self.line = self.line + 1;
                                }
                                _ => self.buf.push(*byte.1 as char),
                            }
                        }
                    }
                    SubState::Time => {}
                    SubState::Date => {}
                    SubState::Message => {}
                },
                State::Match => {}
                State::Complete => {}
            }
        }
    }
}
