// Tests for the undo/redo stack.

#[cfg(test)]
mod tests {
    use crate::core::document::Document_core;
    use crate::core::editor::commands::InsertTextCommand_core;
    use crate::core::editor::undo::UndoStack_core;

    #[test]
    fn apply_and_undo() {
        let mut doc = Document_core::from_markdown("Hello\n");
        let mut stack = UndoStack_core::new();

        let cmd = Box::new(InsertTextCommand_core {
            offset: 5,
            text: " world".into(),
        });
        stack.apply(cmd, &mut doc).unwrap();
        assert!(doc.to_markdown().contains("Hello world"));
        assert!(stack.can_undo());

        stack.undo(&mut doc).unwrap();
        assert!(doc.to_markdown().contains("Hello"));
        assert!(!doc.to_markdown().contains("world"));
    }

    #[test]
    fn undo_redo_cycle() {
        let mut doc = Document_core::from_markdown("A\n");
        let mut stack = UndoStack_core::new();

        stack
            .apply(
                Box::new(InsertTextCommand_core {
                    offset: 1,
                    text: "B".into(),
                }),
                &mut doc,
            )
            .unwrap();

        stack.undo(&mut doc).unwrap();
        assert!(!doc.to_markdown().contains("B"));
        assert!(stack.can_redo());

        stack.redo(&mut doc).unwrap();
        assert!(doc.to_markdown().contains("B"));
    }

    #[test]
    fn redo_cleared_after_new_command() {
        let mut doc = Document_core::from_markdown("Start\n");
        let mut stack = UndoStack_core::new();

        stack
            .apply(
                Box::new(InsertTextCommand_core {
                    offset: 5,
                    text: "1".into(),
                }),
                &mut doc,
            )
            .unwrap();

        stack.undo(&mut doc).unwrap();
        assert!(stack.can_redo());

        stack
            .apply(
                Box::new(InsertTextCommand_core {
                    offset: 5,
                    text: "2".into(),
                }),
                &mut doc,
            )
            .unwrap();
        assert!(!stack.can_redo());
    }

    #[test]
    fn multiple_undo() {
        let mut doc = Document_core::from_markdown("Base\n");
        let mut stack = UndoStack_core::new();

        stack
            .apply(
                Box::new(InsertTextCommand_core {
                    offset: 4,
                    text: " one".into(),
                }),
                &mut doc,
            )
            .unwrap();
        stack
            .apply(
                Box::new(InsertTextCommand_core {
                    offset: 8,
                    text: " two".into(),
                }),
                &mut doc,
            )
            .unwrap();

        assert_eq!(stack.depth(), 2);

        stack.undo(&mut doc).unwrap();
        stack.undo(&mut doc).unwrap();
        assert!(!stack.can_undo());

        let md = doc.to_markdown();
        assert!(md.contains("Base"));
        assert!(!md.contains("one"));
        assert!(!md.contains("two"));
    }

    #[test]
    fn undo_nothing_returns_false() {
        let mut doc = Document_core::new();
        let mut stack = UndoStack_core::new();
        let undo_result = stack.undo(&mut doc).unwrap();
        assert!(!undo_result);
    }

    #[test]
    fn undo_description() {
        let mut doc = Document_core::from_markdown("X\n");
        let mut stack = UndoStack_core::new();

        assert!(stack.undo_description().is_none());

        stack
            .apply(
                Box::new(InsertTextCommand_core {
                    offset: 0,
                    text: "Y".into(),
                }),
                &mut doc,
            )
            .unwrap();
        assert_eq!(stack.undo_description(), Some("insert text"));
    }
}
