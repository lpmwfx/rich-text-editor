/// Rich text editor — GUI mode or MCP server mode.

/// Run the editor in GUI mode with Slint UI.
fn run_gui() -> anyhow::Result<()> {
    // TODO: initialize Slint AppWindow via slint-ui-templates adapter
    Ok(())
}

/// Run the editor as a headless MCP server (stdio transport).
async fn run_mcp() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();
    // TODO: create EditorState, start MCP server
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.contains(&"--mcp".to_string()) {
        let rt = tokio::runtime::Runtime::new()?;
        rt.block_on(run_mcp())?;
    } else {
        run_gui()?;
    }
    Ok(())
}
