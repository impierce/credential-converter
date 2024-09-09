use regex::Regex;

use crate::{
    backend::{
        jsonpointer::{JsonPath, JsonPointer},
        transformations::{DataLocation, OneToOne, Transformation},
    },
    state::{AppState, Pages, Transformations},
    trace_dbg,
};

pub fn set_candidate_output_value(state: &mut AppState, push_transformation: bool) {
    // todo: is it needed to add directcopy to the start?
    let selected_transformations = [
        vec![Transformations::DirectCopy],
        state.selected_transformations.clone(),
    ]
    .concat();

    trace_dbg!(&state.selected_transformations);

    for transformation in selected_transformations.iter() {
        let transformation = define_transformation(state, *transformation);

        // todo: Can we just get the key pointed to instead of copynig the entire repository?
        let mut temp_repository = state.repository.clone();
        temp_repository.apply_transformation(transformation.clone(), state.mapping);

        let candidate_output_value = temp_repository
            .get(&state.mapping.output_format())
            .unwrap()
            .pointer(&state.output_pointer)
            .unwrap();

        state.candidate_output_value = Some(candidate_output_value.to_string());

        // if the candidate_output_value is only loaded for display in the popup, we don't want to push the transformation just yet.
        if push_transformation {
            state.performed_mappings.push(transformation);
        }
    }
}

pub fn define_transformation(state: &mut AppState, transformation: Transformations) -> Transformation {
    let (input_format, output_format) = (state.mapping.input_format(), state.mapping.output_format());
    let source_pointer: JsonPath = JsonPointer(state.input_fields[state.selected_input_field].0.clone()).into();

    let destination_path: JsonPath = JsonPointer(state.output_pointer.clone()).into();

    match transformation {
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
    }
}

pub fn set_output_pointer(state: &mut AppState) {
    let mut output_pointer = state.required_field_pointer.trim_start_matches("/required").to_string();
    if state.page == Pages::OptionalDataP3 {
        output_pointer = state.optional_field_pointer.trim_start_matches("/optional").to_string();
    }
    let regex = Regex::new(r"/(allOf|anyOf|oneOf|not)/\d+").unwrap();
    output_pointer = regex.replace_all(&output_pointer, "").to_string();

    state.output_pointer = output_pointer;
}
