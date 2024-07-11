# ELM - OpenBadges v3 Mapper

## Overview
This project is a Rust-based application that converts JSON files written in the OpenBadges v3 standard to the ELM standard and vice versa. The application utilizes the ratatui and crossterm library to create the terminal user interface in which the mappings can be completed.

## Features
- Bidirectional conversion between OpenBadges v3 and ELM. Extendable to any standard defined in JsonSchema.
- Interactive Terminal Interface: select your input files and output paths, complete mappings and transformations.
- Support for custom mapping files: the program allows you to save all your mappings to a custom mapping file for future use.
- Forward compatibility with DESM: In the future, DESM will provide professionally curated mapping files which will be shipped as default with the program.

## Installation
To run this project, ensure you have Rust installed. If not, you can install it from rust-lang.org.

Clone the repository:

```sh
git clone https://github.com/impierce/impierce-mapper
cd impierce-mapper
Build the project:
```

```sh
cargo build
Usage
Run the application with:
```

```sh
cargo run
```

Upon starting the application, you'll be presented with a terminal interface to select the input file, mapping file, and output file paths.

## Example files and logging
res/elm_example.json: Example input file for ELM standard.
res/mapping_empty.json: An empty mapping file, useful for testing custom mappings.
res/output_credential.json: Example output file for the converted JSON.
res/custom_mapping.json: Example custom mapping file.

Logs are kept in `logging_folder/impierce-mapper.log`. This file is overwritten upon each startup of the program.

## Keyboard & mouse layout
// keys and perhaps a screenshot here x

## Contributing
Feel free to fork this repository, submit issues, and make pull requests. Any contributions are welcome and appreciated.

## License
???
This project is licensed under the MIT License. See the LICENSE file for details.
