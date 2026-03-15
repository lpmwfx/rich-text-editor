// AST to Markdown string serializer.

use crate::core::document::serializer_block::serialize_block;
use crate::core::document::Document_core;

/// Serialize a Document to a Markdown string.
pub fn serialize(doc: &Document_core) -> String {
    let mut out = String::new();

    if let Some(ref fm) = doc.frontmatter {
        out.push_str("---\n");
        out.push_str(&fm.raw);
        if !fm.raw.ends_with('\n') {
            out.push('\n');
        }
        out.push_str("---\n\n");
    }

    for (i, block) in doc.blocks.iter().enumerate() {
        if i > 0 {
            out.push('\n');
        }
        serialize_block(block, &mut out);
        out.push('\n');
    }

    out
}

