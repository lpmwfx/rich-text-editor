/// Rich text editor — GUI mode or MCP server mode.

slint::include_modules!();

use rich_text_editor::ui::mcp::EditorMcpServer;
use rmcp::ServiceExt;

/// Run the editor in GUI mode with Slint UI.
fn run_gui() -> anyhow::Result<()> {
    let ui = AppWindow::new()?;

    ui.set_status_text("Rich Text Editor".into());
    ui.set_nav_items(slint::ModelRc::new(slint::VecModel::from(vec![
        NavItem {
            id: "editor".into(),
            label: "Editor".into(),
            icon: "\u{E70F}".into(),
            is_header: false,
            hidden: false,
        },
    ])));
    ui.set_active_view("editor".into());

    ui.run()?;
    Ok(())
}

/// Run the editor as a headless MCP server (stdio transport).
async fn run_mcp() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    let server = EditorMcpServer::new();
    let service = server.serve(rmcp::transport::stdio()).await?;
    service.waiting().await?;
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
