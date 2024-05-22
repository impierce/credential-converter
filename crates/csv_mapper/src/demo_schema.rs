use digital_credential_data_models::common::*;

#[derive(GenPaths, serde::Deserialize)]
pub struct LatLng {
    pub lat: f32,
    pub lng: f32,
}

#[derive(GenPaths, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    pub street: String,
    pub number: usize,
    pub province: Province,
    pub lat_lng: LatLng,
}

#[derive(GenPaths, serde::Deserialize)]
pub struct Province {
    pub name: String,
    pub code: String,
}

#[derive(GenPaths, Debug, serde::Deserialize)]
pub struct School {
    pub name: String,
    pub level: String,
}

#[derive(GenPaths, serde::Deserialize)]
pub struct Organisation {
    pub company_name: String,
    pub employees: Option<Person>,
    pub address: Address,
}

#[derive(GenPaths, serde::Deserialize)]
pub struct Agent {
    pub agent_name: String,
    pub hired_by: Option<Organisation>,
}

#[derive(GenPaths, serde::Deserialize)]
pub enum AgentOrPersonOrOrganization {
    Agent(Agent),
    Person(Person),
    Organisation(Organisation)
}

#[derive(GenPaths, serde::Deserialize)]
pub struct Person {
    pub name: String,

    #[serde(rename = "surName", default)]
    pub sur_name: Option<String>,
    pub age: usize,
    pub address: Address,
    pub synonyms: Vec<String>,
    pub schools: Option<OneOrMany<School>>,
}

#[derive(GenPaths, serde::Deserialize)]
pub struct DigitalCredential {
    pub credential_name: String,
    pub score: usize,
    pub issuer: AgentOrPersonOrOrganization,
}

