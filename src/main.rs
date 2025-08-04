use std::{fs::File, io::Write};

use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { task_name: String },
}

#[derive(Serialize, Deserialize)]
struct Task {
    name: String,
}

fn get_current_task() -> Vec<Task> {
    let json_file = File::open("tasks.json");
    if let Ok(json_file) = json_file {
        let tasks = serde_json::from_reader::<_, Vec<Task>>(json_file);
        tasks.unwrap_or(Vec::new())
    } else {
        Vec::new()
    }
}

fn save_tasks(tasks: Vec<Task>) {
    let tasks_json = serde_json::to_string_pretty(&tasks).expect("Serialize to json");
    let mut file = File::create("tasks.json").expect("Tasks json file");
    file.write_all(tasks_json.as_bytes())
        .expect("Save tasks json");
}

fn add_task(task_name: String) {
    let mut current_tasks = get_current_task();
    let new_task = Task { name: task_name };
    current_tasks.push(new_task);
    save_tasks(current_tasks);
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { task_name } => add_task(task_name),
    }
}
