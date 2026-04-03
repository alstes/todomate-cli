use crate::api::models::{CreateGoalRequest, UpdateGoalRequest};
use crate::api::ApiClient;
use crate::cli::GoalCommand;
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
