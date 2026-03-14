/// Rich text editor — GUI mode or MCP server mode.

slint::include_modules!();

use rich_text_editor::core::document::Document;
use rich_text_editor::pal::build_paragraphs;

/// Editor canvas width in pixels
const EDITOR_WIDTH: f32 = 800.0;

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

    // Build Skia paragraphs (R1 — real textlayout implementation)
    let _paragraphs = build_paragraphs(&doc, EDITOR_WIDTH)?;

    // Placeholder display until R2 (render notifier callback) wires output to canvas
    ui.set_cursor_line(0);
    ui.set_cursor_col(0);

    ui.on_editor_clicked(|x, y| {
        eprintln!("Click at ({}, {})", x, y);
    });

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
