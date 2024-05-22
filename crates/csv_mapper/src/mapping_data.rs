use core::slice::Iter;
use std::ops;

pub struct JsonPathStr(String);

impl ops::Deref for JsonPathStr {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl JsonPathStr {
    pub fn to_segments(&self) -> Vec<String> {
        Vec::new()
    }
}


#[derive(Debug)]
pub struct MappingRule {
    pub src_path: String,
    pub target_path: String,
    pub transformation: String,
}

#[derive(Debug)]
pub struct MappingData(Vec<MappingRule>);

impl MappingData {
    pub fn new(rules: Vec<MappingRule>) -> Self {
        Self(rules)
    }

    pub fn iter(&self) -> Iter<MappingRule> {
        self.0.iter()
    }
}
