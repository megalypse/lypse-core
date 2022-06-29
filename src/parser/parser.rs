use crate::types::request::Request;

pub trait RequestParser {
    fn parse(&self, raw_request: &str) -> Request;
}
