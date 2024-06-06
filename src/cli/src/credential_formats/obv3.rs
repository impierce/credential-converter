use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Person {
    full_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OBv3 {
    person: Person,
    another_field: String,
    another_field2: Test,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Test {
    test2: Test2,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Test2 {
    test3: String,
    test4: String,
    test5: Test5,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Test5 {
    test6: String,
}
