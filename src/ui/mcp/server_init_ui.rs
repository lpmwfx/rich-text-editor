#![allow(non_camel_case_types)]
/// MCP server constructors and handler implementation.

use std::sync::{Arc, Mutex};

use rmcp::model::{ServerCapabilities, ServerInfo};
use rmcp::{tool_handler, ServerHandler};

use crate::adapter::editor_state_adp::EditorState_adp;
use crate::ui::mcp::EditorMcpServer_ui;

impl EditorMcpServer_ui {
    /// Create a new MCP server with a fresh editor state.
    pub fn new() -> Self {
        Self {
            // Arc::new: standalone MCP server owns its own editor state.
            state: Arc::new(Mutex::new(EditorState_adp::new())),
            tool_router: Self::tool_router(),
        }
    }

    /// Create a new MCP server with existing state.
    pub fn with_state(state: Arc<Mutex<EditorState_adp>>) -> Self {
        Self {
            state,
            tool_router: Self::tool_router(),
        }
    }
}

#[tool_handler]
impl ServerHandler for EditorMcpServer_ui {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build())
            .with_instructions("Rich text editor MCP server — edit Markdown documents live".to_string())
    }
}
