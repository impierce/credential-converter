use core::slice::Iter;

#[derive(Debug)]
pub struct MappingRule {
    pub source: Vec<MappingData>,
    pub target: Vec<MappingData>,
    pub transformation: String,
    pub rule_number: usize, 
}


//source, AchievementSubject, type, AchievementSubjectType, 1|*, false, 
//source, AchievementSubjectType, name, Contact, 1, false



////0, source, Contact, phone

////0, target, Subject, credential, CredentialSubject, 1, false,
////0, target, CredentialSUbject, person, Contact, 1, false
////0, target, Contact, phone

//1, source, AchievementSubject, type, AchievementSubjectType, 1, false


//let mapping_rule =  column[0] 

//HashMap<usize, MappingRule>;




fn to_json_path(source: MappingRule) {
        //$.type.name.phone = "023423432";

    let src_json_path = "$.type.name.phone";
    let target_json_path = "$.credential.person.phone";
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
