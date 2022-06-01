use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Value {
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
}

impl Value {
    pub fn bool(&self) -> Option<bool> {
        if let Value::Bool(b) = self {
            Some(*b)
        } else {
            None
        }
    }
    pub fn int(&self) -> Option<i64> {
        if let Value::Int(i) = self {
            Some(*i)
        } else {
            None
        }
    }
    pub fn float(&self) -> Option<f64> {
        if let Value::Float(f) = self {
            Some(*f)
        } else {
            None
        }
    }
    pub fn str(&self) -> Option<&str> {
        if let Value::String(s) = self {
            Some(&s)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct Property {
    pub name: String,
    pub value: Value,
    pub description: String,
}

pub fn prop(name: &str, value: Value) -> Property {
    Property {
        name: name.to_owned(),
        value,
        description: "".to_owned(),
    }
}

pub struct ConfigSpec {
    props: Vec<Property>,
}

pub struct Config {
    values: HashMap<String, Value>,
}

impl ConfigSpec {
    pub fn new(props: Vec<Property>) -> Self {
        Self { props }
    }

    pub fn set(&mut self, name: &str, value: Value) {
        if let Some(prop) = self.props.iter_mut().find(|prop| prop.name == name) {
            prop.value = value;
        }
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        self.props
            .iter()
            .find(|&prop| prop.name == name)
            .map(|prop| prop.value.clone())
    }

    pub fn values(&self) -> Config {
        let values_iter = self.props.iter().map(|prop| (prop.name.clone(), prop.value.clone()));
        let vals = HashMap::from_iter(values_iter);
        Config::new(vals)
    }
}

impl Config {
    pub fn new(values: HashMap<String, Value>) -> Self {
        Self {
            values
        }
    }

    pub fn from_json(json: &str) -> Self {
        let parsed: serde_json::Value = serde_json::from_str(json).unwrap();

        assert!(parsed.is_object());

        let mut values = HashMap::new();
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
                values.insert(prop_name.to_owned(), value);
            }
        }

        Self { values }
    }
}

impl IntoIterator for Config {
    type Item = (String, Value);
    type IntoIter = std::collections::hash_map::IntoIter<String, Value>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.into_iter()
    }
}

pub trait Configurable {
    fn get_config(&self) -> Config;
    fn set_config(&mut self, config: Config);
}
