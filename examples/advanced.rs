use simple_config::{Config, ConfigOptions};
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
    #[serde(default = "MyDataDefaults::student_debt")]
    pub student_debt: i32,

    #[serde(default = "MyDataDefaults::person")]
    pub person: Person
}

// Storing the default values for our data
pub struct MyDataDefaults;
impl MyDataDefaults {
    pub fn student_debt() -> i32 { 20 }
    pub fn person() -> Person {
        Person {
            name: format!("Joe Mama"),
            age: 400,
            skill_issue: true
        }
    }
}

fn main() {
    // Initializing a logging system (needed to show errors)
    env_logger::init();

    // Creating options
    let options = ConfigOptions {
        pretty: false,
        ..Default::default()
    };

    // Creating a new config struct with our data struct (it can also guess the file extension)
    let mut config = Config::<MyData>::from_options("./config/compressed/myconfig", options);

    // Read/writing to the data
    println!("I am ${} in debt", config.data.student_debt);
    config.data.student_debt = i32::MAX;
    println!("Oh no, i am now ${} in debt!!", config.data.student_debt);

    // Saving it back to the disk
    config.save();
}