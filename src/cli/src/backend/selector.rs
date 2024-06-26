use crate::{
    backend::{
        jsonpointer::{JsonPath, JsonPointer},
        repository::Repository,
        transformations::{DataLocation, OneToOne, Transformation},
    },
    state::{AppState, Transformations},
    trace_dbg,
};

pub fn selector(state: &mut AppState) {
    // This function shows the outcome of the transformation selected in the togglebar not the outcome of the Vec of selected transformations: state.selected_transformations.
    let transformation = Transformations::from_repr(state.selected_transformation).unwrap();
    //trace_dbg!(transformation);

    let (input_format, output_format) = (state.mapping.input_format(), state.mapping.output_format());

    let (source_pointer, _source_value) = state.input_fields[state.selected_input_field].clone();

    let pointer = state.missing_data_fields[state.selected_missing_field].0.clone();
    let destination_path: JsonPath = JsonPointer(pointer.clone()).into();

    let mut temp_repository = Repository::from(state.repository.clone());

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
        Transformations::DirectCopy | _ => Transformation::OneToOne {
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

    //trace_dbg!(&pointer);
    let candidate_data_value = temp_repository.get(&output_format).unwrap().pointer(&pointer).unwrap();

    state.candidate_data_value = Some(candidate_data_value.to_string());
}
