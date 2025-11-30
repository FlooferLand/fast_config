#![allow(dead_code)]
use crate as fast_config;
use crate::FastConfig;
use crate::Format::*;

use serde::Deserialize;
use serde::Serialize;

use std::path::PathBuf;
// Sub-data
#[derive(Default, Debug ,Serialize, Deserialize, PartialEq)]
pub struct SubData {
    pub string: String,
    pub unsigned: u64,
    pub boolean: bool,
}

// GenericData
#[derive(Serialize, Deserialize, PartialEq, Debug, FastConfig)]
pub struct MyData<T: MyTrait> {
    pub number: T::AssociatedType,
    pub subdata: T,
}
impl<T: Default + MyTrait> Default for MyData<T> {
    fn default() -> Self {
        Self {
            number: <T as MyTrait>::AssociatedType::default(),
            subdata: T::default(),
        }
    }
}

pub trait MyTrait {
    type AssociatedType: Serialize + for<'a> Deserialize<'a> + PartialEq + std::fmt::Debug + Default;
}

impl MyTrait for SubData {
    type AssociatedType = i32;
}

struct Setup<'a> {
    path: PathBuf,
    manager: &'a Manager,
}

impl<'a> Drop for Setup<'a> {
    fn drop(&mut self) {
        if self
            .manager
            .0
            .fetch_sub(1, std::sync::atomic::Ordering::SeqCst)
            == 1
        {
            std::fs::remove_dir_all(&self.path).expect("Failed to remove test directory");
        }
    }
}

struct Manager(std::sync::atomic::AtomicUsize);
impl Manager {
    fn setup<'a>(&'a self) -> Setup<'a> {
        self.0.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        Setup {
            path: PathBuf::from("../config/"),
            manager: self,
        }
    }
}

static MANAGER: Manager = Manager(std::sync::atomic::AtomicUsize::new(0));

#[cfg(feature = "json")]
#[test]
fn save_load_json() {
    let c = MANAGER.setup();
    let path = c.path.join("config.json");
    let mut to_save = MyData::<SubData>::default();
    to_save.number = i32::MAX;
    to_save.save(&path, JSON).unwrap();

    let mut to_load = MyData::<SubData>::default();
    to_load.load(&path, JSON).unwrap();

    assert_eq!(to_load, to_save);
}

#[cfg(feature = "json5")]
#[test]
fn save_load_json5() {
    let c = MANAGER.setup();
    let path = c.path.join("config.json5");
    let mut to_save = MyData::<SubData>::default();
    to_save.number = i32::MAX;
    to_save.save(&path, JSON5).unwrap();

    let mut to_load = MyData::<SubData>::default();
    to_load.load(&path, JSON5).unwrap();

    assert_eq!(to_load, to_save);
}

#[cfg(feature = "toml")]
#[test]
fn save_load_toml() {
    let c = MANAGER.setup();
    let path = c.path.join("config.toml");
    let mut to_save = MyData::<SubData>::default();
    to_save.number = i32::MAX;
    to_save.save(&path, TOML).unwrap();

    let mut to_load = MyData::<SubData>::default();
    to_load.load(&path, TOML).unwrap();

    assert_eq!(to_load, to_save);
}

#[cfg(feature = "yaml")]
#[test]
fn save_load_yaml() {
    let c = MANAGER.setup();
    let path = c.path.join("config.yaml");
    let mut to_save = MyData::<SubData>::default();
    to_save.number = i32::MAX;
    to_save.save(&path, YAML).unwrap();

    let mut to_load = MyData::<SubData>::default();
    to_load.load(&path, YAML).unwrap();

    assert_eq!(to_load, to_save);
}
