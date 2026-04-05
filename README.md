# todomate-cli

A native CLI for [TodoMate](https://todo.ac) — the todo app built for developers.

Manage your todos, goals, and vision without leaving the terminal.

> **Coming soon.** TodoMate is in active development and not yet publicly available.
> Watch this repo to be notified at launch.

---

## What is TodoMate?

TodoMate is a todo app designed for developers, built around a simple hierarchy:

**Vision → Goals → Todos**

- Log in with GitHub — no separate account needed
- Your data lives in a private GitHub repository you own
- API-first: web app, iOS app, MCP server, and this CLI all speak the same API
- Free and Pro tiers (coming soon)

---

## Installation

Download a pre-built binary for your platform from [Releases](https://github.com/alstes/todomate-cli/releases).

`cargo install todomate-cli` coming soon.

## Quick start

```sh
todo auth login          # log in via GitHub CLI (gh)
todo list                # list active todos
todo add "Ship it"       # add a todo
todo done <id>           # mark complete
todo goal list           # list goals
todo vision show         # show your vision
```

## Commands

| Command | Description |
|---|---|
| `todo auth login / logout / status` | Authentication |
| `todo list` | List todos (`--priority`, `--limit`, `--offset`, `--all`) |
| `todo add <text>` | Create a todo |
| `todo done <id>` | Mark complete |
| `todo edit <id>` | Update text, priority, notes |
| `todo rm <id>` | Delete a todo |
| `todo reorder <id>` | Move `--top`, `--bottom`, or `--after <id>` |
| `todo tag <id...>` | Set tags on one or more todos |
| `todo goal <subcommand>` | Goal management (same verbs as todos) |
| `todo vision show / set` | View or update your vision |
| `todo config set api-url <url>` | Point at a local API for development |

All commands support `--json` for scripting.

## Requirements

- [GitHub CLI](https://cli.github.com) (`gh`) — used for the default login flow
- A TodoMate Pro account and API key from [todo.ac](https://todo.ac)
