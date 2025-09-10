mod task_manager;
use task_manager::TaskManager;

#[tokio::main]
async fn main() {
    println!("Welcome to MokaRs - A Task Management System!");

    let mut manager = TaskManager::new();

    // Add some sample tasks
    let task1_id = manager.add_task("Design code pipeline".to_string());
    let _task2_id = manager.add_task("Implement CI/CD".to_string());
    let _task3_id = manager.add_task("Add documentation".to_string());

    println!("Created tasks:");
    for task in manager.list_tasks() {
        println!(
            "  {} - {} ({})",
            task.id,
            task.name,
            if task.completed { "✓" } else { "○" }
        );
    }

    // Complete first task
    manager
        .complete_task(task1_id)
        .expect("Failed to complete task");

    println!("\nAfter completing task {}:", task1_id);
    for task in manager.list_tasks() {
        println!(
            "  {} - {} ({})",
            task.id,
            task.name,
            if task.completed { "✓" } else { "○" }
        );
    }

    println!("\nMokaRs pipeline demonstration complete!");
}
