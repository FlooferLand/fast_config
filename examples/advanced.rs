use fast_config::Config;
use fast_config::FastConfig;
use fast_config::Format;
use serde::Deserialize;
use serde::Serialize;

// Sub-structs
#[derive(Serialize, Deserialize)]
pub struct Person {
    pub name: String,
    pub age: u64,
    pub skill_issue: bool,
}

// Creating a config struct to store our data
#[derive(Serialize, Deserialize, FastConfig)]
pub struct MyData {
    pub student_debt: i32,
    pub person: Person,
}

// Setting the default values for the data
impl Default for MyData {
    fn default() -> Self {
        Self {
            student_debt: 20,
            person: Person {
                name: "Joe Mama".into(),
                age: 400,
                skill_issue: true,
            },
        }
    }
}

fn main() {
    let config_path = "./config/myconfig.json5";
    // Creating our data (default values)
    let mut data = MyData::default();

    // load the data from the file
    data.load(config_path, Format::JSON5).unwrap();

    // Read/writing to the data
    println!("I am ${} in debt", data.student_debt);
    data.student_debt = i32::MAX;
    println!("Oh no, i am now ${} in debt!!", data.student_debt);

    // Saving it back to the disk
    data.save(config_path, Format::JSON5).unwrap();
}
