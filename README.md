<h1 align="center">agent-desktop</h1>

<p align="center">Computer use for AI agents: read and operate desktop apps through the OS accessibility tree.</p>

A standalone Rust CLI for macOS. A snapshot returns JSON with a ref for every element an agent can act on. The agent acts on a ref and snapshots again to check, with no pixel matching; the loop stays in your agent.

On macOS, CLI calls reuse a private command host for each snapshot namespace and binary build. It retains native accessibility objects so a removed row cannot silently resolve to a surviving row with the same identifier. The host exits after 15 idle minutes. `TMPDIR` must be owned by the current user with mode `0700`; the standard macOS per-user temporary directory satisfies this requirement. Headless calls still prohibit foreground activation and synthesized input.

Snapshots, clipboard reads, screenshots, command output and debug artifacts can contain private desktop content. Keep them out of public logs and uploads.

## Installation

Build from source on macOS with the Rust toolchain specified in `rust-toolchain.toml`:

```bash
cargo build --release -p agent-desktop
./target/release/agent-desktop version
```

The executable is `target/release/agent-desktop`. Add that directory to your `PATH` to use the commands below.

Grant Accessibility permission to the terminal or agent running the CLI:

```bash
agent-desktop permissions --guide
```

## Quick start

Observe, act on a ref, observe again. Use an app name from `agent-desktop list-apps`; the example below uses Finder.
Each command prints one JSON envelope; a trimmed snapshot follows the commands.

```bash
agent-desktop snapshot --app Finder -i      # interactive elements, each with a ref
agent-desktop click @s8f3k2p9:e8            # act on a ref from that snapshot
agent-desktop snapshot --app Finder -i      # observe again to verify
```

```json
{
  "version": "2.5",
  "ok": true,
  "command": "snapshot",
  "data": {
    "app": "Finder",
    "window": { "id": "w-204749", "title": "Downloads" },
    "snapshot_id": "s8f3k2p9",
    "ref_count": 59,
    "complete": true,
    "tree": {
      "role": "window",
      "name": "Downloads",
      "children": [
        { "role": "treeitem", "name": "Documents", "ref_id": "@s8f3k2p9:e8", "available_actions": ["Click"] }
      ]
    }
  }
}
```

- **Refs belong to their snapshot.** A bare `@e8` is rejected with `INVALID_ARGS`. When the UI changes, an action
  returns `STALE_REF` or `AMBIGUOUS_TARGET`: take a new snapshot and use its refs.
- **Several windows** give `AMBIGUOUS_TARGET`; `snapshot` lists the candidate window ids in `error.details`. Pass one
  with `--window-id`.
- **Dense apps:** map first with `snapshot --skeleton --app Slack -i --compact`, then drill into one region with
  `snapshot --root @s8f3k2p9:e3 -i --compact`.
- **Headless by default.** Ref actions go through the accessibility API without moving the pointer or taking focus.
  The global `--headed` flag allows physical input.

## JSON output


## Commands

It ships 60 command names, 56 operational commands. The other four (`key-down`, `key-up`, `mouse-down`, `mouse-up`) refuse to run until a daemon can release a held key or button.

| Group | Commands |
|---|---|
| Observe | `snapshot`, `find`, `get`, `is`, `screenshot`, `list-surfaces` |
| Act on a ref | `click`, `double-click`, `triple-click`, `right-click`, `type`, `set-value`, `clear`, `focus`, `select`, `toggle`, `check`, `uncheck`, `expand`, `collapse`, `scroll`, `scroll-to` |
| Keyboard and mouse | `press`, `hover`, `drag`, `mouse-move`, `mouse-click`, `mouse-wheel`, `key-down`, `key-up`, `mouse-down`, `mouse-up` |
| Apps and windows | `launch`, `close-app`, `list-apps`, `list-windows`, `list-displays`, `focus-window`, `resize-window`, `move-window`, `minimize`, `maximize`, `restore`, `open-system-surface` |
| Notifications | `list-notifications`, `dismiss-notification`, `dismiss-all-notifications`, `notification-action` |
| Clipboard | `clipboard-get`, `clipboard-set`, `clipboard-clear` |
| Wait | `wait` |
| Sessions | `session`, `trace`, `cursor-overlay` |
| System | `status`, `permissions`, `version`, `batch`, `skills` |

## macOS support

The CLI uses the native macOS accessibility API for snapshots and element actions. It also supports keyboard and mouse input, screenshots, clipboard access, app and window management, and notifications. Shell surfaces through `open-system-surface` are planned.

## Development

Run checks on macOS:

```bash
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all -- --check
```

The workspace contains the shared core (`crates/core`), macOS adapter (`crates/macos`), and CLI (`src`).

## Documentation

| I want to... | Read |
|---|---|

## License

Apache-2.0
