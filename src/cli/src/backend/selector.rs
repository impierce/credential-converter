use crate::{backend::{jsonpointer::{JsonPath, JsonPointer}, repository::Repository, transformations::{DataLocation, OneToOne, Transformation}}, state::{AppState, Transformations}, trace_dbg};

pub fn selector(state: &mut AppState) {
    // This function shows the outcome of the transformation selected in the togglebar not the outcome of the Vec of selected transformations: state.selected_transformations.
    let transformation = state.selected_transformations[0];
    // trace_dbg!(transformation);

    let (input_format, output_format) = (state.mapping.input_format(), state.mapping.output_format());

    let (source_pointer, _source_value) = state.input_fields[state.selected_input_field].clone();

    //let output_pointer = state.missing_data_fields.as_ref().unwrap()[state.selected_missing_field].clone();
    let output_pointer = ("single".to_string(), "me".to_string());
    let destination_path: JsonPath = JsonPointer(output_pointer.0.clone()).into();

    // trace_dbg!(state.repository.clone());
    let mut temp_repository = Repository::from(state.repository.clone());
    // trace_dbg!(temp_repository.clone());

    let transformation = match transformation {
        Transformations::LowerCase => Transformation::OneToOne {
            type_: OneToOne::toLowerCase,
            source: DataLocation {
                format: input_format.clone(),
                path: JsonPath::try_from(JsonPointer(source_pointer)).unwrap().to_string(),
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
                path: JsonPath::try_from(JsonPointer(source_pointer)).unwrap().to_string(),
            },
            destination: DataLocation {
                format: output_format.clone(),
                path: destination_path.to_string(),
            },
        },
        Transformations::Copy | _ => Transformation::OneToOne {
            type_: OneToOne::copy,
            source: DataLocation {
                format: input_format.clone(),
                path: JsonPath::try_from(JsonPointer(source_pointer)).unwrap().to_string(),
            },
            destination: DataLocation {
                format: output_format.clone(),
                path: destination_path.to_string(),
            },
        },
    };

    temp_repository.apply_transformation(transformation);

    // trace_dbg!(output_format.clone());
    //let candidate_data_value = temp_repository.get(&output_format).unwrap().pointer(&output_pointer.0).unwrap();
    let candidate_data_value = "world";
    // trace_dbg!(candidate_data_value);
    state.candidate_data_value = Some(candidate_data_value.to_string());
    trace_dbg!("Selector done");
}
