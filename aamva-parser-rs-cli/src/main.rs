use aamva_parser_rs::{
    serde_json, serde_yaml, CommandLineArguments, OutputFormat, ParsedData, Parser,
};
use std::io::Read;

fn main() {
    let args = CommandLineArguments::parse();
    let mut raw_data = String::new();

    match args.file {
        Some(file_path) => {
            let mut file = std::fs::File::open(&file_path).expect("Must Be Valid File");
            file.read_to_string(&mut raw_data)
                .expect("Must Read To String");
        }
        None => {
            std::io::stdin()
                .read_to_string(&mut raw_data)
                .expect("Must Be Able to Parse STDIN");
        }
    }

    let raw_data = raw_data.replace("\\n", "\n");

    let parsed_data = ParsedData::from_raw_data(&raw_data);

    match args.format {
        OutputFormat::Json => {
            let json_output =
                serde_json::to_string(&parsed_data).expect("Failed to serialize to JSON");
            println!("{}", json_output);
        }
        OutputFormat::Yaml => {
            let yaml_output =
                serde_yaml::to_string(&parsed_data).expect("Failed to serialize to YAML");
            println!("{}", yaml_output);
        }
    }
}
