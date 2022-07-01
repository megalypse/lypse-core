use crate::types::enums::{QueryEntry, QueryParamType};

type Param = (String, String);

pub fn parse_query(raw_uri: &str) -> Vec<QueryEntry> {
    let raw_query = get_raw_params(raw_uri);

    get_params_list(&raw_query)
}

fn get_raw_params(raw_uri: &str) -> String {
    match raw_uri.split_once("?") {
        Some((_, params)) => String::from(params),
        None => String::new(),
    }
}

fn get_params_list(raw_query: &str) -> Vec<QueryEntry> {
    raw_query
        .split("&")
        .map(|x| {
            let key_value = x.split("=").collect::<Vec<&str>>();

            generate_query_entry((String::from(key_value[0]), String::from(key_value[1])))
        })
        .collect::<Vec<QueryEntry>>()
}

fn generate_query_entry(param: Param) -> QueryEntry {
    let (key, value) = param;

    let param_type = get_param_type(&key);

    match param_type {
        QueryParamType::Common => QueryEntry::SingleEntry((key, value)),
        QueryParamType::Nested => {
            let mut keys = get_nested_param_keys(&key);
            let last_key = keys.pop().unwrap();

            return wrap_nested_values(
                &mut keys,
                QueryEntry::SingleEntry((String::from(last_key), value)),
            );
        }
    }
}

fn wrap_nested_values(remaining_keys: &mut Vec<String>, evolving_entry: QueryEntry) -> QueryEntry {
    if !remaining_keys.is_empty() {
        let current_key = remaining_keys.pop();

        return wrap_nested_values(
            remaining_keys,
            QueryEntry::NestedEntry((current_key.unwrap(), Box::new(evolving_entry))),
        );
    }

    evolving_entry
}

fn get_param_type(param_key: &str) -> QueryParamType {
    if param_key.contains("[") {
        return QueryParamType::Nested;
    }

    QueryParamType::Common
}

fn get_nested_param_keys(raw_key: &str) -> Vec<String> {
    raw_key
        .replace("[", " ")
        .replace("]", "")
        .split_whitespace()
        .map(String::from)
        .collect::<Vec<String>>()
}

#[cfg(test)]
mod tests {
    use crate::{
        parser::query_parser::get_params_list,
        types::enums::{QueryEntry, QueryParamType},
    };

    use super::{generate_query_entry, get_nested_param_keys, get_param_type, get_raw_params};

    #[test]
    fn should_separate_context_from_params() {
        let uri = "/mock-context?name=John&surname=Doe";
        let raw_params = get_raw_params(uri);

        assert_eq!(raw_params, "name=John&surname=Doe");
    }

    #[test]
    fn should_create_a_list_of_params() {
        let uri = "/mock-context?name=John&surname=Doe";
        let raw_params = get_raw_params(uri);
        let params_list = get_params_list(&raw_params);

        assert_eq!(
            params_list,
            vec![
                QueryEntry::SingleEntry((String::from("name"), String::from("John"))),
                QueryEntry::SingleEntry((String::from("surname"), String::from("Doe")))
            ]
        );
    }

    #[test]
    fn should_get_nested_param_keys() {
        let sample_key = "level1[level2][level3]";
        let key_list = get_nested_param_keys(sample_key);

        assert_eq!(
            key_list,
            vec![
                String::from("level1"),
                String::from("level2"),
                String::from("level3")
            ]
        );
    }

    #[test]
    fn should_get_paramtype() {
        let common_param = "key";
        let nested_param = "key[key2]";

        assert_eq!(get_param_type(common_param), QueryParamType::Common);
        assert_eq!(get_param_type(nested_param), QueryParamType::Nested);
    }

    #[test]
    fn should_generate_query_entry() {
        let key = "key1[key2][key3]";
        let value = "value";

        let result = generate_query_entry((key.to_string(), value.to_string()));
        let expected = QueryEntry::NestedEntry((
            "key1".to_string(),
            Box::new(QueryEntry::NestedEntry((
                "key2".to_string(),
                Box::new(QueryEntry::SingleEntry((
                    "key3".to_string(),
                    value.to_string(),
                ))),
            ))),
        ));

        assert_eq!(expected, result);
    }
}
