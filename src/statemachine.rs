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
    pub offset: i32,
    pub mindex: i32,
    pub buf: String,
}

pub trait Machine {
    fn state(&self) -> State;
    fn offset(&self) -> i32;
    fn mindex(&self) -> i32;
    fn buf(&self) -> String;
    fn new() -> Self;
    fn next(&mut self);
    fn run(&mut self);
}

impl Machine for StateMachine {
    fn state(&self) -> State {
        self.state.clone()
    }

    fn offset(&self) -> i32 {
        self.offset.clone()
    }

    fn mindex(&self) -> i32 {
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

    fn run(&mut self) {
        match self.state {
            State::Start => {}
            State::Scan => {}
            State::MatchFound => {}
            State::Complete => {}
        }
    }
}
