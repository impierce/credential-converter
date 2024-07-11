# ELM - OpenBadges v3 Mapper

## Overview
This project is a [Rust](https://www.rust-lang.org/)-based application that converts JSON files written in the [OpenBadges v3](https://www.imsglobal.org/spec/ob/v3p0) standard to the [ELM](https://europass.europa.eu/en/node/2128) standard and vice versa. The application utilizes the [ratatui](https://ratatui.rs/) and the [crossterm](https://docs.rs/crossterm/latest/crossterm/) library to create the terminal user interface in which the mappings can be completed.

## Features
- Bidirectional conversion between OpenBadges v3 and ELM. Extendable to any standard defined in JsonSchema.
- Interactive Terminal Interface: select your input files and output paths, complete mappings and transformations.
- Support for custom mapping files: the program allows you to save all your mappings to a custom mapping file for future use.
- Forward compatibility with DESM: In the future, DESM will provide professionally curated mapping files which will be shipped as default with the program.

## Installation
To run this project, ensure you have [Rust](https://www.rust-lang.org/) installed. If not, you can install it from rust-lang.org.

Clone the repository:

```sh
git clone https://github.com/impierce/impierce-mapper
cd impierce-mapper
```

Build the project:
```sh
cargo build
```

Run the application with:
```sh
cargo run
```

## Usage
Upon starting the application, you'll be presented with a terminal interface to select the input file, mapping file, and output file paths.

![impierce-mapper_P1.png](https://github.com/impierce/impierce-mapper/raw/HEAD/.github/impierce-mapper_P1.png)


### Default example files and logging
`res/elm_example.json`: Example input file for ELM standard.  
`res/mapping_empty.json`: An empty mapping file, useful for testing custom mappings.  
`res/output_credential.json`: Example output file for the converted JSON.  
`res/custom_mapping.json`: Example custom mapping file.  


Logs are kept in `logging_folder/impierce-mapper.log`. This file is overwritten upon each startup of the program.

To remove the default file paths remove lines 34 - 38 from the `main.rs`:
```sh
        // Default example values, remove if no longer needed
        input_path: "res/elm_example.json".to_string(),
        mapping_path: "res/mapping_empty.json".to_string(),
        output_path: "res/output_credential.json".to_string(),
        custom_mapping_path: "res/custom_mapping.json".to_string(),
```

## Keyboard & mouse layout
// keys and perhaps a screenshot here x

## Contributing
Feel free to fork this repository, submit issues, and make pull requests. Any contributions are welcome and appreciated.

## License
???
This project is licensed under the MIT License. See the LICENSE file for details.
