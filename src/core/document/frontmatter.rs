// YAML frontmatter parsing and serialization.

use crate::shared::sizes::{FRONTMATTER_DELIM, FRONTMATTER_DELIM_LEN};

use crate::shared::document_types_x::Frontmatter_x as Frontmatter;

/// Extract frontmatter from a Markdown string.
/// Returns (Option<Frontmatter>, body_without_frontmatter).
pub fn extract(input: &str) -> (Option<Frontmatter>, &str) {
    if !input.starts_with(FRONTMATTER_DELIM) {
        return (None, input);
    }

    // Find the closing --- delimiter (must be on its own line).
    let after_opening = &input[FRONTMATTER_DELIM_LEN..];
    let after_opening = after_opening.strip_prefix('\n').unwrap_or(after_opening);

    if let Some(end_pos) = find_closing_delimiter(after_opening) {
        let raw = &after_opening[..end_pos];
        let rest_start = end_pos + FRONTMATTER_DELIM_LEN;
        let rest = &after_opening[rest_start..];
        let rest = rest.strip_prefix('\n').unwrap_or(rest);

        (
            Some(Frontmatter {
                raw: raw.to_string(),
            }),
            rest,
        )
    } else {
        (None, input)
    }
}

/// Find the position of the closing --- delimiter.
fn find_closing_delimiter(input: &str) -> Option<usize> {
    let mut pos = 0;
    for line in input.lines() {
        if line.trim() == FRONTMATTER_DELIM {
            return Some(pos);
        }
        pos += line.len() + 1;
    }
    None
}

/// Format frontmatter for inclusion in a Markdown string.
pub fn format(fm: &Frontmatter) -> String {
    let mut out = String::from("---\n");
    out.push_str(&fm.raw);
    if !fm.raw.ends_with('\n') {
        out.push('\n');
    }
    out.push_str("---\n");
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_frontmatter() {
        let input = "---\ntitle: Hello\n---\nBody text";
        let (fm, body) = extract(input);
        assert!(fm.is_some());
        assert_eq!(fm.unwrap().raw, "title: Hello\n");
        assert_eq!(body, "Body text");
    }

    #[test]
    fn no_frontmatter() {
        let input = "Just a paragraph";
        let (fm, body) = extract(input);
        assert!(fm.is_none());
        assert_eq!(body, input);
    }

    #[test]
    fn unclosed_frontmatter() {
        let input = "---\ntitle: Oops\nNo closing delimiter";
        let (fm, body) = extract(input);
        assert!(fm.is_none());
        assert_eq!(body, input);
    }

    #[test]
    fn format_frontmatter() {
        let fm = Frontmatter {
            raw: "title: Test\n".into(),
        };
        let formatted = format(&fm);
        assert_eq!(formatted, "---\ntitle: Test\n---\n");
    }
}
