// Rich text editor — GUI mode or MCP server mode.

slint::include_modules!();

use rich_text_editor::adapter::editor_app_adp::EditorApp_adp;
use rich_text_editor::shared::sizes::SCROLL_FACTOR;
use std::cell::RefCell;
use std::rc::Rc;

/// Sample Markdown for initial display.
const SAMPLE_MD: &str = "\
# Rich Text Editor

This is a **bold** and *italic* demo.

## Features

- Markdown parsing
- Skia/skparagraph rendering
- Cursor and selection

Here is some `inline code` in a paragraph.

---

[Visit example](https://example.com)
";

/// Update the UI after state changes.
fn sync_ui(ui: &AppWindow, app_ref: &EditorApp_adp) {
    ui.set_rendered_content(app_ref.render_frame());
    let (line, col) = app_ref.cursor_line_col();
    ui.set_cursor_line(line);
    ui.set_cursor_col(col);
}

/// Wire all Slint callbacks to the shared editor app state.
fn wire_callbacks(ui: &AppWindow, app: &Rc<RefCell<EditorApp_adp>>) {
    {
        let app_clone = Rc::clone(app);
        let ui_weak = ui.as_weak();
        ui.on_editor_clicked(move |x, y| {
            let mut app_ref = app_clone.borrow_mut();
            app_ref.handle_click(x, y);
            if let Some(ui) = ui_weak.upgrade() { sync_ui(&ui, &app_ref); }
        });
    }

    {
        let app_clone = Rc::clone(app);
        let ui_weak = ui.as_weak();
        ui.on_editor_key_pressed(move |key, shift, ctrl, alt| {
            let mut app_ref = app_clone.borrow_mut();
            let consumed = app_ref.handle_key(&key, shift, ctrl, alt);
            if consumed {
                if let Some(ui) = ui_weak.upgrade() { sync_ui(&ui, &app_ref); }
            }
            consumed
        });
    }

    {
        let app_clone = Rc::clone(app);
        let ui_weak = ui.as_weak();
        ui.on_editor_scroll(move |delta_y| {
            let mut app_ref = app_clone.borrow_mut();
            app_ref.scroll_y -= delta_y * SCROLL_FACTOR;
            app_ref.clamp_scroll();
            if let Some(ui) = ui_weak.upgrade() {
                ui.set_rendered_content(app_ref.render_frame());
            }
        });
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.contains(&"--mcp".to_string()) {
        tracing_subscriber::fmt()
            .with_writer(std::io::stderr)
            .with_ansi(false)
            .init();
        eprintln!("MCP server mode not yet implemented");
        return Ok(());
    }

    let ui = AppWindow::new()?;
    // Rc: shared between Slint callback closures (single-threaded UI).
    let app = Rc::new(RefCell::new(EditorApp_adp::new(SAMPLE_MD)));
    sync_ui(&ui, &app.borrow());
    wire_callbacks(&ui, &app);
    ui.run()?;
    Ok(())
}
