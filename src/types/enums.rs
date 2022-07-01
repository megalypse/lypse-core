use regex::Regex;

#[derive(Debug, PartialEq)]
pub enum QueryParamType {
    Common,
    Nested,
}

#[derive(Debug, PartialEq)]

pub enum ParamEntry {
    SingularEntry((String, String)),
    NestedEntry((String, Box<ParamEntry>)),
}

impl ParamEntry {
    pub fn get_value(&self) -> &String {
        match self {
            ParamEntry::SingularEntry((_, value)) => value,
            ParamEntry::NestedEntry((_, value)) => ParamEntry::get_value(value),
        }
    }

    pub fn get_value_as_vector(&self) -> Vec<String> {
        let final_value = ParamEntry::get_value(self);

        if self.is_list_value(final_value) {
            return self.parse_list_value(final_value);
        }

        vec![String::from(final_value)]
    }

    fn parse_list_value(&self, raw_value: &str) -> Vec<String> {
        raw_value
            .replace("[", "")
            .replace("]", "")
            .replace("\"", "")
            .split(",")
            .map(|x| String::from(x.trim()))
            .collect::<Vec<String>>()
    }

    fn is_list_value(&self, value: &str) -> bool {
        let is_list_regex = Regex::new("\\[(\"?.*\"?,?)+\\]").unwrap();

        is_list_regex.is_match(value)
    }
}

mod tests {
    use crate::types::enums::ParamEntry;

    #[test]
    fn should_get_final_value() {
        let nested_values = ParamEntry::NestedEntry((
            String::from("key1"),
            Box::new(ParamEntry::NestedEntry((
                "key2".to_string(),
                Box::new(ParamEntry::SingularEntry((
                    "key3".to_string(),
                    "final value".to_string(),
                ))),
            ))),
        ));

        let expected_value = "final value";
        let expected_vector_value = vec![expected_value];

        assert_eq!(expected_value, nested_values.get_value());
        assert_eq!(expected_vector_value, nested_values.get_value_as_vector());
    }

    #[test]
    fn should_check_if_is_a_list_value() {
        let string_value = "value";
        let string_value2 = "[value";
        let string_value3 = "value]";

        let list_value = "[value1,value2]";
        let list_value2 = "[\"value1\",\"value2\"]";
        let list_value3 = "[\"value1\"\"value2\"]";
        let list_value4 = "[value1]";
        let list_value5 = "[\"value1\"]";

        let caller_value = ParamEntry::SingularEntry(("".to_string(), "".to_string()));

        assert_eq!(false, caller_value.is_list_value(string_value));
        assert_eq!(false, caller_value.is_list_value(string_value2));
        assert_eq!(false, caller_value.is_list_value(string_value3));

        assert_eq!(true, caller_value.is_list_value(list_value));
        assert_eq!(true, caller_value.is_list_value(list_value2));
        assert_eq!(true, caller_value.is_list_value(list_value3));
        assert_eq!(true, caller_value.is_list_value(list_value4));
        assert_eq!(true, caller_value.is_list_value(list_value5));
    }

    #[test]
    fn should_parse_list_value() {
        let list_value = "[value1, value2, value3]";
        let list_value2 = "\"value1\",\"value2\",\"value3\"";

        let expected_result = vec![
            String::from("value1"),
            String::from("value2"),
            String::from("value3"),
        ];

        let caller_value = ParamEntry::SingularEntry(("".to_string(), "".to_string()));

        assert_eq!(expected_result, caller_value.parse_list_value(list_value));
        assert_eq!(expected_result, caller_value.parse_list_value(list_value2));
    }
}
