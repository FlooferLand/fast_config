#![allow(dead_code)]
pub use crate as fast_config;
pub use crate::FastConfig;
pub use crate::Format::*;

pub use serde::Deserialize;
pub use serde::Serialize;

pub use std::path::PathBuf;

mod associated;
mod generics;
mod nested;
mod simple;

struct Setup {
    path: PathBuf,
    manager: &'static Manager,
}

impl Drop for Setup {
    fn drop(&mut self) {
        if self
            .manager
            .0
            .fetch_sub(1, std::sync::atomic::Ordering::SeqCst)
            == 1
        {
            let _ = std::fs::remove_dir_all(&self.path)
                .inspect_err(|e| eprintln!("failed to clean up: {e}"));
        }
    }
}

struct Manager(std::sync::atomic::AtomicUsize);
impl Manager {
    fn setup(&'static self) -> Setup {
        self.0.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        Setup {
            path: PathBuf::from("../config/"),
            manager: self,
        }
    }
}

static MANAGER: Manager = Manager(std::sync::atomic::AtomicUsize::new(0));
