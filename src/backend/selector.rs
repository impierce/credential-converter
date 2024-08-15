use regex::Regex;

use crate::{
    backend::{
        jsonpointer::{JsonPath, JsonPointer},
        transformations::{DataLocation, OneToOne, Transformation},
    },
    state::{AppState, Pages, Transformations},
};

pub fn selector(state: &mut AppState) {
    let selected_transformations = [
        vec![Transformations::DirectCopy],
        state.selected_transformations.clone(),
    ]
    .concat();
    // This function shows the outcome of the transformation selected in the togglebar not the outcome of the Vec of selected transformations: state.selected_transformations.
    for transformation in selected_transformations.iter() {
        // trace_dbg!(transformation);

        let (input_format, output_format) = (state.mapping.input_format(), state.mapping.output_format());

        let (source_pointer, _source_value) = state.input_fields[state.selected_input_field].clone();

        let mut output_pointer = state.missing_field_pointer.trim_start_matches("/required");
        if state.page == Pages::UnusedDataP3 {
            output_pointer = state.optional_field_pointer.trim_start_matches("/optional");
        }
        let destination_path: JsonPath = JsonPointer(output_pointer.to_string()).into();

        let mut temp_repository = state.repository.clone();

        let transformation = match transformation {
            Transformations::LowerCase => Transformation::OneToOne {
                type_: OneToOne::toLowerCase,
                source: DataLocation {
                    format: input_format.clone(),
                    path: JsonPath::from(JsonPointer(source_pointer)).to_string(),
                },
                destination: DataLocation {
                    format: output_format.clone(),
                    path: destination_path.to_string(),
                },
            },
            Transformations::UpperCase => Transformation::OneToOne {
                type_: OneToOne::toUpperCase,
                source: DataLocation {
                    format: input_format.clone(),
                    path: JsonPath::from(JsonPointer(source_pointer)).to_string(),
                },
                destination: DataLocation {
                    format: output_format.clone(),
                    path: destination_path.to_string(),
                },
            },
            // todo: This clippy warning is known, this body is for 'DirectCopy' and all others until they
            // get their own branches
            Transformations::DirectCopy | _ => Transformation::OneToOne {
                type_: OneToOne::copy,
                source: DataLocation {
                    format: input_format.clone(),
                    path: JsonPath::from(JsonPointer(source_pointer)).to_string(),
                },
                destination: DataLocation {
                    format: output_format.clone(),
                    path: destination_path.to_string(),
                },
            },
        };

        temp_repository.apply_transformation(transformation.clone(), state.mapping);
        state.mappings.push(transformation);

        // trace_dbg!(&output_pointer);
        let candidate_data_value = temp_repository
            .get(&output_format)
            .unwrap()
            .pointer(output_pointer)
            .unwrap();

        state.candidate_data_value = Some(candidate_data_value.to_string());
    }
}

pub fn input_to_output(state: &mut AppState) { // todo: move all selector functionality to input_to_output
    let selected_transformations = [
        vec![Transformations::DirectCopy],
        state.selected_transformations.clone(),
    ]
    .concat();

    for transformation in selected_transformations.iter() {
        //
        let (input_format, output_format) = (state.mapping.input_format(), state.mapping.output_format());
        let source_pointer: JsonPath = JsonPointer(state.input_fields[state.selected_input_field].0.clone()).into();
        let mut output_pointer = state.missing_field_pointer.trim_start_matches("/required").to_string(); // todo: remove subset paths segment is double code, also in selector
        if state.page == Pages::UnusedDataP3 {
            output_pointer = state.optional_field_pointer.trim_start_matches("/optional").to_string();
        }
        if output_pointer.contains("/allOf")
            || output_pointer.contains("/anyOf")
            || output_pointer.contains("/oneOf")
            || output_pointer.contains("/not")
        {
            let re_allof = Regex::new(r"allOf/.*/").unwrap();
            let re_anyof = Regex::new(r"anyOf/.*/").unwrap();
            let re_oneof = Regex::new(r"oneOf/.*/").unwrap();
            let re_not = Regex::new(r"not/.*/").unwrap();

            output_pointer = re_allof.replace_all(&output_pointer, "").to_string();
            output_pointer = re_anyof.replace_all(&output_pointer, "").to_string();
            output_pointer = re_oneof.replace_all(&output_pointer, "").to_string();
            output_pointer = re_not.replace_all(&output_pointer, "").to_string();
        }

        let destination_path: JsonPath = JsonPointer(output_pointer.clone()).into();

        let transformation = match transformation {
            Transformations::LowerCase => Transformation::OneToOne {
                type_: OneToOne::toLowerCase,
                source: DataLocation {
                    format: input_format.clone(),
                    path: source_pointer.to_string(),
                },
                destination: DataLocation {
                    format: output_format.clone(),
                    path: destination_path.to_string(),
                },
            },
            Transformations::UpperCase => Transformation::OneToOne {
                type_: OneToOne::toUpperCase,
                source: DataLocation {
                    format: input_format.clone(),
                    path: source_pointer.to_string(),
                },
                destination: DataLocation {
                    format: output_format.clone(),
                    path: destination_path.to_string(),
                },
            },
            // todo: This clippy warning is known, this body is for 'DirectCopy' and all others until they
            // get their own branches
            Transformations::DirectCopy | _ => Transformation::OneToOne {
                type_: OneToOne::copy,
                source: DataLocation {
                    format: input_format.clone(),
                    path: source_pointer.to_string(),
                },
                destination: DataLocation {
                    format: output_format.clone(),
                    path: destination_path.to_string(),
                },
            },
        };

        let mut temp_repository = state.repository.clone();
        temp_repository.apply_transformation(transformation.clone(), state.mapping);
        state.mappings.push(transformation);

        //trace_dbg!(&pointer);
        let candidate_data_value = temp_repository
            .get(&output_format)
            .unwrap()
            .pointer(&output_pointer)
            .unwrap();

        state.candidate_data_value = Some(candidate_data_value.to_string());
    }
}
