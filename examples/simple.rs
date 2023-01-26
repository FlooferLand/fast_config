use fast_config::Config;
use serde::{Serialize, Deserialize};

// Creating a config struct to store our data
#[derive(Serialize, Deserialize)]
pub struct MyData {
    pub student_debt: i32
}

fn main() {
    // Initializing a logging system (needed to show errors)
    env_logger::init();

    // Creating our data (default values)
    let data = MyData {
        student_debt: 20
    };

    // Creating a new config struct with our data struct (it can also guess the file extension)
    let mut config = Config::new("./config/myconfig", data).unwrap();

    // Read/writing to the data
    println!("I am ${} in debt", config.data.student_debt);
    config.data.student_debt = i32::MAX;
    println!("Oh no, i am now ${} in debt!!", config.data.student_debt);

    // Saving it back to the disk
    config.save().unwrap();
}