use crate::{ListId, TaskId};

pub trait TaskModel {
    fn create_task(&mut self) -> Result<TaskId, String>;

    fn set_task_name(&mut self, task_id: TaskId, name: &String) -> Result<(), String>;

    fn set_task_statement(&mut self, task_id: TaskId, statement: &String) -> Result<(), String>;

    fn set_task_completed(&mut self, task_id: TaskId, completed: bool) -> Result<(), String>;

    fn set_task_list(&mut self, task_id: TaskId, list_id: ListId) -> Result<(), String>;

    fn get_all_tasks(&self) -> Result<Vec<(ListId, TaskId)>, String>;

    fn get_task_name(&self, task_id: TaskId) -> Result<String, String>;

    fn get_task_statement(&self, task_id: TaskId) -> Result<String, String>;

    fn get_task_completed(&self, task_id: TaskId) -> Result<bool, String>;

    fn delete_task_list(&mut self, list_id: ListId) -> Result<(), String>;
}
