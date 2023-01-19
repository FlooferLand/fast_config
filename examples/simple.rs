use fast_config::Config;
use serde::{Serialize, Deserialize};

// Creating a config struct to store our data
#[derive(Serialize, Deserialize)]
pub struct MyData {
    #[serde(default = "MyDataDefaults::student_debt")]
    pub student_debt: i32
}

// Storing the default values for our data
pub struct MyDataDefaults;
impl MyDataDefaults {
    pub fn student_debt() -> i32 { 20 }
}

fn main() {
    // Initializing a logging system (needed to show errors)
    env_logger::init();

    // Creating a new config struct with our data struct (it can also guess the file extension)
    let mut config = Config::<MyData>::new("./config/myconfig");

    // Read/writing to the data
    println!("I am ${} in debt", config.data.student_debt);
    config.data.student_debt = i32::MAX;
    println!("Oh no, i am now ${} in debt!!", config.data.student_debt);

    // Saving it back to the disk
    config.save();
}