use crate::types::{request::Request, request_type::RequestType};

use super::parser::RequestParser;

pub struct DefaultParser {
    supported_request_types: [RequestType],
}

impl RequestParser for DefaultParser {
    fn parse(&self, raw_request: &str) -> Request {}
}
