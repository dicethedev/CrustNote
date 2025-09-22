# 🦀 CrustNote

CrustNote is a simple **task/notes REST API** built with [Axum](https://github.com/tokio-rs/axum) and [Rust](https://www.rust-lang.org/).

It’s designed as a learning project to practice **ownership, lifetimes, and JSON serialization** while building real-world APIs.

---

## ✨ Features
- Create new tasks (with `POST /tasks`)
- List all tasks (with `GET /tasks`)
- Fetch a task by ID (with `GET /tasks/:id`)
- Delete a task (with `DELETE /tasks/:id`)
- UUID-based task IDs
- JSON request/response (powered by Serde)

---

## 📦 Installation

First, make sure you have Rust installed:  
👉 [Install Rust](https://www.rust-lang.org/tools/install)

Then clone this repo:

```bash
git clone https://github.com/dicethedev/CrustNote.git
cd CrustNote
```

Build the project:

```bash
cargo build
```
Run the server 

```bash
cargo run
```
By default, CrustNote runs on:
👉 http://127.0.0.1:3000


## API Endpoints

#### Create Task

```json
POST /tasks
Content-Type: application/json

{
  "title": "This is why I love Rust",
  "completed": false
}
```
Response

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "title": "This is why I love Rust",
  "completed": false
}
```

#### List All Tasks

```bash
GET /tasks
```
Response

```json
[
  {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "title": "This is why I love Rust",
    "completed": false
  }
]
```
#### Fetch Task by ID

```bash
GET /tasks/:id
```
Example:

```bash
GET /tasks/550e8400-e29b-41d4-a716-446655440000
```
#### Delete Task

```bash
DELETE /tasks/:id
```

## 🛠 Tech Stack

Rust
Axum  (web framework)
Tokio (async runtime)
Serde (serialization)
UUID  (unique IDs)
