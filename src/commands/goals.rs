use crate::api::models::{BulkTagItem, CreateGoalRequest, ReorderPosition, UpdateGoalRequest};
use crate::api::ApiClient;
use crate::cli::GoalCommand;
use crate::error::CliError;
use crate::output;
use anyhow::Result;

pub fn handle(client: &ApiClient, action: &GoalCommand, json: bool) -> Result<()> {
    match action {
        GoalCommand::List { completed, all } => {
            let goals = client.list_goals()?;
            let goals: Vec<_> = goals
                .into_iter()
                .filter(|g| {
                    if *all {
                        true
                    } else if *completed {
                        g.completed
                    } else {
                        !g.completed
                    }
                })
                .collect();
            if json {
                output::print_json(&goals);
            } else {
                output::print_goals(&goals);
            }
        }
        GoalCommand::Add { text, notes } => {
            let goal = client.create_goal(CreateGoalRequest {
                text: text.clone(),
                notes: notes.clone(),
                completed: None,
            })?;
            if json {
                output::print_json(&goal);
            } else {
                output::print_goal_created(&goal);
            }
        }
        GoalCommand::Edit {
            id,
            text,
            notes,
            done,
            uncomplete,
        } => {
            let completed = if *done {
                Some(true)
            } else if *uncomplete {
                Some(false)
            } else {
                None
            };
            let goal = client.update_goal(
                id,
                UpdateGoalRequest {
                    text: text.clone(),
                    notes: notes.clone(),
                    completed,
                },
            )?;
            if json {
                output::print_json(&goal);
            } else {
                output::print_goal_updated(&goal);
            }
        }
        GoalCommand::Done { id } => {
            let goal = client.update_goal(
                id,
                UpdateGoalRequest {
                    completed: Some(true),
                    ..Default::default()
                },
            )?;
            if json {
                output::print_json(&goal);
            } else {
                output::print_goal_done(&goal);
            }
        }
        GoalCommand::Tag { ids, tags } => {
            let updates = ids
                .iter()
                .map(|id| BulkTagItem {
                    id: id.clone(),
                    tags: tags.clone(),
                })
                .collect();
            let result = client.bulk_tag_goals(updates)?;
            if json {
                output::print_json(&result.items);
            } else {
                println!(
                    "Tagged {} goal{}.",
                    result.updated,
                    if result.updated == 1 { "" } else { "s" }
                );
            }
        }
        GoalCommand::Reorder {
            id,
            top,
            bottom,
            after,
        } => {
            let position = if *top {
                ReorderPosition::Named("top".to_string())
            } else if *bottom {
                ReorderPosition::Named("bottom".to_string())
            } else if let Some(after_id) = after {
                ReorderPosition::After {
                    after: after_id.clone(),
                }
            } else {
                return Err(CliError::Other(
                    "specify one of --top, --bottom, or --after <ID>".to_string(),
                )
                .into());
            };
            let goal = client.reorder_goal(id, position)?;
            if json {
                output::print_json(&goal);
            } else {
                println!("Moved: {}", goal.text);
            }
        }
        GoalCommand::Rm { id, force } => {
            if !force {
                eprint!("Delete goal {id}? [y/N] ");
                let mut input = String::new();
                std::io::stdin().read_line(&mut input)?;
                if !input.trim().eq_ignore_ascii_case("y") {
                    println!("Aborted.");
                    return Ok(());
                }
            }
            client.delete_goal(id)?;
            output::print_goal_deleted(id);
        }
    }
    Ok(())
}
