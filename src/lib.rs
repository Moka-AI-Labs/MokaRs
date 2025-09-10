pub mod task_manager;

pub use task_manager::{Task, TaskManager};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_manager_creation() {
        let manager = TaskManager::new();
        assert_eq!(manager.list_tasks().len(), 0);
    }

    #[test]
    fn test_add_task() {
        let mut manager = TaskManager::new();
        let id = manager.add_task("Test task".to_string());

        assert_eq!(id, 1);
        assert_eq!(manager.list_tasks().len(), 1);

        let task = manager.get_task(id).unwrap();
        assert_eq!(task.name, "Test task");
        assert!(!task.completed);
    }

    #[test]
    fn test_complete_task() {
        let mut manager = TaskManager::new();
        let id = manager.add_task("Test task".to_string());

        let result = manager.complete_task(id);
        assert!(result.is_ok());

        let task = manager.get_task(id).unwrap();
        assert!(task.completed);
    }

    #[test]
    fn test_complete_nonexistent_task() {
        let mut manager = TaskManager::new();
        let result = manager.complete_task(999);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Task with id 999 not found");
    }

    #[test]
    fn test_multiple_tasks() {
        let mut manager = TaskManager::new();

        let id1 = manager.add_task("Task 1".to_string());
        let id2 = manager.add_task("Task 2".to_string());

        assert_eq!(id1, 1);
        assert_eq!(id2, 2);
        assert_eq!(manager.list_tasks().len(), 2);
    }

    #[tokio::test]
    async fn test_async_task_operations() {
        let mut manager = TaskManager::new();

        // Simulate async work
        tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;

        let id = manager.add_task("Async task".to_string());
        let task = manager.get_task(id).unwrap();
        assert_eq!(task.name, "Async task");
    }
}
