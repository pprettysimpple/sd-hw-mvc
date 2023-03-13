use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use crate::{ListId, Task, TaskId};
use crate::task_model::TaskModel;

#[derive(Debug, Clone)]
pub struct HashMapModel {
    task_storage: HashMap<TaskId, Task>,
    task_next_empty_id: u64,
    list_storage: HashMap<ListId, Vec<TaskId>>
}

impl Display for HashMapModel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("HashMapModel:\n")?;
        let output: String = self
            .task_storage
            .iter()
            .map(|(task_id, task)| format!("\tEntry: {} -> {:?}", task_id, task))
            .collect::<Vec<String>>()
            .join("\n");

        f.write_str(output.as_str())?;
        Ok(())
    }
}

impl HashMapModel {
    pub fn new() -> Self {
        HashMapModel {
            task_storage: HashMap::new(),
            task_next_empty_id: 0,
            list_storage: HashMap::new(),
        }
    }

    fn get_mut_task(&mut self, task_id: TaskId) -> Result<&mut Task, String> {
        match self.task_storage.get_mut(&task_id) {
            None => Err("No task for given task_id".to_string()),
            Some(task) => Ok(task),
        }
    }

    fn get_task(&self, task_id: TaskId) -> Result<&Task, String> {
        match self.task_storage.get(&task_id) {
            None => Err("No task for given task_id".to_string()),
            Some(task) => Ok(task),
        }
    }
}

impl TaskModel for HashMapModel {
    fn create_task(&mut self) -> Result<TaskId, String> {
        let new_task = Task::default();

        match self.task_storage.insert(self.task_next_empty_id, new_task) {
            None => {
                let result = self.task_next_empty_id;
                self.task_next_empty_id += 1;
                Ok(result)
            },
            Some(_) => Err("Failed to create new task, already exists".to_string()),
        }
    }

    fn set_task_name(&mut self, task_id: TaskId, name: &String) -> Result<(), String> {
        Ok(self.get_mut_task(task_id)?.name = name.to_string())
    }

    fn set_task_statement(&mut self, task_id: TaskId, statement: &String) -> Result<(), String> {
        Ok(self.get_mut_task(task_id)?.statement = statement.to_string())
    }

    fn set_task_completed(&mut self, task_id: TaskId, completed: bool) -> Result<(), String> {
        Ok(self.get_mut_task(task_id)?.completed = completed)
    }

    fn set_task_list(&mut self, task_id: TaskId, list_id: ListId) -> Result<(), String> {
        Ok(self
            .list_storage
            .entry(list_id)
            .or_insert(Vec::new())
            .push(task_id))
    }

    fn get_all_tasks(&self) -> Result<Vec<(ListId, TaskId)>, String> {
        Ok(self
            .list_storage
            .clone()
            .into_iter()
            .flat_map(|(list_id, task_ids)| {
                task_ids
                    .into_iter()
                    .map(move |task_id| (list_id.clone(), task_id))
            })
            .collect::<Vec<(ListId, TaskId)>>())
        // Ok((0..self.task_next_empty_id).collect())
    }

    fn get_task_name(&self, task_id: TaskId) -> Result<String, String> {
        Ok(self.get_task(task_id)?.name.clone())
    }

    fn get_task_statement(&self, task_id: TaskId) -> Result<String, String> {
        Ok(self.get_task(task_id)?.statement.clone())
    }

    fn get_task_completed(&self, task_id: TaskId) -> Result<bool, String> {
        Ok(self.get_task(task_id)?.completed)
    }

    fn delete_task_list(&mut self, list_id: ListId) -> Result<(), String> {
        Ok(self
            .list_storage
            .remove(list_id.as_str())
            .unwrap_or(Vec::new())
            .iter()
            .for_each(|task_id| { self.task_storage.remove(task_id); } ))
    }
}

#[cfg(test)]
// model test
mod test {
    use crate::hash_map_model::HashMapModel;
    use crate::Task;
    use crate::task_model::TaskModel;

    #[test]
    fn test_create_task() -> Result<(), String> {
        let mut model = HashMapModel::new();

        let task_id = model.create_task()?;
        let expected_task = Task::default();
        assert_eq!(model.task_storage.get(&task_id), Some(&expected_task));
        assert_eq!(model.get_all_tasks()?.len(), 1_usize);
        Ok(())
    }

    #[test]
    fn test_create_multiple_task() -> Result<(), String> {
        let expected_size = 100_usize;
        let mut model = HashMapModel::new();

        for task_id_outer in 0..expected_size {
            assert_eq!(model.create_task(), Ok(task_id_outer as u64));
        }

        assert_eq!(model.task_storage.len(), expected_size);
        assert_eq!(model.get_all_tasks()?.len(), expected_size);
        Ok(())
    }

    #[test]
    fn test_task_name() -> Result<(), String> {
        let mut model = HashMapModel::new();

        let task_id = model.create_task()?;

        let expected_task = Task::default();

        assert_eq!(model.task_storage.get(&task_id), Some(&expected_task));

        model.set_task_name(task_id, &"Some task name".to_string())?;

        let expected_task2 = Task {
            name: "Some task name".to_string(),
            statement: "".to_string(),
            completed: false
        };

        assert_eq!(model.task_storage.get(&task_id), Some(&expected_task2));
        assert_eq!(model.get_task_name(task_id), Ok("Some task name".to_string()));

        Ok(())
    }

    #[test]
    fn test_task_statement() -> Result<(), String> {
        let mut model = HashMapModel::new();

        let task_id = model.create_task()?;

        let expected_task = Task::default();

        assert_eq!(model.task_storage.get(&task_id), Some(&expected_task));

        model.set_task_statement(task_id, &"Some task statement".to_string())?;

        let expected_task2 = Task {
            name: "".to_string(),
            statement: "Some task statement".to_string(),
            completed: false
        };

        assert_eq!(model.task_storage.get(&task_id), Some(&expected_task2));
        assert_eq!(model.get_task_statement(task_id), Ok("Some task statement".to_string()));

        Ok(())
    }

    #[test]
    fn test_task_completed() -> Result<(), String> {
        let mut model = HashMapModel::new();

        let task_id = model.create_task()?;

        let expected_task = Task::default();

        assert_eq!(model.task_storage.get(&task_id), Some(&expected_task));

        model.set_task_completed(task_id, true)?;

        let expected_task2 = Task {
            name: "".to_string(),
            statement: "".to_string(),
            completed: true
        };

        assert_eq!(model.task_storage.get(&task_id), Some(&expected_task2));
        assert_eq!(model.get_task_completed(task_id), Ok(true));

        Ok(())
    }

    #[test]
    fn test_task_multiple_fields() -> Result<(), String> {
        let mut model = HashMapModel::new();

        let task_id = model.create_task()?;

        let expected_task = Task::default();

        assert_eq!(model.task_storage.get(&task_id), Some(&expected_task));

        model.set_task_name(task_id, &"Some task name".to_string())?;
        model.set_task_statement(task_id, &"Some task statement".to_string())?;
        model.set_task_completed(task_id, true)?;

        let expected_task2 = Task {
            name: "Some task name".to_string(),
            statement: "Some task statement".to_string(),
            completed: true
        };

        assert_eq!(model.task_storage.get(&task_id), Some(&expected_task2));

        assert_eq!(model.get_task_name(task_id), Ok("Some task name".to_string()));
        assert_eq!(model.get_task_statement(task_id), Ok("Some task statement".to_string()));
        assert_eq!(model.get_task_completed(task_id), Ok(true));

        Ok(())
    }
}