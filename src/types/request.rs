use std::collections::HashMap;

use super::request_type::RequestType;

#[derive(Debug)]
pub struct Request {
    pub version: f64,
    pub request_type: RequestType,
    pub uri: String,
    pub headers: HashMap<String, String>,
}

impl Request {
    pub fn new(
        version: f64,
        request_type: RequestType,
        uri: &str,
        headers: HashMap<String, String>,
    ) -> Request {
        Request {
            version,
            request_type,
            uri: String::from(uri),
            headers,
        }
    }
}
