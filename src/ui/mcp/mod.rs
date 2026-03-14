// MCP server — exposes editor as tool/resource server via rmcp.
// Transport: stdio. Claude Desktop spawns this process with --mcp flag.

use std::sync::{Arc, Mutex};

use rmcp::handler::server::router::tool::ToolRouter;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::{ServerCapabilities, ServerInfo};
use rmcp::{schemars, tool, tool_handler, tool_router, ServerHandler};

use crate::adapter::editor_state_adp::EditorState;
use crate::core::document::{Block, Inline};
use crate::core::editor::commands::{
    DeleteRangeCommand, InsertBlockCommand, InsertTextCommand, ReplaceRangeCommand,
};

// -- Parameter structs with schemars derive --

/// Parameters for get_document tool.
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct GetDocumentParams {
    /// Output format: "markdown" (default) or "ast".
    pub format: Option<String>,
}

/// Parameters for open_document tool.
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct OpenDocumentParams {
    /// Absolute or project-relative path to .md file.
    pub path: String,
}

/// Parameters for insert_text tool.
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct InsertTextParams {
    /// Text to insert.
    pub text: String,
    /// UTF-8 byte offset (None = cursor position).
    pub offset: Option<usize>,
}

/// Parameters for replace_range tool.
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct ReplaceRangeParams {
    /// Start offset (inclusive).
    pub start: usize,
    /// End offset (exclusive).
    pub end: usize,
    /// Replacement text.
    pub replacement: String,
}

/// Parameters for delete_range tool.
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct DeleteRangeParams {
    /// Start offset (inclusive).
    pub start: usize,
    /// End offset (exclusive).
    pub end: usize,
}

/// Parameters for insert_block tool.
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct InsertBlockParams {
    /// Block type: paragraph, heading, code_block, horizontal_rule, bullet_list.
    pub block_type: String,
    /// Text content (ignored for horizontal_rule).
    pub content: Option<String>,
    /// Heading level 1-3 (only for heading).
    pub level: Option<u8>,
    /// Code language (only for code_block).
    pub language: Option<String>,
    /// Block index to insert at (None = end).
    pub index: Option<usize>,
}

/// Parameters for set_cursor tool.
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct SetCursorParams {
    /// UTF-8 byte offset.
    pub offset: usize,
}

/// MCP server wrapping the editor state.
#[derive(Debug, Clone)]
pub struct EditorMcpServer {
    /// Shared editor state.
    pub state: Arc<Mutex<EditorState>>,
    /// Tool router.
    tool_router: ToolRouter<Self>,
}

#[tool_router]
impl EditorMcpServer {
    /// Create a new MCP server with a fresh editor state.
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(EditorState::new())),
            tool_router: Self::tool_router(),
        }
    }

    /// Create a new MCP server with existing state.
    pub fn with_state(state: Arc<Mutex<EditorState>>) -> Self {
        Self {
            state,
            tool_router: Self::tool_router(),
        }
    }

    #[tool(description = "Get the active document as Markdown (default) or AST JSON")]
    fn get_document(&self, Parameters(params): Parameters<GetDocumentParams>) -> String {
        let state = self.state.lock().unwrap();
        let fmt = params.format.unwrap_or_else(|| "markdown".into());
        match fmt.as_str() {
            "ast" => serialize_ast_json(&state.document),
            _ => state.to_markdown(),
        }
    }

    #[tool(description = "Open a .md file — resets cursor, undo stack")]
    fn open_document(&self, Parameters(params): Parameters<OpenDocumentParams>) -> String {
        let mut state = self.state.lock().unwrap();
        match state.open_file(std::path::Path::new(&params.path)) {
            Ok(()) => format!("Opened: {}", params.path),
            Err(e) => format!("Error: {}", e),
        }
    }

    #[tool(description = "Save active document to its current file path")]
    fn save_document(&self) -> String {
        let state = self.state.lock().unwrap();
        match state.save_file() {
            Ok(()) => "Saved".into(),
            Err(e) => format!("Error: {}", e),
        }
    }

    #[tool(description = "Insert raw text at a byte offset (or cursor if offset is omitted)")]
    fn insert_text(&self, Parameters(params): Parameters<InsertTextParams>) -> String {
        let mut state = self.state.lock().unwrap();
        let insert_at = params.offset.unwrap_or(state.cursor);
        let len = params.text.len();
        match state.apply(Box::new(InsertTextCommand {
            offset: insert_at,
            text: params.text,
        })) {
            Ok(()) => {
                state.cursor = insert_at + len;
                "Inserted".into()
            }
            Err(e) => format!("Error: {}", e),
        }
    }

    #[tool(description = "Replace text in a byte range — primary mutation operation")]
    fn replace_range(&self, Parameters(params): Parameters<ReplaceRangeParams>) -> String {
        let mut state = self.state.lock().unwrap();
        let new_cursor = params.start + params.replacement.len();
        match state.apply(Box::new(ReplaceRangeCommand::new(
            params.start,
            params.end,
            params.replacement,
        ))) {
            Ok(()) => {
                state.cursor = new_cursor;
                "Replaced".into()
            }
            Err(e) => format!("Error: {}", e),
        }
    }

    #[tool(description = "Delete text in a byte range")]
    fn delete_range(&self, Parameters(params): Parameters<DeleteRangeParams>) -> String {
        let mut state = self.state.lock().unwrap();
        match state.apply(Box::new(DeleteRangeCommand::new(params.start, params.end))) {
            Ok(()) => {
                state.cursor = params.start;
                "Deleted".into()
            }
            Err(e) => format!("Error: {}", e),
        }
    }

    #[tool(description = "Insert a block (paragraph, heading, code_block, horizontal_rule, bullet_list)")]
    fn insert_block(&self, Parameters(params): Parameters<InsertBlockParams>) -> String {
        let mut state = self.state.lock().unwrap();
        let text = params.content.unwrap_or_default();
        let idx = params.index.unwrap_or(state.document.blocks.len());

        let block = match params.block_type.as_str() {
            "paragraph" => Block::Paragraph {
                content: vec![Inline::Text(text)],
                range: None,
            },
            "heading" => Block::Heading {
                level: params.level.unwrap_or(1),
                content: vec![Inline::Text(text)],
                range: None,
            },
            "code_block" => Block::CodeBlock {
                lang: params.language,
                code: text,
                range: None,
            },
            "horizontal_rule" => Block::HorizontalRule { range: None },
            "bullet_list" => Block::BulletList {
                items: text
                    .lines()
                    .map(|l| vec![Inline::Text(l.to_string())])
                    .collect(),
                range: None,
            },
            other => return format!("Error: unknown block type '{}'", other),
        };

        match state.apply(Box::new(InsertBlockCommand { index: idx, block })) {
            Ok(()) => format!("Inserted {} at index {}", params.block_type, idx),
            Err(e) => format!("Error: {}", e),
        }
    }

    #[tool(description = "Move cursor to a UTF-8 byte offset")]
    fn set_cursor(&self, Parameters(params): Parameters<SetCursorParams>) -> String {
        let mut state = self.state.lock().unwrap();
        state.cursor = params.offset;
        state.selection = None;
        format!("Cursor at {}", params.offset)
    }

    #[tool(description = "Get current selection range and text")]
    fn get_selection(&self) -> String {
        let state = self.state.lock().unwrap();
        match state.selection {
            Some((start, end)) => {
                let text = state.selected_text().unwrap_or_default();
                format!(
                    r#"{{"start": {}, "end": {}, "text": "{}"}}"#,
                    start,
                    end,
                    text.replace('"', "\\\"")
                )
            }
            None => "null".into(),
        }
    }

    #[tool(description = "Undo the last editing operation")]
    fn undo(&self) -> String {
        let mut state = self.state.lock().unwrap();
        match state.undo() {
            Ok(true) => "Undone".into(),
            Ok(false) => "Nothing to undo".into(),
            Err(e) => format!("Error: {}", e),
        }
    }

    #[tool(description = "Redo the last undone operation")]
    fn redo(&self) -> String {
        let mut state = self.state.lock().unwrap();
        match state.redo() {
            Ok(true) => "Redone".into(),
            Ok(false) => "Nothing to redo".into(),
            Err(e) => format!("Error: {}", e),
        }
    }

    #[tool(description = "Get word count, character count, block count, heading counts")]
    fn get_document_stats(&self) -> String {
        let state = self.state.lock().unwrap();
        let md = state.to_markdown();
        let words = md.split_whitespace().count();
        let characters = md.len();
        let characters_no_spaces = md.chars().filter(|c| !c.is_whitespace()).count();
        let blocks = state.document.blocks.len();

        let (mut h1, mut h2, mut h3) = (0usize, 0usize, 0usize);
        let (mut images, mut videos) = (0usize, 0usize);

        for block in &state.document.blocks {
            match block {
                Block::Heading { level: 1, .. } => h1 += 1,
                Block::Heading { level: 2, .. } => h2 += 1,
                Block::Heading { level: 3, .. } => h3 += 1,
                Block::Image { .. } => images += 1,
                Block::Video { .. } => videos += 1,
                _ => {}
            }
        }

        format!(
            r#"{{"words": {}, "characters": {}, "characters_no_spaces": {}, "blocks": {}, "images": {}, "videos": {}, "headings": {{"h1": {}, "h2": {}, "h3": {}}}}}"#,
            words, characters, characters_no_spaces, blocks, images, videos, h1, h2, h3
        )
    }
}

#[tool_handler]
impl ServerHandler for EditorMcpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build())
            .with_instructions("Rich text editor MCP server — edit Markdown documents live".to_string())
    }
}

/// Serialize the document AST to a JSON string with byte ranges.
fn serialize_ast_json(doc: &crate::core::document::Document) -> String {
    let mut blocks_json = Vec::new();

    for block in &doc.blocks {
        let bj = match block {
            Block::Paragraph { content, range } => format!(
                r#"{{"type": "Paragraph", "byte_range": {}, "inlines": {}}}"#,
                range_json(range),
                inlines_json(content)
            ),
            Block::Heading { level, content, range } => format!(
                r#"{{"type": "Heading", "level": {}, "byte_range": {}, "content": "{}"}}"#,
                level,
                range_json(range),
                content.iter().map(|i| i.plain_text()).collect::<String>().replace('"', "\\\"")
            ),
            Block::CodeBlock { lang, code, range } => format!(
                r#"{{"type": "CodeBlock", "lang": {}, "byte_range": {}}}"#,
                lang.as_ref().map(|l| format!("\"{}\"", l)).unwrap_or_else(|| "null".into()),
                range_json(range)
            ),
            Block::Image { alt, path, range, .. } => format!(
                r#"{{"type": "Image", "alt": "{}", "path": "{}", "byte_range": {}}}"#,
                alt.replace('"', "\\\""),
                path.replace('"', "\\\""),
                range_json(range)
            ),
            Block::Video { path, range, .. } => format!(
                r#"{{"type": "Video", "path": "{}", "byte_range": {}}}"#,
                path.replace('"', "\\\""),
                range_json(range)
            ),
            Block::BulletList { items, range } => format!(
                r#"{{"type": "BulletList", "items": {}, "byte_range": {}}}"#,
                items.len(),
                range_json(range)
            ),
            Block::HorizontalRule { range } => format!(
                r#"{{"type": "HorizontalRule", "byte_range": {}}}"#,
                range_json(range)
            ),
        };
        blocks_json.push(bj);
    }

    format!(r#"{{"blocks": [{}]}}"#, blocks_json.join(", "))
}

/// Format a ByteRange as JSON.
fn range_json(range: &Option<crate::core::document::ByteRange>) -> String {
    match range {
        Some(r) => format!("[{}, {}]", r.start, r.end),
        None => "null".into(),
    }
}

/// Format inline elements as a JSON array.
fn inlines_json(inlines: &[Inline]) -> String {
    let items: Vec<String> = inlines
        .iter()
        .map(|i| match i {
            Inline::Text(s) => format!(r#"{{"type": "Text", "text": "{}"}}"#, s.replace('"', "\\\"")),
            Inline::Bold(_) => r#"{"type": "Bold"}"#.into(),
            Inline::Italic(_) => r#"{"type": "Italic"}"#.into(),
            Inline::Code(s) => format!(r#"{{"type": "Code", "text": "{}"}}"#, s.replace('"', "\\\"")),
            Inline::Link { text, url } => format!(
                r#"{{"type": "Link", "text": "{}", "url": "{}"}}"#,
                text.replace('"', "\\\""),
                url.replace('"', "\\\"")
            ),
        })
        .collect();
    format!("[{}]", items.join(", "))
}
