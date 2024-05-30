use crate::{constants::*, definitions::*, statemachine::*, traits::*};

impl StateMachine {
    fn get_header(&mut self, iter: dyn Iterator) -> Header {
        let header = Header::default();
        iter.skip(self.offset + 20);
        for byte in iter {
            match &self.state.2 {
                HeadState::SessionId => {}
                HeadState::PlClientVersion => {}
                HeadState::ReleaseDate => {}
                HeadState::LaunchTime => {}
                HeadState::Os => {}
                HeadState::CurrentLicense => {}
                HeadState::Features => {}
                HeadState::HomeURL => {}
                HeadState::InteractiveUser => {}
                HeadState::UserTempPath => {}
                HeadState::Imperonation => {}
                HeadState::NetCreds => {}
                HeadState::PortMonStat => {}
            }
        }
        header
    }
}
