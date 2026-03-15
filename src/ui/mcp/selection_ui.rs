#![allow(non_camel_case_types)]
/// Selection query helper for MCP.

use crate::adapter::editor_state_adp::EditorState_adp;

/// Format the current selection as a JSON string.
pub fn format_selection(editor_state: &EditorState_adp) -> String {
    match editor_state.selection {
        Some((start, end)) => {
            let text = editor_state.selected_text().unwrap_or_default();
            format!(
                r#"{{"start": {}, "end": {}, "text": "{}"}}"#,
                start, end, text.replace('"', "\\\"")
            )
        }
        None => "null".into(),
    }
}
