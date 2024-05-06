mod constants;
mod statemachine;

use crate::statemachine::*;

fn main() {
    let mut machine = StateMachine::new();
    machine.next();
    println!("{:#?}", machine.state);
}
