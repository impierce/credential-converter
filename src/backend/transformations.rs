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
pub enum StringArrayToOne {
    stringArrayIt,
}

impl StringArrayToOne {
    pub fn apply(&self, value: Value) -> Value {
        match self {
            StringArrayToOne::stringArrayIt => value,
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

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AddIdentifier {
    addIdentifier,
}

impl AddIdentifier {
    pub fn apply(&self, value: Value) -> Value {
        match self {
            AddIdentifier::addIdentifier => value,
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum IdentifierToObject {
    identifierToObject,
}

impl IdentifierToObject {
    pub fn apply(&self, value: Value) -> Value {
        match self {
            IdentifierToObject::identifierToObject => value,
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ImageToIndividualDisplay {
    imageToIndividualDisplay,
}

impl ImageToIndividualDisplay {
    pub fn apply(&self, value: Value) -> Value {
        match self {
            ImageToIndividualDisplay::imageToIndividualDisplay => value,
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AssessmentToProvenBy {
    assessmentToProvenBy,
}

impl AssessmentToProvenBy {
    pub fn apply(&self, value: Value) -> Value {
        match self {
            AssessmentToProvenBy::assessmentToProvenBy => value,
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ObjectToNoteLiteral {
    objectToNoteLiteral,
}

impl ObjectToNoteLiteral {
    pub fn apply(&self, value: Value) -> Value {
        match self {
            ObjectToNoteLiteral::objectToNoteLiteral => value,
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TranslateLearningOutcome {
    translateLearningOutcome,
}

impl TranslateLearningOutcome {
    pub fn apply(&self, value: Value) -> Value {
        match self {
            TranslateLearningOutcome::translateLearningOutcome => value,
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CreateLearningOutcomeSummary {
    createLearningOutcomeSummary,
}

impl CreateLearningOutcomeSummary {
    pub fn apply(&self, value: Value) -> Value {
        match self {
            CreateLearningOutcomeSummary::createLearningOutcomeSummary => value,
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CreditToSpecifiedByObject {
    creditToSpecifiedByObject,
}

impl CreditToSpecifiedByObject {
    pub fn apply(&self, value: Value) -> Value {
        match self {
            CreditToSpecifiedByObject::creditToSpecifiedByObject => value,
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EqfToSpecifiedByQualification {
    eqfToSpecifiedByQualification,
}

impl EqfToSpecifiedByQualification {
    pub fn apply(&self, value: Value) -> Value {
        match self {
            EqfToSpecifiedByQualification::eqfToSpecifiedByQualification => value,
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum LearningSettingToSpecifiedByObject {
    learningSettingToSpecifiedByObject,
}

impl LearningSettingToSpecifiedByObject {
    pub fn apply(&self, value: Value) -> Value {
        match self {
            LearningSettingToSpecifiedByObject::learningSettingToSpecifiedByObject => value,
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TitleToSpecifiedByObject {
    titleToSpecifiedByObject,
}

impl TitleToSpecifiedByObject {
    pub fn apply(&self, value: Value) -> Value {
        match self {
            TitleToSpecifiedByObject::titleToSpecifiedByObject => value,
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ImageToMediaObject {
    imageToMediaObject,
}

impl ImageToMediaObject {
    pub fn apply(&self, value: Value) -> Value {
        match self {
            ImageToMediaObject::imageToMediaObject => value,
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AddressToLocation {
    addressToLocation,
}

impl AddressToLocation {
    pub fn apply(&self, value: Value) -> Value {
        match self {
            AddressToLocation::addressToLocation => value,
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
    StringArrayToOne {
        type_: StringArrayToOne,
        source: StringArrayValue,
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
    AddIdentifier {
        type_: AddIdentifier,
        source: DataTypeLocation,
        destination: DataLocation,
    },
    IdentifierToObject {
        type_: IdentifierToObject,
        source: DataTypeLocation,
        destination: DataLocation,
    },
    ImageToIndividualDisplay {
        type_: ImageToIndividualDisplay,
        source: DataLocation,
        destination: DataLocation,
    },
    ImageToMediaObject {
        type_: ImageToMediaObject,
        source: DataLocation,
        destination: DataLocation,
    },
    TitleToSpecifiedByObject {
        type_: TitleToSpecifiedByObject,
        source: DataLocation,
        destination: DataLocation,
    },
    EqfToSpecifiedByQualification {
        type_: EqfToSpecifiedByQualification,
        source: DataLocation,
        destination: DataLocation,
    },
    LearningSettingToSpecifiedByObject {
        type_: LearningSettingToSpecifiedByObject,
        source: DataLocation,
        destination: DataLocation,
    },
    CreditToSpecifiedByObject {
        type_: CreditToSpecifiedByObject,
        source: DataLocation,
        destination: DataLocation,
    },
    AssessmentToProvenBy {
        type_: AssessmentToProvenBy,
        source: DataLocation,
        destination: DataLocation,
    },
    ObjectToNoteLiteral {
        type_: ObjectToNoteLiteral,
        source: DataLocation,
        destination: DataLocation,
    },
    TranslateLearningOutcome {
        type_: TranslateLearningOutcome,
        source: DataLocation,
        destination: DataLocation,
    },
    CreateLearningOutcomeSummary {
        type_: CreateLearningOutcomeSummary,
        source: DataLocation,
        destination: DataLocation,
    },
    AddressToLocation {
        type_: AddressToLocation,
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
pub struct DataTypeLocation {
    pub format: String,
    pub datatype: String,
    pub path: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StringValue {
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StringArrayValue {
    pub value: Vec<String>,
}
