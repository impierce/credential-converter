use core::slice::Iter;

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
