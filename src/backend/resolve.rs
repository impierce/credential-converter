use serde_json::Value;

pub fn value_to_str(value: &Value) -> String {
    match value {
        Value::String(s) => s.to_string(),
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Array(a) => {
            if a.is_empty() {
                return "[]".to_string();
            }
            else {
                let mut str_ = "[".to_string();
                for e in a {
                    str_ = str_ + value_to_str(e).as_str() + ", "; //again we don't how many levels nested array/objects
                }
                str_.truncate(str_.len() - 2);
                str_ = str_ + "]";
                str_.to_string()
            }
        }
        Value::Object(_) => "<Object>".to_string(),
        Value::Null => "".to_string(),
    }
}
