// Block to skparagraph ParagraphBuilder mapping.
// TODO: Implement skparagraph integration when API is stable.
// For now, this is a placeholder that builds paragraphs metadata.

use crate::core::document::{Block, Document};

/// A paragraph ready for Skia textlayout rendering.
#[derive(Debug)]
pub struct StyledParagraph {
    pub block_type: String,
    pub text: String,
    pub width: f32,
    pub height: f32,
}

/// Convert a Document into paragraphs for Skia rendering.
pub fn build_paragraphs(doc: &Document, available_width: f32) -> anyhow::Result<Vec<StyledParagraph>> {
    let mut paragraphs = Vec::new();

    for block in &doc.blocks {
        match block {
            Block::Heading { level, content, .. } => {
                let text = format!("Heading {}: {:?}", level, content);
                let height = 24.0 * (*level as f32);
                paragraphs.push(StyledParagraph {
                    block_type: format!("heading-{}", level),
                    text,
                    width: available_width,
                    height,
                });
            }

            Block::Paragraph { content, .. } => {
                let text = format!("{:?}", content);
                paragraphs.push(StyledParagraph {
                    block_type: "paragraph".to_string(),
                    text,
                    width: available_width,
                    height: 16.0,
                });
            }

            Block::CodeBlock { code, .. } => {
                let lines = code.lines().count();
                let height = 14.0 * lines as f32;
                paragraphs.push(StyledParagraph {
                    block_type: "code-block".to_string(),
                    text: code.clone(),
                    width: available_width,
                    height,
                });
            }

            Block::BulletList { items, .. } => {
                for item in items {
                    let text = format!("• {:?}", item);
                    paragraphs.push(StyledParagraph {
                        block_type: "list-item".to_string(),
                        text,
                        width: available_width,
                        height: 16.0,
                    });
                }
            }

            Block::HorizontalRule { .. } => {
                paragraphs.push(StyledParagraph {
                    block_type: "rule".to_string(),
                    text: "―――――――".to_string(),
                    width: available_width,
                    height: 16.0,
                });
            }

            Block::Image { alt, path, .. } => {
                paragraphs.push(StyledParagraph {
                    block_type: "image".to_string(),
                    text: format!("[image: {}]({})", alt, path),
                    width: available_width,
                    height: 200.0,
                });
            }

            Block::Video { path, .. } => {
                paragraphs.push(StyledParagraph {
                    block_type: "video".to_string(),
                    text: format!("[video: {}]", path),
                    width: available_width,
                    height: 300.0,
                });
            }
        }
    }

    Ok(paragraphs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_paragraphs() {
        let doc = Document::from_markdown("# Hello\n\nWorld");
        let paragraphs = build_paragraphs(&doc, 800.0).expect("should build");
        assert!(!paragraphs.is_empty());
    }
}
