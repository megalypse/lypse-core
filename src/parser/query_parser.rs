type Param = (String, String);

pub fn parse_query(raw_uri: &str) -> Vec<Param> {
    let raw_query = get_raw_params(raw_uri);

    get_params_list(&raw_query)
}

fn get_raw_params(raw_uri: &str) -> String {
    match raw_uri.split_once("?") {
        Some((_, params)) => String::from(params),
        None => String::new(),
    }
}

fn get_params_list(raw_query: &str) -> Vec<Param> {
    raw_query
        .split("&")
        .map(|x| {
            let key_value = x.split("=").collect::<Vec<&str>>();

            (String::from(key_value[0]), String::from(key_value[1]))
        })
        .collect::<Vec<Param>>()
}

#[cfg(test)]
mod tests {
    use crate::parser::query_parser::get_params_list;

    use super::get_raw_params;

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
                (String::from("name"), String::from("John")),
                (String::from("surname"), String::from("Doe"))
            ]
        );
    }
}
