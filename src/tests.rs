use crate::{Config, ConfigOptions};
use serde::{Serialize, Deserialize};

#[test]
fn main() {
    // Sub-data
    #[derive(Serialize, Deserialize)]
    pub struct SubData {
        pub string: String,
        pub unsigned: u64,
        pub boolean: bool
    }

    // Data
    #[derive(Serialize, Deserialize)]
    pub struct MyData {
        #[serde(default = "MyDataDefaults::number")]
        pub number: i32,

        #[serde(default = "MyDataDefaults::subdata")]
        pub subdata: SubData
    }

    // Data defaults
    pub struct MyDataDefaults;
    impl MyDataDefaults {
        pub fn number() -> i32 { 20 }
        pub fn subdata() -> SubData {
            SubData {
                string:   format!("Joe Mama"),
                unsigned: 400,
                boolean:  true
            }
        }
    }

    // Logging system test
    env_logger::init();
    log::info!("Test started!");

    // Creating options
    let options = ConfigOptions {
        pretty: false,
        ..Default::default()
    };

    // Creating the config and saving it
    let mut config = Config::<MyData>::from_options("./config/testconfig", options);
    config.data.number = i32::MAX;
    config.save();

    // Cleanup
    std::thread::sleep(std::time::Duration::from_millis(30));
    match std::fs::remove_dir_all("./config/") {
        Ok(_) => {}
        Err(e) => {
            log::error!("{e}");
        }
    }
}