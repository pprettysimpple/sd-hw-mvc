use crate::task_controller::TaskController;
use crate::task_model::TaskModel;
use crate::task_viewer::TaskViewer;
use crate::{ListId, TaskId};

pub struct SimpleController<M, V>
    where M: TaskModel,
          V: TaskViewer {
    pub model: Box<M>,
    pub viewer: Box<V>,
}

impl<M, V> TaskController for SimpleController<M, V>
    where M: TaskModel,
          V: TaskViewer {
    fn create_new_undone_task(&mut self, task_name: String, task_statement: String, task_list: ListId) -> Result<(), String> {
        let task_id = self.model.create_task()?;
        self.model.set_task_name(task_id, &task_name)?;
        self.model.set_task_statement(task_id, &task_statement)?;
        self.model.set_task_completed(task_id, false)?;
        self.model.set_task_list(task_id, task_list)?;
        Ok(())
    }

    fn update_task_done(&mut self, task_id: TaskId) -> Result<(), String> {
        self.model.set_task_completed(task_id, true)
    }

    fn delete_task_list(&mut self, list_id: ListId) -> Result<(), String> {
        self.model.delete_task_list(list_id)
    }

    fn render_index(&self) -> Result<String, String> {

        self.viewer.get_all_tasks_html(self.model.as_ref())
    }
}