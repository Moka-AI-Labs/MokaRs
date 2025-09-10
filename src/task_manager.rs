use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: u32,
    pub name: String,
    pub completed: bool,
}

#[derive(Debug)]
pub struct TaskManager {
    tasks: HashMap<u32, Task>,
    next_id: u32,
}

impl TaskManager {
    pub fn new() -> Self {
        Self {
            tasks: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn add_task(&mut self, name: String) -> u32 {
        let id = self.next_id;
        let task = Task {
            id,
            name,
            completed: false,
        };
        self.tasks.insert(id, task);
        self.next_id += 1;
        id
    }

    pub fn complete_task(&mut self, id: u32) -> Result<(), String> {
        match self.tasks.get_mut(&id) {
            Some(task) => {
                task.completed = true;
                Ok(())
            }
            None => Err(format!("Task with id {} not found", id)),
        }
    }

    #[allow(dead_code)]
    pub fn get_task(&self, id: u32) -> Option<&Task> {
        self.tasks.get(&id)
    }

    pub fn list_tasks(&self) -> Vec<&Task> {
        self.tasks.values().collect()
    }
}

impl Default for TaskManager {
    fn default() -> Self {
        Self::new()
    }
}
