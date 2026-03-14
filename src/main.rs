/// Rich text editor — GUI mode or MCP server mode.

slint::include_modules!();

use rich_text_editor::core::document::Document;
use rich_text_editor::pal::{build_paragraphs, build_renderable_paragraphs, cursor, selection};
use rich_text_editor::render;
use std::sync::{Arc, Mutex};
use std::cell::RefCell;
use std::path::PathBuf;

/// Editor canvas width in pixels
const EDITOR_WIDTH: f32 = 800.0;
/// Editor canvas height in pixels
const EDITOR_HEIGHT: f32 = 500.0;
/// Line height in pixels (for coordinate-to-offset calculation)
const LINE_HEIGHT: f32 = 28.0;
/// Character width in pixels (for coordinate-to-offset calculation)
const CHAR_WIDTH: f32 = 8.4;
/// Characters per line (for offset calculation)
const CHARS_PER_LINE: usize = 40;

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

/// Run the editor in GUI mode with Slint UI.
fn run_gui() -> anyhow::Result<()> {
    let ui = AppWindow::new()?;

    // Parse sample markdown
    let doc = Document::from_markdown(SAMPLE_MD);

    // Build renderable Skia paragraphs (actual Paragraph objects)
    let renderable_paragraphs = build_renderable_paragraphs(&doc, EDITOR_WIDTH)?;

    // Also get metadata for UI display
    let metadata = build_paragraphs(&doc, EDITOR_WIDTH)?;

    // Render paragraphs to PNG file
    let temp_render_path = PathBuf::from(std::env::temp_dir()).join("editor-render.png");
    render::render_paragraphs_to_file(&renderable_paragraphs, EDITOR_WIDTH, EDITOR_HEIGHT, &temp_render_path)?;

    // Display rendered content in UI (pass file path as string)
    let rendered_image = slint::Image::load_from_path(&temp_render_path)?;
    ui.set_rendered_content(rendered_image);
    ui.set_line_count(metadata.len() as i32);

    // R4: Setup selection state tracking (RefCell for interior mutability in callbacks)
    let selection_state = RefCell::new(Option::<selection::SelectionRange_pal>::None);

    // Setup editor click handler for cursor positioning (R3) and selection (R4)
    let para_count = renderable_paragraphs.len() as i32;
    ui.on_editor_clicked(move |x, y| {
        // R3: Map click coordinates to document offset
        let line = (y / LINE_HEIGHT).floor() as usize;
        let col = (x / CHAR_WIDTH).floor() as usize;
        let offset = line * CHARS_PER_LINE + col;

        eprintln!("Click at ({}, {}) → offset={}", x, y, offset);
        eprintln!("  Paragraphs: {}", para_count);

        // R4: Start selection from click position
        // (Full drag-select would track mouse move events)
        *selection_state.borrow_mut() = Some(selection::SelectionRange_pal::new(offset, offset));
    });

    ui.set_cursor_line(0);
    ui.set_cursor_col(0);

    ui.run()?;
    Ok(())
}

/// Run the editor in MCP server mode (headless).
async fn run_mcp() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    // MCP server stub — implementation blocked on R1 completion
    eprintln!("MCP server mode not yet implemented");

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.contains(&"--mcp".to_string()) {
        run_mcp().await
    } else {
        run_gui()
    }
}
