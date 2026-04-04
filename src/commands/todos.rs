use crate::api::models::{BulkTagItem, CreateTodoRequest, ReorderPosition, UpdateTodoRequest};
use crate::api::ApiClient;
use crate::cli::{AddArgs, EditArgs, ListArgs, ReorderArgs, RmArgs, TagArgs};
use crate::error::CliError;
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

    let todos = client.list_todos(completed, args.priority.as_deref(), args.limit, args.offset)?;

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

pub fn reorder(client: &ApiClient, args: &ReorderArgs, json: bool) -> Result<()> {
    let position = if args.top {
        ReorderPosition::Named("top".to_string())
    } else if args.bottom {
        ReorderPosition::Named("bottom".to_string())
    } else if let Some(after_id) = &args.after {
        ReorderPosition::After {
            after: after_id.clone(),
        }
    } else {
        return Err(
            CliError::Other("specify one of --top, --bottom, or --after <ID>".to_string()).into(),
        );
    };

    let todo = client.reorder_todo(&args.id, position)?;
    if json {
        output::print_json(&todo);
    } else {
        println!("Moved: {}", todo.text);
    }
    Ok(())
}

pub fn tag(client: &ApiClient, args: &TagArgs, json: bool) -> Result<()> {
    let updates = args
        .ids
        .iter()
        .map(|id| BulkTagItem {
            id: id.clone(),
            tags: args.tags.clone(),
        })
        .collect();
    let result = client.bulk_tag_todos(updates)?;
    if json {
        output::print_json(&result.items);
    } else {
        println!(
            "Tagged {} todo{}.",
            result.updated,
            if result.updated == 1 { "" } else { "s" }
        );
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
