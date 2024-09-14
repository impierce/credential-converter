# ELM - OpenBadges v3 Mapper

## Overview
This project is a [Rust](https://www.rust-lang.org/)-based application that converts JSON files written in the [OpenBadges v3](https://www.imsglobal.org/spec/ob/v3p0) standard to the [ELM](https://europass.europa.eu/en/node/2128) standard and vice versa. The application utilizes the [ratatui](https://ratatui.rs/) and the [crossterm](https://docs.rs/crossterm/latest/crossterm/) library to create the terminal user interface in which the mappings can be completed.

## Features
- Bidirectional conversion between OpenBadges v3 and ELM. Extendable to any standard defined in JsonSchema.
- Supports importing mappings from DESM CSV files.
- Interactive Terminal Interface: select your input files and output paths, complete mappings and transformations.
- Support for custom mapping files: the program allows you to save all your mappings to a custom mapping file for future use.
- Headless/automated execution by calling the program with commandline arguments
- Automated batch conversion between formats taking a directory as input

## Installation
To run this project, ensure you have [Rust](https://www.rust-lang.org/) installed. If not, you can install it from rust-lang.org.

Clone the repository:

```sh
git clone https://github.com/impierce/credential-converter
cd credential-converter
```

Build the project:
```sh
cargo build
```

Run the application with:
```sh
cargo run
```
For headless/automated execution:

Run with -h (--help) for more information on what arguments to pass
```sh
cargo run -- -h
```

example:
```sh
cargo run -- -i example.json -o example_123.json -m example_mapping.json -c o-bv3-to-elm
```

Or find the executable in the `/target/debug` folder named after the repo name `credential-converter`.
```sh
./target/debug/credential-converter
```
```sh
./target/debug/credential-converter -i example.json -o example_123.json -m example_mapping.json -c o-bv3-to-elm
```



*Warning: the ratatui library does not seem to handle different color settings in your terminal perfectly. This causes the colors to differ slightly between builds in different terminals. For reference please continue reading the readme, colors will be explained accompanied by screenshots.*

## Usage

The link below contains a youtube video which is a walkthrough of the program.
The program has since been updated to have a folder like layout for the output fields, DESM and the headless interface, incl. batch conversions, are also not included in the demo. Nonetheless, the video gives a proper tour of the UX of the CLI.

https://www.youtube.com/watch?v=TGc2OyFQqXs  

For a written walkthrough, including screenshots, and explanation on the keyboard and mouse usage read below.

### Page 1, setting program arguments
Upon starting the application, you'll be presented with a terminal interface to select the input file, mapping file, output file paths and the conversion. You can also pick your preferred language and a file path to save your manual mappings to, for future use. 

All files/paths must be .json files/paths. The "Choose Mapping File" and the "Save Custom Mapping To" must be .json files created by or in the same structure as this program generates, otherwise deserialization will error. You can choose to leave these paths empty as well. When entering an existing custom mapping file into "Save Custom Mapping To" it will be extended if new mappings are performed. Enter "DESM" in the "Choose Mapping File" prompt if you wish to make use of their mappings. These mappings will automatically be saved to your "Custom Mapping" file, if you entered a valid path there.

Yellow highlights the active field. Green indicates a valid path. Orange means a given output path will wipe and overwrite an existing file in that location. Red means it's invalid, which disables you from continuing to the next page.
On the bottom you'll find a bar explaining the basic keys as well.


![credential-converter_P1.png](https://github.com/impierce/credential-converter/raw/main/.github/credential-converter_P1.png)

#### Default example files and logging
`res/elm_example.json`: Example input file for ELM standard.  
`res/mapping_empty.json`: An empty mapping file, useful for testing custom mappings.  
`res/output_credential.json`: Example output file for the converted JSON.  
`res/custom_mapping.json`: Example custom mapping file.  

Logs are kept in `logging_folder/credential-converter.log`. This file is overwritten upon each startup of the program.

To remove the default file paths remove lines 34 - 38 from the `main.rs`:
```sh
        // Default example values, remove if no longer needed
        state.input_path = "...".to_string();
        state.mapping_path = "...".to_string();
        state.output_path = "...".to_string();
        state.custom_mapping_path = "...".to_string();
```

#### Keyboard & mouse layout
Use the up and down arrows or tab and F2 to navigate between prompts.  
The left and right arrows enable you the choose within the prompts with tabbers.  
The normal keyboard layout can be used for entering file paths.  
All values are saved automatically.  
Enter can still be used as well to complete a prompt and to move to the next prompt.  
Clicking the complete button in the top right or enter on the last prompt will move you to the next page.  
Esc will prompt you to exit the program, losing all progress.

### Page 2, complete mandatory output fields
Page 2 is focused on the output file. Every standard has a minimum set of mandatory fields which need to be completed in order to render a valid Json file. These mandatory fields are listed on the right. The fields from the input file are listed on the left. On the bottom you will find the mapping bar, containing all mapping options.

Navigating through the output fields is done by hitting enter to open the folder or escape to go back. Only fields denoted by `<object>` open up like folders. Only the field denoted by `Your input >>` allows entering values. `Your input >>` fields are always accompanied by fields giving information about the input expected. Completed output fields and used input fields turn green upon completion. `<object>`'s turn green when they contain one or more completed fields, this doesn't mean all of it's fields have been completed.

The `View` button will open the mapping popup, allowing you to view the selected fields in full length in a popup, as the main page will truncate long texts. The tabs in the popup are scrollable. On the top left you will see the `Input Path` and below the `Input Value`. On the top right you will see the `Output Path` and below either the `Result Value` or the `Output Field Information`. The former displays the candidate value which is ready to be mapped by clicking `Confirm`. The latter displays the informational/non_input field content.Transformations will also be displayed immediately. Navigation on the main page is still possible when the popup is open.


The `Clear` button will close either the popup first if this is open. Then, it will clear all selected mapping options. Finally, if there is no popup nor mapping options selected, it will clear the result-value from the selected output field.
The `Complete` button in the top right will move you to the next page. If not all fields on the right are green, this will render a popup warning informing you the output file will be invalid.

`DirectCopy` does a direct copy from the selected input value to the selected output result-value (both yellow).
`Transformations` takes you to a new mapping bar allowing you to choose from multiple transformation options. `Lowercase` and `Uppercase` will transform the input value as such. `TakeIndex` takes exactly the index given, this can be an item from an array or a char from a string. `Slice` takes a slice from an array or a string from 0 to the given index. Chosen transformations will be shown on the right of the mapping bar, where they can be deleted from as well.  
*Currently `Regex`, `ManytoOne` and `OnetoMany` are not functional yet*.  


![credential-converter_P2.png](https://github.com/impierce/credential-converter/raw/main/.github/credential-converter_P2.png)


![credential-converter_P2_input_field.png](https://github.com/impierce/credential-converter/raw/main/.github/credential-converter_P2_input_field.png)

#### Keyboard & mouse layout
Use the arrows, tab and F2 to navigate inside and between tabs.  
Enter will select a field/transformation/button, sometimes moving you to the next tab.  
The complete button, view button and clear button are also clickable.  
Esc when the outfields tab is active is used to go back up the folder structure, the opposite of enter. Otherwise it works similar as the clear button as it closes the popup if open or clears the mapping options if selected. If none are selected and the popup is closed, it will prompt to exit the program as usual.

### Page 3, completing optional fields
Page 3 is completely optional, the mandatory part is finished in page 2.
In this page you can choose to complete optional output fields.
The layout and usage are the same as page 2.

On the right are now the optional output fields.  
On the left are the same fields from the input file with the used fields still in green.
This doesn't mean you can't use them again.

![credential-converter_P3.png](https://github.com/impierce/credential-converter/raw/main/.github/credential-converter_P3.png)

#### Keyboard & mouse layout
Use the arrows, tab and F2 to navigate inside and between tabs.  
Enter will select a field/transformation/button, sometimes moving you to the next tab.  
The complete button, view button and clear button are also clickable.  
Esc when the outfields tab is active is used to go back up the folder structure, the opposite of enter. Otherwise it works similar as the clear button as it closes the popup if open or clears the mapping options if selected. If none are selected and the popup is closed, it will prompt to exit the program as usual.

### Page 4, finished
You are finished, the mapping is done.  
This page only displays the output paths where you will find the results of your mapping.  
You can safely close the program using either Esc or clicking Complete. 

## Contributing
Feel free to fork this repository, submit issues, and make pull requests. Any contributions are welcome and appreciated.
