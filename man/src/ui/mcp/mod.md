# `src/ui/mcp/mod.rs`

## `pub struct GetDocumentParams`
*Line 21 · struct*

Parameters for get_document tool.

---

## `pub struct OpenDocumentParams`
*Line 28 · struct*

Parameters for open_document tool.

---

## `pub struct InsertTextParams`
*Line 35 · struct*

Parameters for insert_text tool.

---

## `pub struct ReplaceRangeParams`
*Line 44 · struct*

Parameters for replace_range tool.

---

## `pub struct DeleteRangeParams`
*Line 55 · struct*

Parameters for delete_range tool.

---

## `pub struct InsertBlockParams`
*Line 64 · struct*

Parameters for insert_block tool.

---

## `pub struct SetCursorParams`
*Line 79 · struct*

Parameters for set_cursor tool.

---

## `pub struct EditorMcpServer`
*Line 86 · struct*

MCP server wrapping the editor state.

---

## `pub fn new() -> Self`
*Line 96 · fn*

Create a new MCP server with a fresh editor state.

---

## `pub fn with_state(state: Arc<Mutex<EditorState>>) -> Self`
*Line 104 · fn*

Create a new MCP server with existing state.

---



---

<!-- LARS:START -->
<a href="https://lpmathiasen.com">
  <img src="https://carousel.lpmathiasen.com/carousel.svg?slot=4" alt="Lars P. Mathiasen"/>
</a>
<!-- LARS:END -->
