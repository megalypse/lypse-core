use std::collections::HashMap;

use super::request_type::RequestType;

pub struct Request {
    version: f64,
    request_type: RequestType,
    uri: String,
}
