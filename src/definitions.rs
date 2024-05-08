#[derive(Clone, Debug)]
pub enum State {
    Start,
    Scan,
    MatchFound,
    Complete,
}

pub enum SubState {
    Header,
    Date,
    Time,
    Message,
}

pub struct StateMachine {
    pub state: (State, SubState),
    pub offset: usize,
    pub mindex: usize,
    pub buf: String,
}

pub struct Header {
    session_id: char,
    pl_client_version: String,
    release_date: String,
    launch_time: String,
    operating_system: String,
    current_license: String,
    features: Vec<String>,
    home_url: String,
    interactive_user: String,
    user_temp_path: String,
    impersonated_account: String,
    network_credentials: String,
    pl_portmonitor_status: String,
}

impl State {
    pub const STATES: [State; 4] = [Self::Start, Self::Scan, Self::MatchFound, Self::Complete];
}
