#[derive(Debug)]
struct Task {
    id: i32,
    description: String,
    completed: bool,
}

fn add_task(description: &str) -> Task {
    let new_id = to_do_list.len() as i32 + 1;
    let new_task = Task {
        id: new_id,
        description: String::from(description),
        completed: false, // Default to false for uncompleted tasks
    };
    to_do_list.push(new_task.clone());
    new_task
}

fn complete_task(id: i32) -> Option<()> {
    if let Some(task) = to_do_list.iter_mut().find(|t| t.id == id) {
        task.completed = true;
        Some(())
    } else {
        None
    }
}

fn list_tasks() {
    for task in &to_do_list {
        println!("{:?}", task);
    }
}

fn main() {
    let mut to_do_list: Vec<Task> = Vec::new();

    let task1 = add_task("Buy groceries");
    let task2 = add_task("Write code");
    let task3 = add_task("Exercise");

    list_tasks();

    println!();

    complete_task(task2.id);

    list_tasks();
}

