use super::*;


#[derive(Debug ,Serialize, Deserialize, PartialEq)]
pub enum NestedData {
    A,
    B,
    C 
}


#[derive(Debug ,Serialize, Deserialize, PartialEq, FastConfig)]
pub struct Data<T> {
    pub string: String,
    pub number: i32,
    pub unsigned: u64,
    pub boolean: bool,
    pub generic: T,
}

#[cfg(feature = "json")]
#[test]
fn create_save_change_save_load_json() {
    let c = MANAGER.setup();
    let path = c.path.join("config_generic.json");

    let mut config = Data {
        string: "test".into(),
        number: i32::MAX,
        unsigned: 0,
        boolean: true,
        generic: NestedData::A,
    };
    config.save(&path, JSON).unwrap();
    let loaded = Data::new(&path, JSON).unwrap();
    assert_eq!(loaded, config);

    config.number = i32::MIN;
    config.generic = NestedData::B;
    config.save(&path, JSON).unwrap();
    let updated = Data::new(&path, JSON).unwrap();
    config.load(&path, JSON).unwrap();
    assert_eq!(updated, config);
}

#[cfg(feature = "json5")]
#[test]
fn create_save_change_save_load_json5() {
    let c = MANAGER.setup();
    let path = c.path.join("config_generic.json5");
    let mut config = Data {
        string: "test".into(),
        number: i32::MAX,
        unsigned: 0,
        boolean: true,
        generic: NestedData::A,
    };
    config.save(&path, JSON5).unwrap();
    let loaded = Data::new(&path, JSON5).unwrap();
    assert_eq!(loaded, config);

    config.number = i32::MIN;
    config.generic = NestedData::B;
    config.save(&path, JSON5).unwrap();
    let updated = Data::new(&path, JSON5).unwrap();
    config.load(&path, JSON5).unwrap();
    assert_eq!(updated, config);
}

#[cfg(feature = "toml")]
#[test]
fn create_save_change_save_load_toml() {
    let c = MANAGER.setup();
    let path = c.path.join("config_generic.toml");
    let mut config = Data {
        string: "test".into(),
        number: i32::MAX,
        unsigned: 0,
        boolean: true,
        generic: NestedData::A,
    };
    config.save(&path, TOML).unwrap();
    let loaded = Data::new(&path, TOML).unwrap();
    assert_eq!(loaded, config);

    config.number = i32::MIN;
    config.generic = NestedData::B;
    config.save(&path, TOML).unwrap();
    let updated = Data::new(&path, TOML).unwrap();
    config.load(&path, TOML).unwrap();
    assert_eq!(updated, config);
}

#[cfg(feature = "yaml")]
#[test]
fn create_save_change_save_load_yaml() {
    let c = MANAGER.setup();
    let path = c.path.join("config_generic.yaml");
    let mut config = Data {
        string: "test".into(),
        number: i32::MAX,
        unsigned: 0,
        boolean: true,
        generic: NestedData::A,
    };
    config.save(&path, YAML).unwrap();
    let loaded = Data::new(&path, YAML).unwrap();
    assert_eq!(loaded, config);

    config.number = i32::MIN;
    config.generic = NestedData::B;
    config.save(&path, YAML).unwrap();
    let updated = Data::new(&path, YAML).unwrap();
    config.load(&path, YAML).unwrap();
    assert_eq!(updated, config);
}
