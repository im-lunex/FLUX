use crate::tasks::storage::get_all_tasks;

pub fn show_task_statistics(username: &str) {
    println!("\n--- Task Statistics ---");
    let tasks = get_all_tasks(username);

    if tasks.is_empty() {
        println!("(no tasks available)");
        return;
    }

    let total = tasks.len();
    let completed = tasks.iter().filter(|task| task.completed).count();
    let pending = total - completed;
    let completion_rate = if total > 0 {
        (completed as f64 / total as f64) * 100.0
    } else {
        0.0
    };

    println!("Total tasks: {}", total);
    println!("Completed: {}", completed);
    println!("Pending: {}", pending);
    println!("Completion rate: {:.1}%", completion_rate);
}
