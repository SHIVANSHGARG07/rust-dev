# notes-mcp-server

A tiny end-to-end MCP (Model Context Protocol) server written in Rust with
the official [`rmcp`](https://github.com/modelcontextprotocol/rust-sdk)
SDK (v1.7.0). It keeps an in-memory list of notes and exposes four tools:

| Tool          | Arguments          | What it does                    |
|---------------|--------------------|----------------------------------|
| `add_note`    | `title`, `content` | Creates a note, returns its id  |
| `list_notes`  | —                   | Lists all notes                 |
| `get_note`    | `id`                | Fetches one note                |
| `delete_note` | `id`                | Deletes one note                |

It talks to clients over **stdio** — the same transport Claude Desktop and
Claude Code use to launch local MCP servers as a child process.

## 1. Prerequisites

- Rust + Cargo installed. If you don't have it: https://rustup.rs
- Check your version:
  ```bash
  rustc --version
  cargo --version
  ```

## 2. Build it

```bash
cd notes-mcp-server
cargo build
```

The first build will download `rmcp` and its dependencies from crates.io,
so you'll need an internet connection for this step. It may take a minute
or two the first time because `rmcp`'s macros pull in a fair number of
transitive crates.

If you hit a version resolution error, delete `Cargo.lock` (if one got
generated) and run `cargo update` then `cargo build` again.

## 3. Run it directly (sanity check)

```bash
cargo run
```

You should see:
```
notes-mcp-server: starting up, waiting for a client over stdio...
```
...and then it will just sit there. That's correct — it's waiting for an
MCP client to speak JSON-RPC to it over stdin/stdout. Ctrl+C to stop.

**Never `println!` in this server.** stdout is the protocol channel; any
stray text on stdout will corrupt the JSON-RPC stream and the client will
fail to parse it. Use `eprintln!` (stderr) for logging, as `main.rs` does.

## 4. Test it with the MCP Inspector (recommended first step)

The [MCP Inspector](https://github.com/modelcontextprotocol/inspector) is
the standard way to poke at a server interactively without wiring up a
full client. It requires Node.js.

```bash
npx @modelcontextprotocol/inspector cargo run
```

This opens a browser UI where you can:
- See the four tools with their generated JSON schemas
- Call `add_note`, then `list_notes` to see it persisted in memory
- Call `get_note` / `delete_note` with the id you got back

## 5. Wire it into Claude Desktop or Claude Code

First, build a release binary so the config points at something fast and
stable:

```bash
cargo build --release
```

The binary will be at `target/release/notes-mcp-server` (or
`notes-mcp-server.exe` on Windows). Note the **full absolute path** to it.

### Claude Desktop

Edit your Claude Desktop config file:
- macOS: `~/Library/Application Support/Claude/claude_desktop_config.json`
- Windows: `%APPDATA%\Claude\claude_desktop_config.json`

Add an entry under `mcpServers`:

```json
{
  "mcpServers": {
    "notes": {
      "command": "/absolute/path/to/notes-mcp-server/target/release/notes-mcp-server"
    }
  }
}
```

Restart Claude Desktop. The four note tools should now show up in the
tool picker.

### Claude Code

From the project directory you want the server available in:

```bash
claude mcp add notes /absolute/path/to/notes-mcp-server/target/release/notes-mcp-server
```

## 6. Where to go next

- Add a `resources` capability (e.g. expose each note as a
  `note://{id}` resource) — see the "Resources" section of the
  [rmcp README](https://github.com/modelcontextprotocol/rust-sdk).
- Swap the in-memory `HashMap` for a real store (SQLite via `sqlx`,
  or a JSON file on disk) so notes survive a restart.
- Add a second transport: `rmcp` also supports Streamable HTTP
  (`transport-streamable-http-server` feature) if you want the server
  reachable over the network instead of only as a local child process.
- Read the MCP spec itself: https://modelcontextprotocol.io
