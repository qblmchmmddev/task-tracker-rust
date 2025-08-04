use clap::{Parser, Subcommand};

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

fn add_task(task_name: String) {
    println!("Add Task {}", task_name)
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { task_name } => add_task(task_name),
    }
}
