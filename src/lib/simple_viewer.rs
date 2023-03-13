use log::info;
use serde::Serialize;
use tinytemplate::TinyTemplate;
use crate::{ListId, Task, TaskId};
use crate::task_model::TaskModel;
use crate::task_viewer::TaskViewer;

#[derive(Serialize)]
struct RenderContext {
    tasks: Vec<(ListId, TaskId, Task)>,
}

pub struct SimpleViewer {
    data: String,
}

impl SimpleViewer {
    pub fn new(data: String) -> Self {
        SimpleViewer {
            data,
        }
    }
}

impl TaskViewer for SimpleViewer {
    fn get_all_tasks_html<T: TaskModel>(&self, model: &T) -> Result<String, String> {
        info!("GET ALL TASKS HTML");
        let mut template = TinyTemplate::new();
        info!("Empty template constructed");

        template
            .add_template(
                "easy_viewer",
                self.data.as_str(),
            )
            .expect("Failed to add template to renderer, possibly broken template");

        info!("Ok, created template");
        let tasks = model
            .get_all_tasks()?
            .into_iter()
            .filter_map(|(list_id, task_id)| {
                if let Ok(name) = model.get_task_name(task_id) {
                    if let Ok(statement) = model.get_task_statement(task_id) {
                        if let Ok(completed) = model.get_task_completed(task_id) {
                            return Some((list_id, task_id, Task {
                                name,
                                statement,
                                completed,
                            }));
                        }
                    }
                }
                None
            })
            .collect::<Vec<(ListId, TaskId, Task)>>();

        info!("Ok, got tasks");
        let context = RenderContext { tasks };

        Ok(template
            .render("easy_viewer", &context)
            .expect("Failed to render"))
    }
}