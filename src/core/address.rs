use std::fmt::Display;

pub struct Address {
    pub host: String,
    pub port: String,
}

impl Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let host = self.host.clone();
        let port = self.port.clone();

        write!(f, "{host}:{port}")
    }
}
