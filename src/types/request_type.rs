#[derive(Clone, Copy, Debug)]
pub enum RequestType {
    GET,
    POST,
    PUT,
    PATCH,
}

impl RequestType {
    pub fn value(&self) -> String {
        match *self {
            RequestType::GET => String::from("GET"),
            RequestType::POST => String::from("POST"),
            RequestType::PUT => String::from("PUT"),
            RequestType::PATCH => String::from("PATCH"),
        }
    }
}
