use std::{io::{self, Write}, path::Path};

#[tokio::main]
async fn main() {
    println!("\n\n~~~~~   Welcom to the 'OpenBadges-ELM Mapper' command line interface!   ~~~~~\n\n");

    let path_input  = prompt_input_path("Please enter the path to the JSON file you would like to map,\nFor relative paths, the current directory is ");
    let path_target = prompt_output_path("Please enter the path to the output file you would like to use for mapping");
    let path_format = prompt_option("Please enter the format of the input JSON file ", vec!["OBv3", "ELM"]);
    // Since we only have two options currently, we can assume target is the other format than the input.
    // This makes the next prompt unnecessary. Still debatable how to inform the user of this automatic deduction.
    // let target_format = prompt("Please enter the target format for the new JSON file [OBv3, ELM]");

    generate_json_paths();
}

//////////     HELPERS     //////////

fn prompt_input_path(question: &str) -> String {
    let valid_path = loop {
        print!("{}{}:\n", question, std::env::current_dir().unwrap().display());
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let input_trimmed = input.trim();
        if Path::new(input_trimmed).is_file() {
            break input_trimmed.to_string();
        } 
        else {
            println!("Invalid file path.");
        }
    };

    valid_path
}

fn prompt_output_path(question: &str) -> String {
    print!("{}:\n", question);
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let input_trimmed = input.trim();
    if Path::new(input_trimmed).is_file() {
        println!("File {} exists.\n! Warning: the file will be wiped and written to !", input_trimmed);
    } else {
        println!("File {} will be created and written to.", input_trimmed);
    }

    input_trimmed.to_string()
}

fn prompt_option(question: &str, options: Vec<&str>) -> String {
    let valid_option = loop {
        print!("{}[{}]:\n", question, options.join(", "));
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let input_trimmed = input.trim();
        if options.contains(&input_trimmed) {
            println!("Option {} selected.", input_trimmed);
            break input_trimmed.to_string();
        } 
        else {
            println!("Invalid option.");
        }
    };
    
    valid_option
}
