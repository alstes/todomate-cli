use crate::api::models::{Goal, Todo, Vision};
use colored_json::ColorMode;
use comfy_table::{presets::UTF8_BORDERS_ONLY, Table};
use owo_colors::OwoColorize;
use serde::Serialize;

use std::sync::OnceLock;

static NO_COLOR: OnceLock<bool> = OnceLock::new();

pub fn init(no_color: bool) {
    let piped = !atty_stdout();
    NO_COLOR.set(no_color || piped).ok();
}

fn color_enabled() -> bool {
    !NO_COLOR.get().copied().unwrap_or(false)
}

fn atty_stdout() -> bool {
    // Simple heuristic: check if stdout file descriptor is a terminal.
    // We avoid pulling in the `atty` crate by using libc on Unix / Windows API on Windows.
    // For now, use the presence of the TERM env var as a proxy (good enough for v0.1).
    std::env::var("TERM").map(|v| v != "dumb").unwrap_or(false)
        || std::env::var("COLORTERM").is_ok()
}

// --- JSON passthrough ---

pub fn print_json<T: Serialize>(value: &T) {
    let mode = if color_enabled() {
        ColorMode::On
    } else {
        ColorMode::Off
    };
    let s = colored_json::to_colored_json(value, mode).unwrap_or_default();
    println!("{s}");
}

// --- Todo output ---

pub fn print_todos(todos: &[Todo]) {
    if todos.is_empty() {
        println!("No todos.");
        return;
    }

    let mut table = Table::new();
    table.load_preset(UTF8_BORDERS_ONLY);
    table.set_header(vec!["#", "Priority", "Text"]);

    for (i, todo) in todos.iter().enumerate() {
        let priority = format_priority(&todo.priority);
        table.add_row(vec![(i + 1).to_string(), priority, todo.text.clone()]);
    }

    println!("{table}");
}

pub fn print_todo_created(todo: &Todo) {
    if color_enabled() {
        println!("{} {}", "+ Created:".green(), format_todo_line(todo));
    } else {
        println!("+ Created: {}", format_todo_line(todo));
    }
}

pub fn print_todo_done(todo: &Todo) {
    if color_enabled() {
        println!("{} {}  {}", "✓ Done:".green(), todo.text, todo.id.dimmed());
    } else {
        println!("✓ Done: {}  {}", todo.text, todo.id);
    }
}

pub fn print_todo_deleted(id: &str) {
    println!("Deleted todo {id}");
}

pub fn print_todo_updated(todo: &Todo) {
    if color_enabled() {
        println!("{} {}", "~ Updated:".cyan(), format_todo_line(todo));
    } else {
        println!("~ Updated: {}", format_todo_line(todo));
    }
}

fn format_todo_line(todo: &Todo) -> String {
    format!("{}  [{}]  {}", todo.text, todo.priority.to_uppercase(), todo.id)
}

fn format_priority(p: &str) -> String {
    match p {
        "high" => {
            if color_enabled() {
                "HIGH".red().to_string()
            } else {
                "HIGH".to_string()
            }
        }
        "medium" => {
            if color_enabled() {
                "MEDIUM".yellow().to_string()
            } else {
                "MEDIUM".to_string()
            }
        }
        "low" => "LOW".to_string(),
        other => other.to_uppercase(),
    }
}

// --- Goal output ---

pub fn print_goals(goals: &[Goal]) {
    if goals.is_empty() {
        println!("No goals.");
        return;
    }

    let mut table = Table::new();
    table.load_preset(UTF8_BORDERS_ONLY);
    table.set_header(vec!["#", "Text"]);

    for (i, goal) in goals.iter().enumerate() {
        table.add_row(vec![(i + 1).to_string(), goal.text.clone()]);
    }

    println!("{table}");
}

pub fn print_goal_created(goal: &Goal) {
    if color_enabled() {
        println!("{} {}", "+ Created:".green(), goal.text);
    } else {
        println!("+ Created: {}", goal.text);
    }
}

pub fn print_goal_done(goal: &Goal) {
    if color_enabled() {
        println!("{} {}", "✓ Done:".green(), goal.text);
    } else {
        println!("✓ Done: {}", goal.text);
    }
}

pub fn print_goal_deleted(id: &str) {
    println!("Deleted goal {id}");
}

pub fn print_goal_updated(goal: &Goal) {
    if color_enabled() {
        println!("{} {}", "~ Updated:".cyan(), goal.text);
    } else {
        println!("~ Updated: {}", goal.text);
    }
}

// --- Vision output ---

pub fn print_vision(vision: &Vision) {
    if vision.description.is_empty() {
        println!("No vision set.");
    } else {
        println!("{}", vision.description);
    }
}
