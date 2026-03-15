/// Editor state — the adapter's central state owner.
pub mod editor_state_adp;
/// Editor state tests.
#[cfg(test)]
mod editor_state_adp_tests;
/// EditorApp — state, cache, cursor, render orchestrator.
pub mod editor_app_adp;
/// Keyboard input handling (handle_key impl on EditorApp_adp).
pub mod key_handler_adp;
/// Navigation key handling (arrow keys, home/end, backspace, delete).
pub mod key_nav_adp;
