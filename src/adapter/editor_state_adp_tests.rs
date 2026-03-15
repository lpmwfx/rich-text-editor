// Tests for EditorState adapter.

#[cfg(test)]
mod tests {
    use crate::adapter::editor_state_adp::EditorState_adp;
    use crate::core::editor::commands::InsertTextCommand_core;

    #[test]
    fn new_state_is_empty() {
        let editor_state = EditorState_adp::new();
        assert!(editor_state.document.blocks.is_empty());
        assert_eq!(editor_state.cursor, 0);
        assert!(editor_state.selection.is_none());
    }

    #[test]
    fn apply_and_undo() {
        let mut editor_state = EditorState_adp::from_markdown("Hello\n");
        editor_state
            .apply(Box::new(InsertTextCommand_core {
                offset: 5,
                text: " world".into(),
            }))
            .unwrap();
        assert!(editor_state.to_markdown().contains("Hello world"));

        editor_state.undo().unwrap();
        assert!(!editor_state.to_markdown().contains("world"));
    }

    #[test]
    fn selected_text() {
        let mut editor_state = EditorState_adp::from_markdown("Hello world\n");
        editor_state.selection = Some((6, 11));
        assert_eq!(editor_state.selected_text().unwrap(), "world");
    }

    #[test]
    fn open_and_save_file() {
        let dir = std::env::temp_dir().join("rte_test_adapter");
        let _ = std::fs::create_dir_all(&dir);
        let path = dir.join("test_open.md");
        std::fs::write(&path, "# Test\n\nBody\n").unwrap();

        let mut editor_state = EditorState_adp::new();
        editor_state.open_file(&path).unwrap();
        assert_eq!(editor_state.file_path.as_ref().unwrap(), &path);
        assert!(!editor_state.document.blocks.is_empty());

        let md = editor_state.to_markdown();
        let insert_offset = md.find("Body").unwrap();
        editor_state
            .apply(Box::new(InsertTextCommand_core {
                offset: insert_offset,
                text: "Edited ".into(),
            }))
            .unwrap();
        editor_state.save_file().unwrap();

        let saved = std::fs::read_to_string(&path).unwrap();
        assert!(saved.contains("Edited Body"));

        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn save_without_file_errors() {
        let editor_state = EditorState_adp::new();
        assert!(editor_state.save_file().is_err());
    }
}
