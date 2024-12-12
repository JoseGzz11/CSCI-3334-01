use std::fmt;

#[derive(Debug)]
pub struct MonitorError(String);

impl fmt::Display for MonitorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<ureq::Error> for MonitorError {
    fn from(error: ureq::Error) -> Self {
        MonitorError(format!("{:?}", error))
    }
}
