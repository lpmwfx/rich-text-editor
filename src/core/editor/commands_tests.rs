// Tests for editing commands.

#[cfg(test)]
mod tests {
    use crate::core::document::{Block, Document_core, Inline};
    use crate::core::editor::commands::*;

    #[test]
    fn insert_text_and_undo() {
        let mut doc = Document_core::from_markdown("Hello world\n");
        let cmd = InsertTextCommand_core {
            offset: 5,
            text: " beautiful".into(),
        };
        cmd.apply(&mut doc).unwrap();
        let md = doc.to_markdown();
        assert!(md.contains("Hello beautiful world"));

        cmd.undo(&mut doc).unwrap();
        let md = doc.to_markdown();
        assert!(md.contains("Hello world"));
        assert!(!md.contains("beautiful"));
    }

    #[test]
    fn delete_range_and_undo() {
        let mut doc = Document_core::from_markdown("Hello beautiful world\n");
        let cmd = DeleteRangeCommand::new(5, 15);
        cmd.apply(&mut doc).unwrap();
        let md = doc.to_markdown();
        assert!(!md.contains("beautiful"));

        cmd.undo(&mut doc).unwrap();
        let md = doc.to_markdown();
        assert!(md.contains("Hello beautiful world"));
    }

    #[test]
    fn replace_range_and_undo() {
        let mut doc = Document_core::from_markdown("Hello world\n");
        let cmd = ReplaceRangeCommand::new(6, 11, "Rust".into());
        cmd.apply(&mut doc).unwrap();
        let md = doc.to_markdown();
        assert!(md.contains("Hello Rust"));

        cmd.undo(&mut doc).unwrap();
        let md = doc.to_markdown();
        assert!(md.contains("Hello world"));
    }

    #[test]
    fn insert_block_and_undo() {
        let mut doc = Document_core::from_markdown("# Title\n");
        let block = Block::Paragraph {
            content: vec![Inline::Text("New paragraph".into())],
            range: None,
        };
        let cmd = InsertBlockCommand {
            index: 1,
            block,
        };
        cmd.apply(&mut doc).unwrap();
        assert_eq!(doc.blocks.len(), 2);

        cmd.undo(&mut doc).unwrap();
        assert_eq!(doc.blocks.len(), 1);
    }

    #[test]
    fn offset_out_of_bounds() {
        let mut doc = Document_core::from_markdown("Hi\n");
        let cmd = InsertTextCommand_core {
            offset: 9999,
            text: "x".into(),
        };
        assert!(cmd.apply(&mut doc).is_err());
    }
}
