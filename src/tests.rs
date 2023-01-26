use log::LevelFilter;
use crate::{Config, ConfigOptions, format_dependant};
use serde::{Serialize, Deserialize};

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

#[test]
fn run() {
    // Logging
    env_logger::builder()
        .filter_level(LevelFilter::Info)
        .init();

    // Creating options
    let options = ConfigOptions {
        pretty: true,
        ..Default::default()
    };

    // Creating the config and saving it
    {
        let mut config = Config::from_options("./config/testconfig", options, MyData::default()).unwrap();
        config.data.number = i32::MAX;
        config.save().unwrap();
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
        let config = Config::new("./config/testconfig", data).unwrap();
        let default = MyData::default();
        assert_eq!(config.data.number, i32::MAX);
        assert_eq!(config.data.subdata.string, default.subdata.string);
        assert_eq!(config.data.subdata.unsigned, default.subdata.unsigned);
        assert_eq!(config.data.subdata.boolean, default.subdata.boolean);
    }

    // Advanced test
    if let Ok(value) = std::env::var("advanced_test") {
        if !value.is_empty() {
            advanced_test();
        }
    }

    // Cleanup
    std::thread::sleep(std::time::Duration::from_millis(20));
    match std::fs::remove_dir_all("./config/") {
        Ok(_) => {}
        Err(e) => {
            log::error!("{e}");
        }
    }
}

// TODO: Advanced test code should be refactored as it looks incredibly messy.
//       Doesn't really matter due to the fact it's a test, though
fn advanced_test() {
    #[derive(Debug)]
    pub enum FormatFinder {
        GuessExtension(String),
        Config(crate::ConfigFormat),
        Feature
    }
    
    #[derive(Debug)]
    pub struct Case {
        pub format_finder: FormatFinder,
        pub pretty: bool
    }
    impl Case {
        pub fn new(format_finder: FormatFinder, pretty: bool) -> Self {
            Self { format_finder, pretty }
        }
    }

    // Adding all different possible cases
    // <!> Could probably be made slightly faster and cleaner by
    //     being moved into an array via a macro
    let available = format_dependant::get_enabled_features();
    let mut cases = Vec::with_capacity(
        3    /* `push` calls */
        * 2  /* `pretty` */
    );
    let mut pretty = false;
    for _ in 0..2 {
        for format in &available {
            cases.push(Case::new(
                FormatFinder::GuessExtension(format.to_string()),
                pretty
            ));
            cases.push(Case::new(
                FormatFinder::Config(format.clone()),
                pretty
            ));
            cases.push(Case::new(
                FormatFinder::Feature,
                pretty
            ));
        }
        pretty = !pretty;
    }

    // Automated case-based tests
    for case in cases {
        let mut path = String::from("./config/advtestconfig");
        let mut format = ConfigOptions::default().format;
        match case.format_finder {
            FormatFinder::GuessExtension(ext) => {
                path += format!(".{ext}").as_str();
            },
            FormatFinder::Config(fmt) => {
                format = fmt;
            },
            FormatFinder::Feature => {}
        };

        // Creating options
        let options = ConfigOptions {
            pretty: case.pretty,
            format,
            ..Default::default()
        };

        // Creating the config and saving it
        {
            let mut config = Config::from_options(&path, options, MyData::default()).unwrap();
            config.data.number = i32::MAX;
            config.save().unwrap();
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
            let config = Config::new(&path, data).unwrap();
            let default = MyData::default();
            assert_eq!(config.data.number, i32::MAX);
            assert_eq!(config.data.subdata.string, default.subdata.string);
            assert_eq!(config.data.subdata.unsigned, default.subdata.unsigned);
            assert_eq!(config.data.subdata.boolean, default.subdata.boolean);
        }
    }
}
