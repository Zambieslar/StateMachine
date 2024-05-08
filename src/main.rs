mod constants;
mod definitions;
mod statemachine;

use crate::definitions::*;
use crate::statemachine::*;

fn main() {
    let file = std::fs::read("/home/brandon/Downloads/PrinterLogicServicePrinterApp - Copy.log")
        .expect("Unable to read file");
    let mut machine = StateMachine::new();
    machine.run(&file);
}
