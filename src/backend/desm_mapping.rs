use std::fs::File;

use crate::{
    backend::transformations::Transformation,
    state::{AppState, Mapping},
    trace_dbg,
};

use csv::ReaderBuilder;
use serde::Deserialize;

use super::transformations::{DataLocation, DataLocations, Multiplicity, OneToOneType};

/// Currently DESM only works at the Property level without considering the fields within a property.
/// This basically renders the mapping useless as for us only the field level really matters.
/// Another problem is that DESM mappings are path agnostic, but we need the specific paths.
pub fn apply_desm_mapping(state: &mut AppState) {
    let elm_spine_mapping: Vec<DesmCSVParsed> = desm_csv_parser(
        "desm/assertion_csvs/Microcredential+Mapping_Assertion_ELM+Micro-Credential_20240208063519.csv",
        Format::ELM,
    );
    let obv3_spine_mapping: Vec<DesmCSVParsed> = desm_csv_parser(
        "desm/assertion_csvs/Microcredential+Mapping_Assertion_Open+Badges+3.0_3.0_20240301181832.csv",
        Format::OBv3,
    );
    let obv2_spine_mapping: Vec<DesmCSVParsed> = desm_csv_parser(
        "desm/assertion_csvs/Microcredential+Mapping_Assertion_Open+Badges+2.0_2.0_20240221174535.csv",
        Format::OBv2,
    );

    let transformations: Vec<Transformation> = match state.mapping {
        Mapping::ELMToOBv3 => build_transformations_from_csv_parsed(elm_spine_mapping, obv3_spine_mapping),
        Mapping::OBv3ToELM => build_transformations_from_csv_parsed(obv3_spine_mapping, elm_spine_mapping),
        Mapping::OBv2ToOBv3 => build_transformations_from_csv_parsed(obv2_spine_mapping, obv3_spine_mapping),
        Mapping::OBv3ToOBv2 => build_transformations_from_csv_parsed(obv3_spine_mapping, obv2_spine_mapping),
    };

    trace_dbg!(&transformations);

    state.performed_mappings.extend(transformations.clone());
    let mut completed_fields = state.repository.apply_transformations(transformations, state.mapping);
    for tuple in &mut completed_fields {
        tuple.0 = tuple.0.trim_start_matches('$').replace('.', "/");
        tuple.1 = tuple.1.trim_start_matches('$').replace('.', "/");
    }
    state.completed_fields.append(&mut completed_fields);

    trace_dbg!(&state.completed_fields);
}

pub fn desm_csv_parser(path: &str, format: Format) -> Vec<DesmCSVParsed> {
    let mut ret: Vec<DesmCSVParsed> = Vec::new();

    let rdr = File::open(path).expect("error: wrong path hardcoded in the backend");
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(rdr);

    for line in rdr.deserialize() {
        let result: Result<DesmCSVParsed, csv::Error> = line;
        if let Ok(mut entry) = result {
            if &entry.mapping_predicate_label == "Identical" || &entry.mapping_predicate_label == "Reworded" {
                match format {
                    Format::OBv3 => {
                        entry.mapped_schema = "OBv3".to_string();
                    }
                    Format::OBv2 => {
                        entry.mapped_schema = "OBv2".to_string();
                    }
                    Format::ELM => {
                        entry.mapped_schema = "ELM".to_string();
                    }
                }

                ret.push(entry);
            }
        }
    }

    ret
}

pub fn build_transformations_from_csv_parsed(
    source_csv_mapping: Vec<DesmCSVParsed>,
    output_csv_mapping: Vec<DesmCSVParsed>,
) -> Vec<Transformation> {
    let mut transformations = Vec::new();

    for src_e in source_csv_mapping {
        for outp_e in &output_csv_mapping {
            if src_e.spine_term_name == outp_e.spine_term_name {
                // Todo: please note that also this hardcoded pathbuilding with "$." is bad, but as long as DESM remains path agnostic, we can only work with the assertion at the root level.
                transformations.push(Transformation {
                    type_: Multiplicity::OneToOne(OneToOneType::copy),
                    source: DataLocations(vec![DataLocation {
                        format: src_e.mapped_schema.clone(),
                        path: "$.".to_owned() + to_camel_case(&src_e.mapped_term_name).as_str(),
                    }]),
                    destination: DataLocations(vec![DataLocation {
                        format: outp_e.mapped_schema.clone(),
                        path: "$.".to_owned() + to_camel_case(&outp_e.mapped_term_name).as_str(),
                    }]),
                });

                // Remove any duplicates or subsets which have been moved out of it's parent by adding a delete type to the transformation vec
                for i in 0..transformations.len() {
                    // Due to this first(), we only check the first path, thereby ManyToOne transformations are not handled properly.
                    for j in 0..transformations.len() {
                        if i == j {
                            continue;
                        }

                        // src_i will be the child, src_j the parent
                        let src_i = transformations[i].source.first_index();
                        let src_j = transformations[j].source.first_index();

                        // The second condition prefends looping over the newly added `delete` transformations.
                        if src_i.path.starts_with(&(src_j.path.clone() + "."))
                            && transformations[i].type_ != Multiplicity::OneToOne(OneToOneType::delete)
                        {
                            let destination_path = transformations[j].destination.first_index().path.clone()
                                + transformations[i]
                                    .destination
                                    .first_index()
                                    .path
                                    .clone()
                                    .trim_start_matches("$");

                            transformations.push(Transformation {
                                type_: Multiplicity::OneToOne(OneToOneType::delete),
                                source: DataLocations(vec![src_i.clone()]),
                                destination: DataLocations(vec![DataLocation {
                                    format: transformations[i].destination.first_index().format.clone(),
                                    path: destination_path,
                                }]),
                            });
                        }
                    }
                }
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
    mapped_schema: String, // todo: This field currently is used as an extra check to see if the mapping comes from the correct schema, need to discuss further with desm to see if this is really necessary.
}

// HELPER

fn to_camel_case(input: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = false;

    if input.contains(" ") {
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
    } else {
        input.to_string()
    }
}

/////////      STRUCTS     //////////

pub enum Format {
    OBv3,
    OBv2,
    #[allow(clippy::upper_case_acronyms)]
    ELM,
}
