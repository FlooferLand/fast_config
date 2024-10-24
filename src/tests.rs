#![allow(dead_code)]

use crate::{format_dependant, Config, ConfigSetupOptions};
use crate::{Deserialize, Serialize};
use log::LevelFilter;

// Sub-data
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct SubData {
    pub string: String,
    pub unsigned: u64,
    pub boolean: bool,
}

// Data
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct MyData {
    pub number: i32,
    pub subdata: SubData,
}
impl Default for MyData {
    fn default() -> Self {
        Self {
            number: 20,
            subdata: SubData {
                string: "Joe Mama".into(),
                unsigned: 400,
                boolean: true,
            },
        }
    }
}

#[test]
fn run() {
    // Logging
    let _ = env_logger::builder()
        .is_test(true)
        .filter_level(LevelFilter::Info)
        .try_init();

    // Creating options
    let options = ConfigSetupOptions {
        pretty: true,
        format: {
            // These test the format auto-picking
            // Chooses JSON by default when all features are enabled; in the normal library this would throw an error
            #[cfg(all(feature = "json", feature = "json5", feature = "toml", feature = "yaml"))] {
                Some(crate::ConfigFormat::JSON)
            }
            #[cfg(not(all(feature = "json", feature = "json5", feature = "toml", feature = "yaml")))] {
                None
            }
        },
        ..Default::default()
    };

    // Creating the config and saving it
    {
        let mut config =
            Config::from_options("./config/testconfig", options, MyData::default()).unwrap();
        config.data.number = i32::MAX;
        config.save().unwrap();
    }

    // Reading from that config + assertions
    {
        // Test data
        let data = MyData::default();
        let config = Config::from_options("./config/testconfig", options, data).unwrap();
        let default = MyData::default();
        assert_eq!(config.data.number, i32::MAX);
        assert_eq!(config.data.subdata.string, default.subdata.string);
        assert_eq!(config.data.subdata.unsigned, default.subdata.unsigned);
        assert_eq!(config.data.subdata.boolean, default.subdata.boolean);
    }

    // Advanced test
    if let Ok(value) = std::env::var("ADVANCED_TEST") {
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

// Called by `run` when ADVANCED_TEST env argument is enabled 
fn advanced_test() {
    #[derive(Debug)]
    pub enum FormatFinder {
        GuessExtension(String),
        Config(crate::ConfigFormat),
        Feature,
    }

    #[derive(Debug)]
    pub struct Case {
        pub format_finder: FormatFinder,
        pub pretty: bool,
    }

    impl Case {
        pub fn new(format_finder: FormatFinder, pretty: bool) -> Self {
            Self {
                format_finder,
                pretty,
            }
        }
    }

    // Adding all different possible cases
    // <!> Could probably be made slightly faster and cleaner by
    //     being moved into an array via a macro
    let available = format_dependant::get_enabled_features();
    let mut cases = Vec::with_capacity(
        2  /* how many `push` calls there are in the for loop below */
        * 2, /* `pretty` switches (for _ in 0..2) */
    );
    let mut pretty = false;
    for _ in 0..2 {
        for format in &available {
            // TODO: Add the GuessExtension test case back in (and update cases vec above accordingly)
            //       Currently this is bugged because of the new JSON/JSON5 extension guessing
            //cases.push(Case::new(
            //    FormatFinder::GuessExtension(format.to_string()),
            //    pretty,
            //));

            cases.push(Case::new(FormatFinder::Config(*format), pretty));
            #[cfg(not(all(feature = "json", feature = "json5", feature = "toml", feature = "yaml")))] {
                cases.push(Case::new(FormatFinder::Feature, pretty));
            }
        }
        pretty = !pretty;
    }

    // Automated case-based tests
    println!("######## Case test started! ########");
    println!("| All errors will now be split into sections |");
    for case in cases {
        let mut path = String::from("./config/advtestconfig");
        let mut format = None;
        match case.format_finder {
            FormatFinder::GuessExtension(ext) => {
                println!("\n\n------ GUESS EXTENSION ------ ");
                path += format!(".{ext}").as_str();
            }
            FormatFinder::Config(fmt) => {
                println!("\n\n---------- CONFIG ----------- ");
                format = Some(fmt);
            }
            FormatFinder::Feature => {
                println!("\n\n---------- FEATURE ---------- ");
                format = Some(format_dependant::get_first_enabled_feature());
            }
        };

        // Creating options
        let options = ConfigSetupOptions {
            pretty: case.pretty,
            format,
            ..Default::default()
        };

        // Creating the config and saving it
        {
            let config = Config::from_options(&path, options, MyData::default()).unwrap();
            config.save().unwrap();
        }

        // Reading from that config + assertions
        {
            // Test data
            let data = MyData::default();
            let config = Config::from_options(&path, options, data).unwrap();
            let default = MyData::default();
            assert_eq!(config.data.number, default.number);
            assert_eq!(config.data.subdata.string, default.subdata.string);
            assert_eq!(config.data.subdata.unsigned, default.subdata.unsigned);
            assert_eq!(config.data.subdata.boolean, default.subdata.boolean);
        }
    }
}

// what happens if no existing config file?
#[test]
fn no_create_if_missing() {
    const CONFIG_FILE_PATH: &str = "./config/testconfig";

    // Logging
    let _ = env_logger::builder()
        .is_test(true)
        .filter_level(LevelFilter::Info)
        .try_init();
    
    // Creating options
    let options = ConfigSetupOptions {
        pretty: true,
        format: {
            #[cfg(feature = "toml")] {
                Some(crate::ConfigFormat::TOML)
            }
            #[cfg(not(feature = "toml"))] {
                None
            }
        },
        ..Default::default()
    };

    let default = MyData::default();

    assert!(
        std::fs::metadata(CONFIG_FILE_PATH).is_err(),
        "precondition: no config file"
    );

    let config = Config::from_options(CONFIG_FILE_PATH, options, default).unwrap();

    assert_eq!(
        config.data,
        MyData::default(),
        "post_test: config == defaults"
    );

    let md = std::fs::metadata(CONFIG_FILE_PATH);
    log::error!("md is {:?}", md);
    assert!(md.is_err(), "should not have created config file");
}
