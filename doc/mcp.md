# Editor MCP-interface

Denne fil beskriver hvordan rich text editoren eksponerer sig selv som en MCP-server.
Chatten foregår i Claude Desktop — editoren er et live preview- og mutations-target.
Bruger-godkendelse sker via chatten, ikke via editor-dialogs.

Se `CLAUDE.md` for overordnet projektarkitektur og build-kommandoer.

---

## Startmode

Editoren startes i MCP-mode med `--mcp` flag:

```bash
./target/release/rte --mcp
```

I MCP-mode:
- Al output til **stdout** er JSON-RPC (MCP-protokol) — skriv aldrig til stdout selv
- Al logging går til **stderr** via `tracing` med `.with_writer(std::io::stderr)`
- Slint GUI starter **ikke** — processen er headless fra Claude Desktops perspektiv
- Brugeren ser dokumentets tilstand i et **separat** editor-vindue (GUI-mode instans)

GUI-mode og MCP-mode deler state via en enkelt `Arc<Mutex<EditorState>>`.

---

## Crate og transport

```toml
[dependencies]
rmcp = { version = "0.16.0", features = ["server", "transport-io", "macros"] }
```

Transport er **stdio** — Claude Desktop spawner editoren som subprocess.

```rust
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.contains(&"--mcp".to_string()) {
        tracing_subscriber::fmt()
            .with_writer(std::io::stderr)
            .with_ansi(false)
            .init();
        let state = Arc::new(Mutex::new(EditorState::new()));
        let server = EditorMcpServer { state };
        let service = server.serve(rmcp::transport::stdio()).await?;
        service.waiting().await?;
    } else {
        run_gui()?;
    }
    Ok(())
}
```

---

## `.mcp.json` (projektroden)

```json
{
  "mcpServers": {
    "rich-text-editor": {
      "command": "./target/release/rte",
      "args": ["--mcp"],
      "env": {}
    }
  }
}
```

---

## Capabilities

```rust
ServerCapabilities::builder()
    .enable_tools()
    .enable_resources()
    .build()
```

Serveren eksponerer **tools** (mutations) og **resources** (læsning af dokumenttilstand).
Prompts er ikke implementeret — samtalen styres af Claude Desktop.

---

## Resources

### `document://current`

Det aktive dokuments fulde indhold som Markdown-streng.

Opdateres automatisk via `notify_resource_updated` efter enhver mutation.
Claude Desktop kan abonnere og re-læse ved ændringer uden eksplicit at spørge.

```rust
async fn read_resource(&self, req: ReadResourceRequestParams, ..)
    -> Result<ReadResourceResult, McpError>
{
    let state = self.state.lock().unwrap();
    let md = state.document.to_markdown();
    Ok(ReadResourceResult {
        contents: vec![ResourceContents::text(md, "document://current")],
        meta: None,
    })
}
```

### `document://file/{path}`

Læs en specifik `.md`-fil fra disk uden at åbne den i editoren.
Nyttigt til at inspicere relaterede filer under en session.

---

## Tools — overblik

| Tool | Formål |
|---|---|
| `get_document` | Hent aktivt dokument som Markdown eller AST-JSON |
| `open_document` | Åbn en `.md`-fil i editoren |
| `save_document` | Gem aktivt dokument til disk |
| `insert_text` | Indsæt rå tekst ved offset eller cursor |
| `replace_range` | Erstat tekstrange med nyt indhold |
| `delete_range` | Slet tekstrange |
| `format_range` | Anvend inline formatering på range |
| `insert_block` | Indsæt en hel blok (heading, codeblock, rule) |
| `insert_media` | Indlejr billede eller video som placeholder |
| `set_cursor` | Flyt cursor til offset (synkroniserer editor-UI) |
| `get_selection` | Hent aktuel selektion og dens tekst |
| `undo` | Fortryd seneste ændring |
| `redo` | Gentag fortryd ændring |
| `get_document_stats` | Ordantal, tegnantal, blokantal |

---

## Tools — detaljerede signaturer

### `get_document`

```rust
#[derive(Deserialize, JsonSchema)]
struct GetDocumentParams {
    /// "markdown" (default) eller "ast" — returnerer AST som JSON
    format: Option<String>,
}
```

Returnerer dokumentets indhold. Brug `"ast"` når du skal forstå struktur
(hvilke blokke findes, hvilke ranges svarer til hvad) inden du muterer.

---

### `open_document`

```rust
#[derive(Deserialize, JsonSchema)]
struct OpenDocumentParams {
    /// Absolut sti eller projektrelativ sti til .md fil
    path: String,
}
```

Åbner filen, parser til AST, nulstiller cursor og undo-stak.
Notificerer `document://current` resource.

---

### `save_document`

Ingen parametre — gemmer aktivt dokument til sin nuværende sti.
Fejler med `invalid_params` hvis intet dokument er åbnet.

---

### `insert_text`

```rust
#[derive(Deserialize, JsonSchema)]
struct InsertTextParams {
    /// Tekst der indsættes (kan indeholde Markdown inline-syntaks)
    text: String,
    /// UTF-8 byte-offset i dokumentets rå Markdown-streng.
    /// None = indsæt ved nuværende cursor-position.
    offset: Option<usize>,
}
```

Teksten indsættes som rå streng i AST'en via `InsertTextCommand`.
Markdown inline-syntaks i `text` parses **ikke** automatisk —
brug `insert_block` til strukturerede elementer.

---

### `replace_range`

```rust
#[derive(Deserialize, JsonSchema)]
struct ReplaceRangeParams {
    /// Start-offset (UTF-8 bytes, inklusiv)
    start: usize,
    /// Slut-offset (UTF-8 bytes, eksklusiv)
    end: usize,
    /// Erstatnings-tekst. Tom streng = sletning.
    replacement: String,
}
```

Den primære mutations-operation. Brug denne til omskrivning af afsnit.
Hent korrekte offsets via `get_document` med `format: "ast"` inden brug.

---

### `delete_range`

```rust
#[derive(Deserialize, JsonSchema)]
struct DeleteRangeParams {
    start: usize,
    end: usize,
}
```

Svarer til `replace_range` med tom `replacement`.
Adskilt for klarhedens skyld i tool-listen.

---

### `format_range`

```rust
#[derive(Deserialize, JsonSchema)]
struct FormatRangeParams {
    start: usize,
    end: usize,
    /// En af: "bold", "italic", "code",
    ///        "heading_1", "heading_2", "heading_3",
    ///        "remove" (fjerner al formatering i range)
    format: String,
}
```

Anvender inline-formatering via `FormatRangeCommand`.
`heading_*` konverterer den blok der indeholder range til en heading.
`"remove"` stripper Bold/Italic/Code wrappers og returnerer til plain Text.

---

### `insert_block`

```rust
#[derive(Deserialize, JsonSchema)]
struct InsertBlockParams {
    /// "paragraph", "heading", "code_block", "horizontal_rule", "bullet_list"
    block_type: String,
    /// Tekstindhold (ignoreres for "horizontal_rule")
    content: Option<String>,
    /// Niveau 1-3 (kun relevant for "heading")
    level: Option<u8>,
    /// Sprog til syntaks-highlight (kun relevant for "code_block")
    language: Option<String>,
    /// UTF-8 offset hvor blokken indsættes. None = efter nuværende blok.
    offset: Option<usize>,
}
```

Indsætter en struktureret blok i AST'en direkte —
parser ikke Markdown-syntaks i `content`.

---

### `insert_media`

```rust
#[derive(Deserialize, JsonSchema)]
struct InsertMediaParams {
    /// Sti til fil relativ til projektmappen (bruges i Markdown som ![alt](path))
    path: String,
    /// Alt-tekst / beskrivelse
    alt: String,
    /// Valgfri billedtekst under mediet
    caption: Option<String>,
    /// UTF-8 offset. None = indsæt ved cursor.
    offset: Option<usize>,
}
```

Indsætter `Block::Image` eller `Block::Video` afhængigt af filendelse.
Kendte video-endelser: `.mp4`, `.webm`, `.mov`, `.mkv`.
Alt andet behandles som billede.

Mediet registreres i `MediaManager` og tildeles et placeholder-index
til skparagraph-rendering. Stien gemmes as-is i Markdown: `![alt](path)`.

---

### `set_cursor`

```rust
#[derive(Deserialize, JsonSchema)]
struct SetCursorParams {
    /// UTF-8 byte-offset i dokumentet
    offset: usize,
}
```

Synkroniserer editor-UI'ets cursor-position.
Nyttigt inden `insert_text` uden explicit offset,
eller for at vise brugeren hvad Claude er ved at redigere.

---

### `get_selection`

Ingen parametre. Returnerer JSON:

```json
{
  "start": 142,
  "end": 198,
  "text": "den valgte tekst"
}
```

Returnerer `null` hvis ingen selektion er aktiv.

---

### `undo`

Ingen parametre. Fortryder den seneste operation på undo-stakken.
Notificerer `document://current` resource.

Primær recovery-mekanisme — bruges når brugeren siger "fortryd" i chatten.

---

### `redo`

Ingen parametre. Gentager en fortryd operation.

---

### `get_document_stats`

Ingen parametre. Returnerer JSON:

```json
{
  "words": 847,
  "characters": 4821,
  "characters_no_spaces": 4102,
  "blocks": 23,
  "images": 3,
  "videos": 0,
  "headings": { "h1": 1, "h2": 4, "h3": 6 }
}
```

---

## Live preview — hvad brugeren ser

Editoren highlighter blokke der lige er blevet muteret via MCP.
Highlighten fades ud efter ~2 sekunder (amber baggrund via Slint animation).

```slint
component BlockView inherits Rectangle {
    in property <bool> recently-changed;
    background: recently-changed ? #EF9F2720 : transparent;
    animate background { duration: 2000ms; easing: ease-out; }
}
```

Brugeren ser ændringer ske live i editor-vinduet mens Claude arbejder.
Der er **ingen godkendelses-dialog** i editoren — samtalen styres i Claude Desktop.
Undo-stakken er safety net: brugeren siger "fortryd" i chatten → Claude kalder `undo`.

---

## Intern signalering (MCP-lag → Slint UI)

MCP-server og Slint UI kører i samme process og kommunikerer via `tokio::sync::mpsc`:

```rust
pub enum UiEvent {
    DocumentChanged,          // re-render hele dokumentet
    BlockChanged(usize),      // re-render specifik blok (block-index)
    CursorMoved(usize),       // synkroniser cursor i UI
    MediaLoaded(usize),       // placeholder klar til at vise thumbnail
}
```

MCP-laget sender events til Slint via `ui_tx.send(event).await`.
Slint-laget modtager via `slint::invoke_from_event_loop` fra en `tokio::spawn` task.

---

## Resource notification

Efter enhver mutation notificeres Claude Desktop:

```rust
async fn notify_changed(&self, peer: &Peer) {
    let _ = peer.notify_resource_updated(ResourceUpdatedNotificationParams {
        uri: "document://current".into(),
        ..Default::default()
    }).await;
}
```

Dette giver Claude Desktop mulighed for at holde sin kontekst frisk
uden at hvert tool-svar behøver at inkludere hele dokumentet.

---

## Fejlhåndtering

| Situation | MCP fejltype |
|---|---|
| Offset uden for dokument | `invalid_params` |
| Ukendt format-streng | `invalid_params` |
| Intet åbent dokument | `invalid_params` |
| Fil ikke fundet | `internal_error` med OS-fejl |
| Undo-stak tom | returnerer OK med besked "Intet at fortryde" |
| Mediefil ukendt type | returnerer OK, behandles som billede |

Alle tool-handlers returnerer `Result<String, rmcp::Error>`.
Fejl-beskeder er på dansk og menneskelæsbare — de vises i Claude Desktops chat.

---

## Offset-konvention

Alle offsets er **UTF-8 byte-offsets** i dokumentets rå Markdown-repræsentation
(dvs. `Document::to_markdown()` output, ikke AST-interne indices).

Hent korrekte offsets ved at kalde `get_document` med `format: "ast"` —
AST-responsen inkluderer `byte_range: [start, end]` per blok og inline-span.

Eksempel AST-output:
```json
{
  "blocks": [
    {
      "type": "Heading",
      "level": 1,
      "byte_range": [0, 18],
      "content": "Min overskrift"
    },
    {
      "type": "Paragraph",
      "byte_range": [19, 142],
      "inlines": [...]
    }
  ]
}
```

---

## Anbefalede arbejdsgange

### Omskriv et afsnit

```
1. get_document(format: "ast")          → find blokken og dens byte_range
2. set_cursor(offset: blok.start)       → vis brugeren hvad der redigeres
3. replace_range(start, end, ny_tekst)  → udfør erstatningen
```

### Tilføj et nyt afsnit til sidst

```
1. get_document_stats()                 → find dokumentets længde
2. insert_block(type: "paragraph", content: "...", offset: None)
```

### Fortryd hvis brugeren er utilfreds

```
1. undo()    → fortryder seneste operation
2. undo()    → kald igen for at gå længere tilbage (ingen grænse)
```

### Inspicér inden store ændringer

```
1. get_document(format: "markdown")    → læs hele dokumentet
2. get_document_stats()                → overblik over struktur
3. ...planlæg ændringer...
4. udfør mutations-tools i rækkefølge
```
