# Rust To-Do List API

A simple, file-based to-do list API built with Rust and the Rocket web framework. This application allows you to manage a list of tasks stored in a `tasks.csv` file.

## Features

*   **List Tasks:** Fetch all tasks in JSON format.
*   **File-based Storage:** Tasks are stored in a simple `tasks.csv` file.
*   **Robust Error Handling:** Gracefully handles errors when parsing the CSV file.

## Getting Started

### Prerequisites

*   [Rust](https://www.rust-lang.org/tools/install) (latest stable version recommended)

### Installation

1.  Clone the repository:
    ```bash
    git clone https://github.com/Sunil-Reddy-Gummalla/todo_rust.git
    ```
2.  Navigate to the project directory:
    ```bash
    cd todo_rust
    ```
3.  Build the project:
    ```bash
    cargo build
    ```

## Usage

Run the application using `cargo`:

```bash
cargo run
```

The application will start on `http://1227.0.0.1:8000`.

## API Endpoints

### `GET /tasks`

Returns a JSON array of all the tasks from the `tasks.csv` file.

**Example Response:**

```json
[
    {
        "task_name": "blockchain",
        "task_description": "Implement blockchain module",
        "task_complete": "No"
    },
    {
        "task_name": "api",
        "task_description": "Develop RESTful API",
        "task_complete": "Yes"
    },
    {
        "task_name": "frontend",
        "task_description": "Create frontend interface",
        "task_complete": "No"
    }
]
```

### `GET /home/<name>`

A simple endpoint to greet a user.

**Example Request:**

`GET /home/world`

**Example Response:**

```
Hello, world!
```

## Project Structure

```
.
├── Cargo.toml
├── Cargo.lock
├── tasks.csv
├── src
│   ├── main.rs
│   └── task.rs
└── target
```

*   `src/main.rs`: The main entry point of the application. It sets up the Rocket server and routes.
*   `src/task.rs`: Defines the `Task` data structure and handles reading from and writing to the `tasks.csv` file.
*   `tasks.csv`: The file where the to-do list tasks are stored.
*   `Cargo.toml`: The package manager configuration file, containing metadata and dependencies.

## Dependencies

This project uses the following Rust crates:

*   [rocket](https://rocket.rs/): A web framework for Rust.
*   [serde](https://serde.rs/): A framework for serializing and deserializing Rust data structures.
*   [csv](https://crates.io/crates/csv): A CSV parser for Rust.