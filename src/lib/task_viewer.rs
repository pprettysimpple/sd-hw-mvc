use crate::task_model::TaskModel;

pub trait TaskViewer {
    fn get_all_tasks_html<T: TaskModel>(&self, model: &T) -> Result<String, String>;
}
