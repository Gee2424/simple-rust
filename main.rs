mod task;
mod store;

use std::env;
use task::Task;
use store::{load_tasks, save_tasks};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please provide a command: add, list, complete");
        return Ok(());
    }

    match args[1].as_str() {
        "add" => {
            if args.len() < 3 {
                println!("Please provide a task description");
                return Ok(());
            }
            add_task(&args[2..].join(" "))?;
        },
        "list" => list_tasks()?,
        "complete" => {
            if args.len() < 3 {
                println!("Please provide a task index to mark as complete");
                return Ok(());
            }
            complete_task(args[2].parse().unwrap())?;
        },
        _ => println!("Unknown command. Available commands: add, list, complete"),
    }

    Ok(())
}

fn add_task(description: &str) -> std::io::Result<()> {
    let mut tasks = load_tasks()?;
    let task = Task::new(description);
    tasks.push(task);
    save_tasks(&tasks)?;
    println!("Task added.");
    Ok(())
}

fn list_tasks() -> std::io::Result<()> {
    let tasks = load_tasks()?;
    for (i, task) in tasks.iter().enumerate() {
        println!("{} - {} [{}]", i, task.description, if task.completed { "Done" } else { "Pending" });
    }
    Ok(())
}

fn complete_task(index: usize) -> std::io::Result<()> {
    let mut tasks = load_tasks()?;
    if index >= tasks.len() {
        println!("Invalid index.");
        return Ok(());
    }
    tasks[index].mark_completed();
    save_tasks(&tasks)?;
    println!("Task marked as complete.");
    Ok(())
}
