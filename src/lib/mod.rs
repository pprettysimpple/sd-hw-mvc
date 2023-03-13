extern crate core;

use serde::Serialize;

pub mod hash_map_model;
pub mod task_model;
pub mod task_controller;
pub mod task_viewer;
pub mod simple_viewer;
pub mod simple_controller;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Task {
    pub name: String,
    pub statement: String,
    pub completed: bool,
}

impl Default for Task {
    fn default() -> Self {
        Task {
            name: "".to_string(),
            statement: "".to_string(),
            completed: false
        }
    }
}

pub type TaskId = u64;

pub type ListId = String;