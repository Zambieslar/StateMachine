use std::mem::size_of_val;

use crate::{definitions::*, statemachine::*};

pub enum Keys {}

pub const NEWLINE: u8 = 10; // Tracking each line using byte value of new line
pub const DELIMITER: u8 = 20; // Delimiting each token using byte value for space

pub struct ResultData {}

impl Keys {
    pub const KEYS: [&'static str; 7] = [
        "Error", "Warning", "Can't", "Failed", "Unable", "Failure", "Unknown",
    ];
}

impl State {
    pub const STATES: [State; 4] = [Self::Start, Self::Scan, Self::Match, Self::Complete];
}

impl HeadState {
    pub const HEADSTATE: [HeadState; 13] = [
        Self::SessionId,
        Self::PlClientVersion,
        Self::ReleaseDate,
        Self::LaunchTime,
        Self::Os,
        Self::CurrentLicense,
        Self::Features,
        Self::HomeURL,
        Self::InteractiveUser,
        Self::UserTempPath,
        Self::Imperonation,
        Self::NetCreds,
        Self::PortMonStat,
    ];
}
