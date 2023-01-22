use log::LevelFilter;
use crate::{Config, ConfigFormat, ConfigOptions};
use serde::{Serialize, Deserialize};
use strum::IntoEnumIterator;

fn run(format: ConfigFormat) {
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

    // Creating options
    let options = ConfigOptions {
        pretty: false,
        format,
        ..Default::default()
    };

    // Data defaults
    let data = MyData {
        number: 20,
        subdata: SubData {
            string:   format!("Joe Mama"),
            unsigned: 400,
            boolean:  true
        }
    };

    // Creating the config and saving it
    let mut config = Config::<MyData>::from_options("./config/testconfig", options, data);
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

#[test]
fn main() {
    env_logger::builder()
        .filter_level(LevelFilter::Info)
        .init();
    for format in ConfigFormat::iter() {
        if format != ConfigFormat::None {
            log::info!("Testing {format:?}..");
            run(format)
        }
    }
}
