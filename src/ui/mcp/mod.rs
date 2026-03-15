#![allow(non_camel_case_types)]
// MCP server — exposes editor as tool/resource server via rmcp.
// Transport: stdio. Claude Desktop spawns this process with --mcp flag.

/// MCP parameter structs.
pub mod params;
/// AST to JSON serialization.
pub mod ast_json;
/// Block type discriminator enum.
pub mod block_kind_ui;
/// Document output format discriminator enum.
pub mod format_kind_ui;
/// Document statistics helpers.
pub mod stats_ui;
/// Block construction helpers.
pub mod build_block_ui;
/// MCP server constructors.
pub mod server_init_ui;
/// Selection query helper.
pub mod selection_ui;

use std::sync::{Arc, Mutex};

use rmcp::handler::server::router::tool::ToolRouter;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::{tool, tool_router};

use crate::adapter::editor_state_adp::EditorState_adp;
use block_kind_ui::BlockKind_ui;
use build_block_ui::build_block;
use format_kind_ui::FormatKind_ui;
use params::*;

/// MCP server wrapping the editor state.
#[derive(Debug, Clone)]
pub struct EditorMcpServer_ui {
    /// Shared editor state.
    // Arc: shared between MCP tool handlers (potentially concurrent).
    pub state: Arc<Mutex<EditorState_adp>>,
    /// Tool router.
    tool_router: ToolRouter<Self>,
}

#[tool_router]
impl EditorMcpServer_ui {
    #[tool(description = "Get the active document as Markdown (default) or AST JSON")]
    fn get_document(&self, Parameters(params): Parameters<GetDocumentParams_ui>) -> String {
        let Ok(editor_state) = self.state.lock() else {
            return "Error: state lock poisoned".into();
        };
        let fmt = params.format.unwrap_or_else(|| "markdown".into());
        match FormatKind_ui::parse(&fmt) {
            FormatKind_ui::Ast => ast_json::serialize_ast_json(&editor_state.document.blocks),
            FormatKind_ui::Markdown => editor_state.to_markdown(),
        }
    }

    #[tool(description = "Open a .md file — resets cursor, undo stack")]
    fn open_document(&self, Parameters(params): Parameters<OpenDocumentParams_ui>) -> String {
        let Ok(mut editor_state) = self.state.lock() else {
            return "Error: state lock poisoned".into();
        };
        match editor_state.open_file(std::path::Path::new(&params.path)) {
            Ok(()) => format!("Opened: {}", params.path),
            Err(e) => format!("Error: {}", e),
        }
    }

    #[tool(description = "Save active document to its current file path")]
    fn save_document(&self) -> String {
        let Ok(editor_state) = self.state.lock() else {
            return "Error: state lock poisoned".into();
        };
        match editor_state.save_file() {
            Ok(()) => "Saved".into(),
            Err(e) => format!("Error: {}", e),
        }
    }

    #[tool(description = "Insert raw text at a byte offset (or cursor if offset is omitted)")]
    fn insert_text(&self, Parameters(params): Parameters<InsertTextParams_ui>) -> String {
        let Ok(mut editor_state) = self.state.lock() else {
            return "Error: state lock poisoned".into();
        };
        let insert_at = params.offset.unwrap_or(editor_state.cursor);
        let len = params.text.len();
        match editor_state.insert_text_at(insert_at, params.text) {
            Ok(()) => {
                editor_state.cursor = insert_at + len;
                "Inserted".into()
            }
            Err(e) => format!("Error: {}", e),
        }
    }

    #[tool(description = "Replace text in a byte range — primary mutation operation")]
    fn replace_range(&self, Parameters(params): Parameters<ReplaceRangeParams_ui>) -> String {
        let Ok(mut editor_state) = self.state.lock() else {
            return "Error: state lock poisoned".into();
        };
        let new_cursor = params.start + params.replacement.len();
        match editor_state.replace_range(params.start, params.end, params.replacement) {
            Ok(()) => {
                editor_state.cursor = new_cursor;
                "Replaced".into()
            }
            Err(e) => format!("Error: {}", e),
        }
    }

    #[tool(description = "Delete text in a byte range")]
    fn delete_range(&self, Parameters(params): Parameters<DeleteRangeParams_ui>) -> String {
        let Ok(mut editor_state) = self.state.lock() else {
            return "Error: state lock poisoned".into();
        };
        match editor_state.delete_range(params.start, params.end) {
            Ok(()) => {
                editor_state.cursor = params.start;
                "Deleted".into()
            }
            Err(e) => format!("Error: {}", e),
        }
    }

    #[tool(description = "Insert a block (paragraph, heading, code_block, horizontal_rule, bullet_list)")]
    fn insert_block(&self, Parameters(params): Parameters<InsertBlockParams_ui>) -> String {
        let Ok(mut editor_state) = self.state.lock() else {
            return "Error: state lock poisoned".into();
        };
        let text = params.content.unwrap_or_default();
        let idx = params.index.unwrap_or(editor_state.document.blocks.len());

        let block_kind = match BlockKind_ui::parse(&params.block_type) {
            Ok(kind) => kind,
            Err(msg) => return format!("Error: {}", msg),
        };

        let block = build_block(block_kind, text, params.level, params.language);

        match editor_state.insert_block_at(idx, block) {
            Ok(()) => format!("Inserted {} at index {}", params.block_type, idx),
            Err(e) => format!("Error: {}", e),
        }
    }

    #[tool(description = "Move cursor to a UTF-8 byte offset")]
    fn set_cursor(&self, Parameters(params): Parameters<SetCursorParams_ui>) -> String {
        let Ok(mut editor_state) = self.state.lock() else {
            return "Error: state lock poisoned".into();
        };
        editor_state.cursor = params.offset;
        editor_state.selection = None;
        format!("Cursor at {}", params.offset)
    }

    #[tool(description = "Get current selection range and text")]
    fn get_selection(&self) -> String {
        let Ok(editor_state) = self.state.lock() else {
            return "Error: state lock poisoned".into();
        };
        selection_ui::format_selection(&editor_state)
    }

    #[tool(description = "Undo the last editing operation")]
    fn undo(&self) -> String {
        let Ok(mut editor_state) = self.state.lock() else {
            return "Error: state lock poisoned".into();
        };
        match editor_state.undo() {
            Ok(true) => "Undone".into(),
            Ok(false) => "Nothing to undo".into(),
            Err(e) => format!("Error: {}", e),
        }
    }

    #[tool(description = "Redo the last undone operation")]
    fn redo(&self) -> String {
        let Ok(mut editor_state) = self.state.lock() else {
            return "Error: state lock poisoned".into();
        };
        match editor_state.redo() {
            Ok(true) => "Redone".into(),
            Ok(false) => "Nothing to redo".into(),
            Err(e) => format!("Error: {}", e),
        }
    }

    #[tool(description = "Get word count, character count, block count, heading counts")]
    fn get_document_stats(&self) -> String {
        let Ok(editor_state) = self.state.lock() else {
            return "Error: state lock poisoned".into();
        };
        stats_ui::compute_document_stats(&editor_state)
    }
}

