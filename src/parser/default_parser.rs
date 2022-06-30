use std::collections::HashMap;

use crate::types::{request::Request, request_type::RequestType};

use super::parser::RequestParser;

const REQUEST_TYPES: [RequestType; 4] = [
    RequestType::GET,
    RequestType::POST,
    RequestType::PATCH,
    RequestType::PUT,
];

pub struct DefaultParser {}

impl DefaultParser {
    fn get_request_type(&self, target: &str) -> RequestType {
        let request_types = REQUEST_TYPES
            .iter()
            .filter(|x| target.contains(&(x.value())))
            .collect::<Vec<&RequestType>>();

        let request_type = request_types
            .get(0)
            .expect("Failed to determine request type");

        *(*request_type)
    }

    fn get_uri(&self, target: &str) -> String {
        let sections = target.split(" ").collect::<Vec<&str>>();

        String::from(sections[1])
    }

    fn get_http_version(&self, target: &str) -> f64 {
        let sections = target.split(" ").collect::<Vec<&str>>();

        if let [.., version] = sections[2].split("/").collect::<Vec<&str>>()[..] {
            return version
                .trim()
                .parse::<f64>()
                .expect("Failed to determine HTTP version");
        }

        panic!("Failed to determine HTTP version")
    }

    fn get_headers(&self, lines: Vec<&str>) -> HashMap<String, String> {
        lines
            .into_iter()
            .filter(|x| self.is_header(x))
            .map(|x| x.split_once(":"))
            .filter(|result| result.is_some())
            .map(|result| {
                let (key, value) = result.unwrap();

                (String::from(key.trim()), String::from(value.trim()))
            })
            .collect::<HashMap<String, String>>()
    }

    fn is_header(&self, target: &str) -> bool {
        target.contains(":")
    }
}

impl RequestParser for DefaultParser {
    fn parse(&self, raw_request: &str) -> Request {
        let mut lines = raw_request.split("\r\n").collect::<Vec<&str>>();
        let first_line = lines[0];
        let _content = lines.pop();

        Request::new(
            self.get_http_version(first_line),
            self.get_request_type(first_line),
            &self.get_uri(first_line),
            self.get_headers(lines),
        )
    }
}

pub const SAMPLE_REQUEST: &str = "\
GET / HTTP/1.1
User-Agent: PostmanRuntime/7.29.0
Accept: */*
Postman-Token: 5042f47d-45ba-427b-9205-1b05e902c993
Host: 127.0.0.1:7878
Accept-Encoding: gzip, deflate, br
Connection: keep-alive\
";
