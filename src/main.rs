use clap::Parser;
use serde::Serialize;
use std::collections::HashMap;
use std::io::Read;

#[derive(Serialize)]
enum Gender {
    MALE = 1,
    FEMALE = 2,
    UNSPECIFIED = 9,
}

#[derive(clap::ValueEnum, Debug, Clone)]
enum OutputFormat {
    Json,
    Yaml,
}

#[derive(Serialize)]
struct ParsedData {
    vehicle_class: String,
    driving_privileges: String,
    additional_privileges: String,
    expiration_date: String,
    last_name: String,
    first_name: String,
    middle_name: String,
    issue_date: String,
    date_of_birth: String,
    gender: Gender,
    eye_color: String,
    height: String,
    street: String,
    city: String,
    state: String,
    postal_code: String,
}

impl ParsedData {
    fn from_raw_data(raw_data: &str) -> Self {
        let mut parsed_data = ParsedData {
            vehicle_class: String::new(),
            driving_privileges: String::new(),
            additional_privileges: String::new(),
            expiration_date: String::new(),
            last_name: String::new(),
            first_name: String::new(),
            middle_name: String::new(),
            issue_date: String::new(),
            date_of_birth: String::new(),
            gender: Gender::UNSPECIFIED,
            eye_color: String::new(),
            height: String::new(),
            street: String::new(),
            city: String::new(),
            state: String::new(),
            postal_code: String::new(),
        };

        let mut data_map = HashMap::new();

        // Split the raw data into lines
        for line in raw_data.lines() {
            if line.len() >= 3 {
                let key = line[..3].to_string();
                let value = line[3..].trim().to_string(); // Get the value after the key

                data_map.insert(key, value);
            }
        }

        // Map values from data_map to parsed_data fields
        if let Some(value) = data_map.get("DCA") {
            parsed_data.vehicle_class = value.to_string();
        }
        if let Some(value) = data_map.get("DCB") {
            parsed_data.driving_privileges = value.to_string();
        }
        if let Some(value) = data_map.get("DCD") {
            parsed_data.additional_privileges = value.to_string();
        }
        if let Some(value) = data_map.get("DBA") {
            parsed_data.expiration_date = Self::standardize_date(value);
        }
        if let Some(value) = data_map.get("DCS") {
            parsed_data.last_name = value.to_string();
        }
        if let Some(value) = data_map.get("DAC") {
            parsed_data.first_name = value.to_string();
        }
        if let Some(value) = data_map.get("DAD") {
            parsed_data.middle_name = value.to_string();
        }
        if let Some(value) = data_map.get("DBD") {
            parsed_data.issue_date = Self::standardize_date(value);
        }
        if let Some(value) = data_map.get("DBB") {
            parsed_data.date_of_birth = Self::standardize_date(value);
        }
        if let Some(value) = data_map.get("DBC") {
            parsed_data.gender = match value.parse::<u8>().unwrap_or(9) {
                1 => Gender::MALE,
                2 => Gender::FEMALE,
                _ => Gender::UNSPECIFIED,
            };
        }
        if let Some(value) = data_map.get("DAY") {
            parsed_data.eye_color = value.to_string();
        }
        if let Some(value) = data_map.get("DAU") {
            parsed_data.height = Self::convert_height(value);
        }
        if let Some(value) = data_map.get("DAG") {
            parsed_data.street = value.to_string();
        }
        if let Some(value) = data_map.get("DAI") {
            parsed_data.city = value.to_string();
        }
        if let Some(value) = data_map.get("DAJ") {
            parsed_data.state = value.to_string();
        }
        if let Some(value) = data_map.get("DAK") {
            parsed_data.postal_code = value.to_string();
        }

        parsed_data
    }

    fn standardize_date(date_str: &str) -> String {
        if date_str.len() == 8 {
            // MMDDCCYY format (US)
            if let Ok(month) = date_str[..2].parse::<u32>() {
                if let Ok(day) = date_str[2..4].parse::<u32>() {
                    let year_str = &date_str[4..8];
                    let year = match year_str.parse::<u32>() {
                        Ok(y) => y,
                        Err(_) => return "Invalid Date".to_string(),
                    };

                    // Convert to ISO format (YYYY-MM-DD)
                    return format!("{:04}-{:02}-{:02}", year, month, day);
                }
            }
        } else if date_str.len() == 8 {
            // CCYYMMDD format (Canada)
            let year = &date_str[..4];
            let month = &date_str[4..6];
            let day = &date_str[6..8];

            // Validate and convert to ISO format (YYYY-MM-DD)
            return format!("{}-{}-{}", year, month, day);
        }

        "Invalid Date".to_string() // Fallback for invalid formats
    }

    fn convert_height(height_str: &str) -> String {
        if height_str.ends_with("CM") {
            if let Ok(cm) = height_str[..height_str.len() - 2].trim().parse::<f64>() {
                let inches = cm / 2.54; // Convert cm to inches
                return format!("{:.2}", inches); // Return only the number
            }
        } else if height_str.ends_with("IN") {
            if let Ok(inches) = height_str[..height_str.len() - 2].trim().parse::<f64>() {
                return format!("{:.2}", inches); // Return only the number
            }
        }
        "Invalid Height".to_string() // Fallback for invalid heights
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct CommandLineArguments {
    #[arg(short, long, value_name = "FILE")]
    /// Input file (defaults to stdin if not provided)
    file: Option<String>,

    #[arg(short = 'o', long, value_name = "FORMAT", default_value = "json")]
    /// Output format (defaults to json)
    format: OutputFormat,
}

fn main() {
    // Gather Command Line Arguments
    let args = CommandLineArguments::parse();
    let mut raw_data = String::new();

    match args.file {
        Some(file_path) => {
            // Try to open the file and read its contents
            let mut file = std::fs::File::open(&file_path)
                .map_err(|e| {
                    eprintln!("Error opening file '{}': {}", file_path, e);
                    e
                })
                .expect("Must Be Valid File");
            file.read_to_string(&mut raw_data)
                .map_err(|e| {
                    eprintln!("Error reading file '{}': {}", file_path, e);
                    e
                })
                .expect("Must Read To String");
        }
        None => {
            // If no file is provided, read from stdin
            std::io::stdin()
                .read_to_string(&mut raw_data)
                .expect("Must Be Able to Parse STDIN");
        }
    }

    // Interpret escape sequences (e.g., `\n`)
    let raw_data = raw_data.replace("\\n", "\n");

    let parsed_data = ParsedData::from_raw_data(&raw_data);

    // Serialize based on output format
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
