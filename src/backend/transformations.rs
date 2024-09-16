use serde::{Deserialize, Serialize};
use serde_json::Value;

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum OneToOne {
    copy,
    toLowerCase,
    toUpperCase,
    takeIndex { index: usize },
    slice { index: usize },
}

impl OneToOne {
    pub fn apply(&self, value: Value) -> Value {
        match self {
            OneToOne::copy => value,
            OneToOne::toLowerCase => {
                if let Value::String(s) = value {
                    Value::String(s.to_lowercase())
                } else {
                    value
                }
            }
            OneToOne::toUpperCase => {
                if let Value::String(s) = value {
                    Value::String(s.to_uppercase())
                } else {
                    value
                }
            }
            OneToOne::takeIndex { index } => {
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
            OneToOne::slice { index } => {
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
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum OneToMany {
    split,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ManyToOne {
    concat,
}

impl ManyToOne {
    pub fn apply(&self, values: Vec<Value>) -> Value {
        match self {
            ManyToOne::concat => {
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

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum StringToOne {
    stringit,
}

impl StringToOne {
    pub fn apply(&self, value: String) -> Value {
        match self {
            StringToOne::stringit => Value::String(value),
        }
    }
}


#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum JsonToMarkdown {
    jsonToMarkdown,
}

impl JsonToMarkdown {
    pub fn apply(&self, value: Value) -> Value {
        match self {
            JsonToMarkdown::jsonToMarkdown => value,
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MarkdownToJson {
    markdownToJson,
}

impl MarkdownToJson {
    pub fn apply(&self, value: Value) -> Value {
        match self {
            MarkdownToJson::markdownToJson => value,
        }
    }
}



#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Transformation {
    OneToOne {
        type_: OneToOne,
        source: DataLocation,
        destination: DataLocation,
    },
    StringToOne {
        type_: StringToOne,
        source: StringValue,
        destination: DataLocation,
    },
    MarkdownToJson {
        type_: MarkdownToJson,
        source: DataLocation,
        destination: DataLocation,
    },
    JsonToMarkdown {
        type_: JsonToMarkdown,
        source: DataLocation,
        destination: DataLocation,
    },
    OneToMany {
        type_: OneToMany,
        source: DataLocation,
        destinations: Vec<DataLocation>,
    },
    ManyToOne {
        type_: ManyToOne,
        sources: Vec<DataLocation>,
        destination: DataLocation,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DataLocation {
    pub format: String,
    pub path: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StringValue {
    pub value: String,
}