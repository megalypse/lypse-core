use super::request_type::RequestType;

#[derive(Debug)]
pub struct Request {
    pub version: f64,
    pub request_type: RequestType,
    pub uri: String,
}

impl Request {
    pub fn new(version: f64, request_type: RequestType, uri: &str) -> Request {
        Request {
            version,
            request_type,
            uri: String::from(uri),
        }
    }
}
