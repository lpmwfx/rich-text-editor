// Inline element parsing and helpers for the Markdown parser.

use pulldown_cmark::{Event, Tag, TagEnd};

use super::{Inline};

/// Parse an inline element from the event stream.
/// Returns the parsed inline and next index.
pub fn parse_inline(events: &[(Event, std::ops::Range<usize>)], start: usize) -> (Inline, usize) {
    let (ref event, _) = events[start];

    match event {
        Event::Text(t) => (Inline::Text(t.to_string()), start + 1),

        Event::Code(t) => (Inline::Code(t.to_string()), start + 1),

        Event::Start(Tag::Emphasis) => {
            let mut children = Vec::new();
            let mut idx = start + 1;
            while idx < events.len() {
                match &events[idx].0 {
                    Event::End(TagEnd::Emphasis) => {
                        return (Inline::Italic(children), idx + 1);
                    }
                    _ => {
                        let (child, next) = parse_inline(events, idx);
                        children.push(child);
                        idx = next;
                    }
                }
            }
            (Inline::Italic(children), idx)
        }

        Event::Start(Tag::Strong) => {
            let mut children = Vec::new();
            let mut idx = start + 1;
            while idx < events.len() {
                match &events[idx].0 {
                    Event::End(TagEnd::Strong) => {
                        return (Inline::Bold(children), idx + 1);
                    }
                    _ => {
                        let (child, next) = parse_inline(events, idx);
                        children.push(child);
                        idx = next;
                    }
                }
            }
            (Inline::Bold(children), idx)
        }

        Event::Start(Tag::Link { dest_url, .. }) => {
            let url = dest_url.to_string();
            let mut text = String::new();
            let mut idx = start + 1;
            while idx < events.len() {
                match &events[idx].0 {
                    Event::End(TagEnd::Link) => {
                        return (Inline::Link { text, url }, idx + 1);
                    }
                    Event::Text(t) => {
                        text.push_str(t);
                        idx += 1;
                    }
                    _ => {
                        idx += 1;
                    }
                }
            }
            (Inline::Link { text, url }, idx)
        }

        Event::SoftBreak | Event::HardBreak => (Inline::Text("\n".into()), start + 1),

        _ => (Inline::Text(String::new()), start + 1),
    }
}

/// Check if a file path has a known video extension.
pub fn is_video_extension(path: &str) -> bool {
    let lower = path.to_lowercase();
    crate::shared::paths::VIDEO_EXTENSIONS
        .iter()
        .any(|ext| lower.ends_with(&format!(".{}", ext)))
}
