pub enum Keys {
    Error(String),
    Warning(String),
    Date(String),
    Time(String),
}

pub struct ResultData {}

impl Keys {
    pub fn as_str(&self) -> &'static str {
        match self {
            Keys::Error(_) => "Error",
            Keys::Warning(_) => "Warning",
            Keys::Date(_) => "Date",
            Keys::Time(_) => "Time",
        }
    }
}
