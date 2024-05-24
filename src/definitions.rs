#[derive(Clone, Debug)]
pub enum State {
    Start,
    Scan,
    Match,
    Complete,
}

pub enum SubState {
    Header,
    Date,
    Time,
    Message,
}

pub enum HeadState {
    SessionId,
    PlClientVersion,
    ReleaseDate,
    LaunchTime,
    Os,
    CurrentLicense,
    Features,
    HomeURL,
    InteractiveUser,
    UserTempPath,
    Imperonation,
    NetCreds,
    PortMonStat,
}

pub enum Action {
    ADD,
    DELETE,
}

pub struct StateMachine {
    pub state: (State, SubState),
    pub offset: usize,
    pub mindex: usize,
    pub line: i32,
    pub buf: String,
}

pub struct Header {
    session_id: Option<i32>,
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

pub struct Info {
    comp_object: String,
    cur_user: String,
    detected_nic: String,
    adobject: ADObject,
}

pub struct ADObject {
    ad_user: String,
    ad_comp: String,
    ad_groups: Vec<String>,
    ad_digest: Option<[u8; 256]>,
}

pub struct Printer {
    name: String,
    driver: String,
    action: Action,
    profile: Option<String>,
    error: Option<Error>,
}

pub struct Warning {
    time_stamp: (String, String),
    warning: String,
    warning_index: usize,
    warning_digest: Option<[u8; 256]>,
}

pub struct Error {
    error: String,
    error_index: usize,
    error_digest: Option<[u8; 256]>,
}

pub struct DeserializedLog {
    header: Header,
    info: Info,
}
