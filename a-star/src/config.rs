use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Value {
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
}

#[derive(Debug, Clone)]
pub struct Property {
    name: String,
    value: Value,
    description: String,
}

pub fn prop(name: &str, value: Value) -> Property {
    Property {
        name: name.to_owned(),
        value,
        description: "".to_owned(),
    }
}

// TODO: create derive macro 'Configurable' for structs:
//  - set_config(map<string, Value>)
//  - get_config() -> map<string, Value>
//  - get_property(string) -> Value
//  - set_property(string, Value)
// create derive macro 'Prop' for fields
// which registers struct's Config properties
struct Config {
    props: Vec<Property>,
}

impl Config {
    pub fn new(props: Vec<Property>) -> Self {
        Self { props: props }
    }

    pub fn from_json(json: &str) -> Self {
        let parsed: serde_json::Value = serde_json::from_str(json).unwrap();

        assert!(parsed.is_object());

        let mut props = vec![];
        for (prop_name, json_value) in parsed.as_object().unwrap().into_iter() {
            let parsed_val = match json_value {
                serde_json::Value::Bool(val) => Some(Value::Bool(*val)),
                serde_json::Value::Number(val) => {
                    if val.is_f64() {
                        Some(Value::Float(val.as_f64().unwrap()))
                    } else {
                        Some(Value::Int(val.as_i64().unwrap()))
                    }
                }
                serde_json::Value::String(val) => Some(Value::String(val.clone())),
                _ => None,
            };

            if let Some(value) = parsed_val {
                props.push(prop(prop_name, value));
            }
        }

        Config { props: vec![] }
    }
}
