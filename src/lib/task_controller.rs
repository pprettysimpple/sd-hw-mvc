use crate::{ListId, TaskId};

pub trait TaskController {
    fn create_new_undone_task(&mut self, task_name: String, task_statement: String, task_list: ListId) -> Result<(), String>;

    fn update_task_done(&mut self, task_id: TaskId) -> Result<(), String>;

    fn delete_task_list(&mut self, list_id: ListId) -> Result<(), String>;

    fn render_index(&self) -> Result<String, String>;
}
