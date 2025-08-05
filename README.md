# 📝 CLI Task Manager in Rust

A lightweight and minimal command-line task manager written in Rust. Tasks are stored locally in a `tasks.json` file and support basic operations like adding, updating, marking, listing, and deleting.

## 🚀 Features

- Add new tasks with descriptions
- List tasks, optionally filtered by status (`todo`, `in-progress`, `done`)
- Mark tasks as `todo`, `in-progress`, or `done`
- Update task name
- Delete tasks
- Data stored persistently in a local JSON file (`tasks.json`)
- Task creation time is humanized (e.g. "2 hours ago")

## 🧑‍💻 Usage

### 🛠 Install

Make sure you have Rust installed:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Clone this repo and build:

```bash
git clone https://github.com/yourusername/cli-task-manager.git
cd cli-task-manager
cargo build --release
```

### 📦 Run

```bash
cargo run -- [COMMAND]
```

### 📚 Commands

```bash
# Add a new task
cargo run -- add "Buy groceries"

# List all tasks
cargo run -- list

# List tasks with specific status
cargo run -- list done

# Mark a task as in progress
cargo run -- mark 1 in-progress

# Update task name
cargo run -- update 1 "Buy vegetables"

# Delete a task
cargo run -- delete 1
```

> Note: Status options are: `todo`, `in-progress`, `done`

## 📁 Data Storage

All tasks are stored in a JSON file named `tasks.json` in the same directory. No external databases required.

Example format:

```json
{
  "tasks": [
    {
      "id": 0,
      "name": "Buy groceries",
      "status": "Todo",
      "created_at": "2025-08-05T02:00:00Z",
      "updated_at": "2025-08-05T02:00:00Z"
    }
  ],
  "id_counter": 1
}
```

## 🧰 Dependencies

- [clap](https://docs.rs/clap/latest/clap/) – for CLI parsing
- [serde](https://serde.rs/) – for JSON serialization
- [chrono](https://docs.rs/chrono/latest/chrono/) – for date/time handling
- [chrono-humanize](https://docs.rs/chrono-humanize/latest/chrono_humanize/) – for human-readable time differences

## 🧪 Example Output

```bash
$ cargo run -- add "Write documentation"
Task added successfully (ID: 0)

$ cargo run -- list
[Todo] #0 Write documentation (in 0 seconds)
```

## 🧹 TODO / Ideas

- Add due dates
- Add priorities
- Sorting or filtering options
- Export/Import from JSON
- Cross-platform sync or cloud storage support

This project is based on the [Task Tracker project from roadmap.sh](https://roadmap.sh/projects/task-tracker).

## 📄 License

MIT
