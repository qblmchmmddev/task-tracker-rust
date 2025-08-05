use std::{fmt::Display, fs::File, io::Write};

use chrono::{DateTime, Utc};
use clap::{Parser, Subcommand, ValueEnum};
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
    List { status: Option<TaskStatus> },
    Mark { id: u64, status: TaskStatus },
}

#[derive(Serialize, Deserialize, Default)]
struct TaskData {
    tasks: Vec<Task>,
    id_counter: u64,
}

#[derive(Serialize, Deserialize)]
struct Task {
    id: u64,
    name: String,
    status: TaskStatus,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(PartialEq, Serialize, Deserialize, Clone, ValueEnum)]
enum TaskStatus {
    Todo,
    InProgress,
    Done,
}

impl Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Todo => "Todo",
                Self::InProgress => "InProgress",
                Self::Done => "Done",
            }
        )
    }
}

fn get_current_task_data() -> TaskData {
    let json_file = File::open("tasks.json");
    if let Ok(json_file) = json_file {
        let task_data = serde_json::from_reader::<_, TaskData>(json_file);
        task_data.unwrap_or_default()
    } else {
        TaskData::default()
    }
}

fn save_task_data(task_data: TaskData) {
    let task_data_json = serde_json::to_string_pretty(&task_data).expect("Serialize to json");
    let mut file = File::create("tasks.json").expect("Tasks json file");
    file.write_all(task_data_json.as_bytes())
        .expect("Save tasks json");
}

fn add_task(task_name: String) {
    let mut task_data = get_current_task_data();
    let id = task_data.id_counter;
    let status = TaskStatus::Todo;
    let now = Utc::now();
    let new_task = Task {
        id,
        name: task_name,
        status,
        created_at: now,
        updated_at: now,
    };
    task_data.tasks.push(new_task);
    task_data.id_counter += 1;
    save_task_data(task_data);
    println!("Task added successfully (ID: {})", id)
}

fn list_task(status: Option<TaskStatus>) {
    let current_task_data = get_current_task_data();
    let print_task = |t: &Task| {
        let now = Utc::now();
        let task_age = t.created_at - now;
        let task_age_humanize = chrono_humanize::HumanTime::from(task_age);
        println!(
            "[{}] #{} {} ({})",
            t.status, t.id, t.name, task_age_humanize
        );
    };
    let task_iter = current_task_data.tasks.iter();
    if let Some(status) = status {
        task_iter
            .filter(|t| t.status == status)
            .for_each(print_task);
    } else {
        task_iter.for_each(print_task);
    };
}

fn mark_task(id: u64, status: TaskStatus) {
    let mut current_task_data = get_current_task_data();
    let task_to_update_index = current_task_data.tasks.iter().position(|t| t.id == id);
    if let Some(task_to_update_index) = task_to_update_index {
        let task_to_update = &mut current_task_data.tasks[task_to_update_index];
        if task_to_update.status == status {
            return;
        }
        task_to_update.status = status;
        task_to_update.updated_at = Utc::now();
    } else {
        println!("No task with id {}", id)
    }
    save_task_data(current_task_data);
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { task_name } => add_task(task_name),
        Commands::List { status } => list_task(status),
        Commands::Mark { id, status } => mark_task(id, status),
    }
}
