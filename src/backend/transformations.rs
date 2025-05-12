use serde::{Deserialize, Serialize};
use serde_json::Value;

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum OneToOneType {
    copy,
    toLowerCase,
    toUpperCase,
    takeIndex { index: usize },
    slice { index: usize },
    delete,
    regex, // TODO
}

impl OneToOneType {
    pub fn apply(&self, value: Value) -> Value {
        match self {
            OneToOneType::copy => value,
            OneToOneType::toLowerCase => {
                if let Value::String(s) = value {
                    Value::String(s.to_lowercase())
                } else {
                    value
                }
            }
            OneToOneType::toUpperCase => {
                if let Value::String(s) = value {
                    Value::String(s.to_uppercase())
                } else {
                    value
                }
            }
            OneToOneType::takeIndex { index } => {
                if let Value::Array(array) = &value {
                    if let Some(slice) = array.get(*index) {
                        slice.clone()
                    } else {
                        value
                    }
                } else if let Value::String(s) = &value {
                    if let Some(slice) = s.chars().nth(*index) {
                        Value::String(slice.to_string())
                    } else {
                        value
                    }
                } else {
                    value
                }
            }
            OneToOneType::slice { index } => {
                if let Value::Array(array) = &value {
                    if let Some(slice) = array.get(..*index + 1) {
                        Value::Array(slice.to_vec())
                    } else {
                        value
                    }
                } else if let Value::String(s) = &value {
                    if let Some(slice) = s.get(..*index + 1) {
                        Value::String(slice.to_string())
                    } else {
                        value
                    }
                } else {
                    value
                }
            }
            OneToOneType::delete => Value::Null,
            _ => value, // remaining arms are handled under different multiplicity implementations
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum OneToManyType {
    split, // TODO
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum ManyToOneType {
    concat, // TODO
}

impl ManyToOneType {
    pub fn apply(&self, values: Vec<Value>) -> Value {
        match self {
            &ManyToOneType::concat => {
                let mut s = String::new();
                for value in values {
                    if let Value::String(string) = value {
                        s.push_str(&string);
                    }
                }
                Value::String(s)
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Multiplicity {
    OneToOne(OneToOneType),
    OneToMany(OneToManyType),
    ManyToOne(ManyToOneType),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transformation {
    pub type_: Multiplicity,
    pub source: DataLocations,
    pub destination: DataLocations,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DataLocation {
    pub format: String,
    pub path: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DataLocations(pub Vec<DataLocation>);

impl DataLocations {
    pub fn first_index(&self) -> &DataLocation {
        self.0.first().expect("No value provided in Vec<DataLocation>")
    }
}
