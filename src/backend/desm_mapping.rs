use std::fs::File;

use crate::{backend::transformations::Transformation, state::{AppState, Mapping}, trace_dbg};

use csv::ReaderBuilder;
use serde::Deserialize;

use super::transformations::{DataLocation, OneToOne};

/// Currently DESM only works at the Property level without considering the fields within a property.
/// This basically renders the mapping useless as for us only the field level really matters.
/// Another problem is that DESM mappings are path agnostic, but we need the specific paths.
pub fn apply_desm_mapping(state: &mut AppState) {
    let elm_spine_mapping: Vec<DesmCSVParsed> = desm_csv_parser("desm/assertion_csvs/Microcredential+Mapping_Assertion_ELM+Micro-Credential_20240208063519.csv");
    let obv3_spine_mapping: Vec<DesmCSVParsed> = desm_csv_parser("desm/assertion_csvs/Microcredential+Mapping_Assertion_Open+Badges+3.0_3.0_20240301181832.csv");
    
    let transformations: Vec<Transformation> = match state.mapping {
        Mapping::ELMToOBv3 => {
            build_transformations_from_csv_parsed(elm_spine_mapping, obv3_spine_mapping)
        },
        Mapping::OBv3ToELM => {
            build_transformations_from_csv_parsed(obv3_spine_mapping, elm_spine_mapping)
        }
    };
    
    trace_dbg!(&transformations);
    
    state.repository.apply_transformations(transformations, state.mapping);
}

pub fn desm_csv_parser(path: &str) -> Vec<DesmCSVParsed> {
    let mut ret: Vec<DesmCSVParsed> = Vec::new();

    let elm_rdr = File::open(path).expect("error: wrong path hardcoded in the backend");
    let mut elm_rdr = ReaderBuilder::new().has_headers(true).from_reader(elm_rdr);

    for line in elm_rdr.deserialize() {
        let result: Result<DesmCSVParsed, csv::Error> = line;
        if let Ok(mut entry) = result {
            // Todo: ugly hardcode, but can only develop this further in tandem with DESM when their output is more mature.
            if entry.mapped_schema == "ELM Micro-Credential".to_string() {
                entry.mapped_schema = "ELM".to_string();
            }
            else if entry.mapped_schema.starts_with("ob:") || entry.mapped_schema == "Open Badges 3.0 (3.0)" {
                entry.mapped_schema = "OBv3".to_string();
            }

            // We could add "Similar" here as well. Right now it doesnt mean much as DESM currently only maps Properties and doesnt go down to the field level.
            if &entry.mapping_predicate_label == "Identical" || &entry.mapping_predicate_label == "Reworded" {
                ret.push(entry);
            }
        }
    }

    ret
}

pub fn build_transformations_from_csv_parsed(source_csv_mapping: Vec<DesmCSVParsed>, output_csv_mapping: Vec<DesmCSVParsed>) -> Vec<Transformation> {
    let mut transformations = Vec::new();

    for src_e in source_csv_mapping {
        for outp_e in &output_csv_mapping {
            if src_e.spine_term_name == outp_e.spine_term_name {
                // Todo: please note that also this hardcoded pathbuilding with "$." is terrible, but as long as DESM remains path agnostic, we can only work with the assertion at the root level.
                transformations.push( Transformation::OneToOne {
                    type_: OneToOne::copy,
                    source: DataLocation { format: src_e.mapped_schema.clone(), path: "$.".to_owned() + to_camel_case(&src_e.mapped_term_name).as_str() },
                    destination: DataLocation {format: outp_e.mapped_schema.clone(), path: "$.".to_owned() + to_camel_case(&outp_e.mapped_term_name).as_str() }
                });
            }
        }
    }

    transformations
}

#[derive(Debug, Clone, Deserialize)]
pub struct DesmCSVParsed {
    #[serde(rename = "Spine term name")]
    spine_term_name: String,
    #[serde(rename = "Mapped term name")]
    mapped_term_name: String,
    #[serde(rename = "Mapping predicate label")]
    mapping_predicate_label: String,
    #[serde(rename = "Mapped term origin")]
    mapped_schema: String
}

// HELPER

fn to_camel_case(input: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = false;

    for c in input.chars() {
        if c.is_whitespace() {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(c.to_ascii_uppercase());
            capitalize_next = false;
        } else {
            result.push(c.to_ascii_lowercase());
        }
    }

    result
}
