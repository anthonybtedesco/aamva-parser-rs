use std::fs::File;

enum Gender {
    MALE = 1,
    FEMALE = 2,
    UNSPECIFIED = 9,
}

struct ParsedData {
    vehicle_class: String,         // V12ANS
    driving_privileges: String,    // V12ANS
    additional_privileges: String, // V12ANS
    expiration_date: String,       // F8N (MMDDCCYY for U.S. CCYYMMDD for Canada)
    last_name: String,             // V40ANS
    first_name: String,            // V40ANS
    middle_name: String,           // V40ANS (multiple names seperated by commas)
    issue_date: String,            // F8N (MMDDCCYY for U.S. CCYYMMDD for Canada)
    date_of_birth: String,         // F8N (MMDDCCYY for U.S. CCYYMMDD for Canada)
    gender: Gender,                // F1N
    eye_color: String,             // F3A (ANSI D-20 codes)
    height: String,                // F6ANS (number followed by in or cm)
    street: String,                // V35ANS
    city: String,                  // V20ANS
    state: String,                 // F2A
    postal_code: String,           // F11ANS (9 digits)
}

impl ParsedData {}

struct CommandLineArguments {}

fn main() {
    // Gather Command Line Arguments
    let args: CommandLineArguments;
    // Get Raw Data from File or Stdin
    let raw_data: File;
    // Parse Data into Parsed Data Struct
    // Serialize Parsed Data into json or yaml depending on cli args
}
