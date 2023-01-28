use fast_config::{Config, ConfigSetupOptions};
use fast_config::error::{ConfigError, DataParseError};
use serde::{Serialize, Deserialize};

// Sub-structs
#[derive(Serialize, Deserialize)]
pub struct Person {
    pub name: String,
    pub age: u64,
    pub skill_issue: bool
}

// Creating a config struct to store our data
#[derive(Serialize, Deserialize)]
pub struct MyData {
    pub student_debt: i32,
    pub person: Person
}

// Setting the default values for the data
impl Default for MyData {
    fn default() -> Self {
        Self {
            student_debt: 20,
            person: Person {
                name: format!("Joe Mama"),
                age: 400,
                skill_issue: true
            }
        }
    }
}

fn main() {
    // Initializing a logging system (needed to show some warnings/errors)
    env_logger::init();

    // Creating options
    let options = ConfigSetupOptions {
        pretty: false,
        .. Default::default()
    };

    // Creating a new config struct with our data struct (it can also guess the file extension)
    let result = Config::from_options(
        "./config/myconfig",
        options,
        MyData::default()
    );

    // Error matching
    let mut config = match result {
        Ok(cfg) => {
            cfg
        }
        Err(error) => {
            match error {
                // Failed parsing the config
                ConfigError::DataParseError(parse_err) => {
                    match parse_err {
                        DataParseError::Serialize(format) =>
                            panic!("Failed to serialize format {format}!"),
                        DataParseError::Deserialize(format, _string) =>
                            panic!("Failed to deserialize format {format}!")
                    }
                }
                _ => panic!("Other error!")
            }
        }
    };

    // Read/writing to the data
    println!("I am ${} in debt", config.data.student_debt);
    config.data.student_debt = i32::MAX;
    println!("Oh no, i am now ${} in debt!!", config.data.student_debt);

    // Saving it back to the disk
    config.save().unwrap();
}