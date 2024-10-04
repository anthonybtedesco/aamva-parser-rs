# aamva-parser
The purpose of this program is to parse data from North American ID cards following the specification set forth by AAMVA (American Association of Motor 1Vehicle Administrators)

| Data Ref. | Element ID | Data Element Definition                                                                                               | Card type | Length / type |
|-----------|------------|-----------------------------------------------------------------------------------------------------------------------|-----------|---------------|
| a         | DCA        | Jurisdiction-specific vehicle class / group code, designating the type of vehicle the cardholder has privilege to drive. | DL        | V6ANS         |
| b         | DCB        | Jurisdiction-specific codes that represent restrictions to driving privileges (such as airbrakes, automatic transmission, daylight only, etc.). | DL        | V12ANS        |
| c         | DCD        | Jurisdiction-specific codes that represent additional privileges granted to the cardholder beyond the vehicle class (such as transportation of passengers, hazardous materials, operation of motorcycles, etc.). | DL        | V5ANS         |
| d         | DBA        | Date on which the driving and identification privileges granted by the document are no longer valid. (MMDDCCYY for U.S., CCYYMMDD for Canada) | Both      | F8N           |
| e         | DCS        | Family name of the cardholder. (Family name is sometimes also called “last name” or “surname.”) Collect full name for record, print as many characters as possible on portrait side of DL/ID. | Both      | V40ANS        |
| f         | DAC        | First name of the cardholder.                                                                                         | Both      | V40ANS        |
| g         | DAD        | Middle name(s) of the cardholder. In the case of multiple middle names they shall be separated by a comma “,”.       | Both      | V40ANS        |
| h         | DBD        | Date on which the document was issued. (MMDDCCYY for U.S., CCYYMMDD for Canada)                                      | Both      | F8N           |
| i         | DBB        | Date on which the cardholder was born. (MMDDCCYY for U.S., CCYYMMDD for Canada)                                      | Both      | F8N           |
| j         | DBC        | Gender of the cardholder. 1 = male, 2 = female, 9 = not specified.                                                  | Both      | F1N           |
| k         | DAY        | Color of cardholder's eyes. (ANSI D-20 codes)                                                                        | Both      | F3A           |
| l         | DAU        | Height of cardholder. Inches (in): number of inches followed by " in" ex. 6'1'' = "073 in" Centimeters (cm): number of centimeters followed by " cm" ex. 181 centimeters="181 cm" | Both      | F6ANS         |
| m         | DAG        | Street portion of the cardholder address.                                                                             | Both      | V35ANS        |
| n         | DAI        | City portion of the cardholder address.                                                                               | Both      | V20ANS        |
| o         | DAJ        | State portion of the cardholder address.                                                                              | Both      | F2A           |
| p         | DAK        | Postal code portion of the cardholder address in the U.S. and Canada. If the trailing portion of the postal code in the U.S. is not known, zeros will be used to fill the trailing set of numbers up to nine (9) digits. | Both      | F11ANS        |


Given RAW pdf417 data the program will return either formatted json or yaml depending on a -o flag (default is JSON).
The default input for the program is stdin, however you can use the --file flag to load the data from a text file.

## Mapping from 3 Letter Code to Key. 
| Data Ref. | Element ID     | Data Element Definition                                                                                               |
|-----------|----------------|-----------------------------------------------------------------------------------------------------------------------|
| DCA       | vehicle_class   | Jurisdiction-specific vehicle class / group code, designating the type of vehicle the cardholder has privilege to drive. |
| DCB       | driving_privileges | Jurisdiction-specific codes that represent restrictions to driving privileges (such as airbrakes, automatic transmission, daylight only, etc.). |
| DCD       | additional_privileges | Jurisdiction-specific codes that represent additional privileges granted to the cardholder beyond the vehicle class (such as transportation of passengers, hazardous materials, operation of motorcycles, etc.). |
| DBA       | expiration_date | Date on which the driving and identification privileges granted by the document are no longer valid. (MMDDCCYY for U.S., CCYYMMDD for Canada) |
| DCS       | last_name      | Family name of the cardholder. (Family name is sometimes also called “last name” or “surname.”) Collect full name for record, print as many characters as possible on portrait side of DL/ID. |
| DAC       | first_name     | First name of the cardholder.                                                                                         |
| DAD       | middle_name     | Middle name(s) of the cardholder. In the case of multiple middle names they shall be separated by a comma “,”.       |
| DBD       | issue_date     | Date on which the document was issued. (MMDDCCYY for U.S., CCYYMMDD for Canada)                                      |
| DBB       | date_of_birth  | Date on which the cardholder was born. (MMDDCCYY for U.S., CCYYMMDD for Canada)                                      |
| DBC       | gender         | Gender of the cardholder. 1 = male, 2 = female, 9 = not specified.                                                  |
| DAY       | eye_color      | Color of cardholder's eyes. (ANSI D-20 codes)                                                                        |
| DAU       | height         | Height of cardholder. Inches (in): number of inches followed by " in" ex. 6'1'' = "073 in" Centimeters (cm): number of centimeters followed by " cm" ex. 181 centimeters="181 cm" |
| DAG       | street         | Street portion of the cardholder address.                                                                             |
| DAI       | city           | City portion of the cardholder address.                                                                               |
| DAJ       | state          | State portion of the cardholder address.                                                                              |
| DAK       | postal_code    | Postal code portion of the cardholder address in the U.S. and Canada. If the trailing portion of the postal code in the U.S. is not known, zeros will be used to fill the trailing set of numbers up to nine (9) digits. |
## Example Usage
- ./aamva-parser-rs --file raw-data.txt -o yaml
- cat raw-data.txt | ./aamva-parse-rs

The above command would parse the raw data from raw-data.txt and output a yaml.
