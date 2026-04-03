use crate::api::models::{CreateTodoRequest, UpdateTodoRequest};
use crate::api::ApiClient;
use crate::cli::{AddArgs, EditArgs, ListArgs, RmArgs};
use crate::output;
use anyhow::Result;

pub fn list(client: &ApiClient, args: &ListArgs, json: bool) -> Result<()> {
    let completed = if args.all {
        None // don't filter by completion
    } else if args.completed {
        Some(true)
    } else {
        Some(false) // default: active only
    };

    let todos = client.list_todos(completed, args.priority.as_deref())?;

    if json {
        output::print_json(&todos);
    } else {
        output::print_todos(&todos);
    }
    Ok(())
}

pub fn add(client: &ApiClient, args: &AddArgs, json: bool) -> Result<()> {
    let todo = client.create_todo(CreateTodoRequest {
        text: args.text.clone(),
        description: args.description.clone(),
        notes: args.notes.clone(),
        priority: args.priority.clone(),
        goal_ids: args.goal.clone(),
        completed: None,
    })?;

    if json {
        output::print_json(&todo);
    } else {
        output::print_todo_created(&todo);
    }
    Ok(())
}

pub fn done(client: &ApiClient, id: &str, json: bool) -> Result<()> {
    let todo = client.update_todo(
        id,
        UpdateTodoRequest {
            completed: Some(true),
            ..Default::default()
        },
    )?;

    if json {
        output::print_json(&todo);
    } else {
        output::print_todo_done(&todo);
    }
    Ok(())
}

pub fn edit(client: &ApiClient, args: &EditArgs, json: bool) -> Result<()> {
    let completed = if args.uncomplete { Some(false) } else { None };

    let todo = client.update_todo(
        &args.id,
        UpdateTodoRequest {
            text: args.text.clone(),
            description: args.description.clone(),
            notes: args.notes.clone(),
            priority: args.priority.clone(),
            completed,
        },
    )?;

    if json {
        output::print_json(&todo);
    } else {
        output::print_todo_updated(&todo);
    }
    Ok(())
}

pub fn rm(client: &ApiClient, args: &RmArgs) -> Result<()> {
    if !args.force {
        eprint!("Delete todo {}? [y/N] ", args.id);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        if !input.trim().eq_ignore_ascii_case("y") {
            println!("Aborted.");
            return Ok(());
        }
    }

    client.delete_todo(&args.id)?;
    output::print_todo_deleted(&args.id);
    Ok(())
}
