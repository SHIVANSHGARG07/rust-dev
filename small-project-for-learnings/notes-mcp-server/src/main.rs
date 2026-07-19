// A minimal end-to-end MCP server, written with the official `rmcp` Rust SDK.
//
// It exposes four "tools" that any MCP-compatible client (Claude Desktop,
// Claude Code, the MCP Inspector, etc.) can call:
//   - add_note(title, content)  -> creates a note, returns its id
//   - list_notes()              -> lists all notes currently in memory
//   - get_note(id)               -> fetches one note by id
//   - delete_note(id)            -> removes a note by id
//
// Storage is a simple in-memory HashMap behind a Mutex, wrapped in an Arc so
// it can be cheaply cloned (rmcp clones the handler per-connection).
//
// IMPORTANT: stdout is reserved for the MCP JSON-RPC protocol messages.
// Never `println!` in a stdio-transport server -- it will corrupt the
// protocol stream. Use stderr (eprintln!) for any debug logging instead.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use rmcp::{
    handler::server::wrapper::Parameters, schemars, tool, tool_router, transport::stdio,
    ServiceExt,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
struct Note {
    id: String,
    title: String,
    content: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct AddNoteParams {
    /// Short title for the note
    title: String,
    /// Body text of the note
    content: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct NoteIdParams {
    /// The id returned by add_note
    id: String,
}

#[derive(Clone)]
struct NotesServer {
    notes: Arc<Mutex<HashMap<String, Note>>>,
}

impl NotesServer {
    fn new() -> Self {
        Self {
            notes: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

// `#[tool_router(server_handler)]` scans the impl block below, turns every
// `#[tool]`-annotated method into an MCP tool definition (name, description,
// JSON schema derived from the parameter struct), AND generates the
// `ServerHandler` implementation for us -- that's what `server_handler` does.
#[tool_router(server_handler)]
impl NotesServer {
    #[tool(description = "Create a new note and return its generated id")]
    fn add_note(
        &self,
        Parameters(AddNoteParams { title, content }): Parameters<AddNoteParams>,
    ) -> String {
        let id = Uuid::new_v4().to_string();
        let note = Note {
            id: id.clone(),
            title,
            content,
        };
        self.notes.lock().unwrap().insert(id.clone(), note);
        format!("Created note with id: {id}")
    }

    #[tool(description = "List every note currently stored, with id, title, and content")]
    fn list_notes(&self) -> String {
        let notes = self.notes.lock().unwrap();
        if notes.is_empty() {
            return "No notes yet.".to_string();
        }
        notes
            .values()
            .map(|n| format!("- [{}] {}: {}", n.id, n.title, n.content))
            .collect::<Vec<_>>()
            .join("\n")
    }

    #[tool(description = "Fetch a single note by its id")]
    fn get_note(&self, Parameters(NoteIdParams { id }): Parameters<NoteIdParams>) -> String {
        match self.notes.lock().unwrap().get(&id) {
            Some(n) => format!("{}: {}", n.title, n.content),
            None => format!("No note found with id: {id}"),
        }
    }

    #[tool(description = "Delete a note by its id")]
    fn delete_note(&self, Parameters(NoteIdParams { id }): Parameters<NoteIdParams>) -> String {
        if self.notes.lock().unwrap().remove(&id).is_some() {
            format!("Deleted note: {id}")
        } else {
            format!("No note found with id: {id}")
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    eprintln!("notes-mcp-server: starting up, waiting for a client over stdio...");

    // `stdio()` wires the server up to process stdin/stdout as the JSON-RPC
    // transport. `.serve(...)` performs the MCP initialize handshake, and
    // `.waiting()` blocks the main task until the client disconnects.
    let service = NotesServer::new().serve(stdio()).await?;
    service.waiting().await?;

    Ok(())
}
