use super::*;

pub trait AssociatedTrait {
    type AssociatedType: Serialize + for<'a> Deserialize<'a> + PartialEq + Sized;
}

#[derive(PartialEq, Debug)]
pub enum ForigenData {
    A,
    B,
    C,
}

impl AssociatedTrait for ForigenData {
    type AssociatedType = f64;
}

#[derive(Debug, Serialize, Deserialize, PartialEq, FastConfig)]
pub struct Data<T: AssociatedTrait> {
    pub string: String,
    pub number: i32,
    pub unsigned: u64,
    pub boolean: bool,
    pub associated: T::AssociatedType,
}

#[cfg(feature = "json")]
#[test]
fn create_save_change_save_load_json() {
    let c = MANAGER.setup();
    let path = c.path.join("config_associated.json");

    let mut config = Data::<ForigenData> {
        string: "test".into(),
        number: i32::MAX,
        unsigned: 0,
        boolean: true,
        associated: 0.0,
    };
    config.save(&path, JSON).unwrap();
    let loaded = Data::new(&path, JSON).unwrap();
    assert_eq!(loaded, config);

    config.number = i32::MIN;
    config.associated = 1.0;
    config.save(&path, JSON).unwrap();
    let updated = Data::new(&path, JSON).unwrap();
    config.load(&path, JSON).unwrap();
    assert_eq!(updated, config);
}

#[cfg(feature = "json5")]
#[test]
fn create_save_change_save_load_json5() {
    let c = MANAGER.setup();
    let path = c.path.join("config_associated.json5");
    let mut config = Data::<ForigenData> {
        string: "test".into(),
        number: i32::MAX,
        unsigned: 0,
        boolean: true,
        associated: 0.0,
    };
    config.save(&path, JSON5).unwrap();
    let loaded = Data::new(&path, JSON5).unwrap();
    assert_eq!(loaded, config);

    config.number = i32::MIN;
    config.associated = 1.0;
    config.save(&path, JSON5).unwrap();
    let updated = Data::new(&path, JSON5).unwrap();
    config.load(&path, JSON5).unwrap();
    assert_eq!(updated, config);
}

#[cfg(feature = "toml")]
#[test]
fn create_save_change_save_load_toml() {
    let c = MANAGER.setup();
    let path = c.path.join("config_associated.toml");
    let mut config = Data::<ForigenData> {
        string: "test".into(),
        number: i32::MAX,
        unsigned: 0,
        boolean: true,
        associated: 0.0,
    };
    config.save(&path, TOML).unwrap();
    let loaded = Data::new(&path, TOML).unwrap();
    assert_eq!(loaded, config);

    config.number = i32::MIN;
    config.associated = 1.0;
    config.save(&path, TOML).unwrap();
    let updated = Data::new(&path, TOML).unwrap();
    config.load(&path, TOML).unwrap();
    assert_eq!(updated, config);
}

#[cfg(feature = "yaml")]
#[test]
fn create_save_change_save_load_yaml() {
    let c = MANAGER.setup();
    let path = c.path.join("config_associated.yaml");
    let mut config = Data::<ForigenData> {
        string: "test".into(),
        number: i32::MAX,
        unsigned: 0,
        boolean: true,
        associated: 0.0,
    };
    config.save(&path, YAML).unwrap();
    let loaded = Data::new(&path, YAML).unwrap();
    assert_eq!(loaded, config);

    config.number = i32::MIN;
    config.associated = 1.0;
    config.save(&path, YAML).unwrap();
    let updated = Data::new(&path, YAML).unwrap();
    config.load(&path, YAML).unwrap();
    assert_eq!(updated, config);
}
