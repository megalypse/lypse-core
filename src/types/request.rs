use std::collections::HashMap;

use super::{enums::ParamEntry, request_type::RequestType};

#[derive(Debug)]
pub struct Request {
    pub version: f64,
    pub request_type: RequestType,
    pub uri: String,
    pub headers: HashMap<String, String>,
    pub query_params: Vec<ParamEntry>,
}

impl Request {
    pub fn new(
        version: f64,
        request_type: RequestType,
        uri: &str,
        headers: HashMap<String, String>,
        query_params: Vec<ParamEntry>,
    ) -> Request {
        Request {
            version,
            request_type,
            uri: String::from(uri),
            headers,
            query_params,
        }
    }
}
