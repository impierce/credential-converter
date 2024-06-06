use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Person {
    fname: String,
    lname: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ELM {
    person: Person,
    just_a_string: String,
}
