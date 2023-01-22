use log::LevelFilter;
use crate::{Config, ConfigOptions};
use serde::{Serialize, Deserialize};

#[test]
fn run() {
    // Logging
    env_logger::builder()
        .filter_level(LevelFilter::Info)
        .init();

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
        pub number: i32,
        pub subdata: SubData
    }

    impl Default for MyData {
        fn default() -> Self {
            Self {
                number: 20,
                subdata: SubData {
                    string:   format!("Joe Mama"),
                    unsigned: 400,
                    boolean:  true
                }
            }
        }
    }

    // Creating options
    let options = ConfigOptions {
        pretty: false,
        ..Default::default()
    };

    // Creating the config and saving it
    {
        let mut config = Config::<MyData>::from_options("./config/testconfig", options, MyData::default());
        config.data.number = i32::MAX;
        config.save();
    }

    // Reading from that config + assertions
    {
        // Test data
        let data = MyData {
            number: 0,
            subdata: SubData {
                string:   String::new(),
                unsigned: 0,
                boolean:  false
            }
        };
        let config = Config::<MyData>::new("./config/testconfig", data);
        let default = MyData::default();
        assert_eq!(config.data.number, i32::MAX);
        assert_eq!(config.data.subdata.string, default.subdata.string);
        assert_eq!(config.data.subdata.unsigned, default.subdata.unsigned);
        assert_eq!(config.data.subdata.boolean, default.subdata.boolean);
    }

    // Cleanup
    std::thread::sleep(std::time::Duration::from_millis(30));
    match std::fs::remove_dir_all("./config/") {
        Ok(_) => {}
        Err(e) => {
            log::error!("{e}");
        }
    }
}
