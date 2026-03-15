#![allow(non_camel_case_types)]
/// Block type discriminator for MCP insert_block operations.

/// Wire name for paragraph blocks.
const PARAGRAPH: &str = "paragraph";
/// Wire name for heading blocks.
const HEADING: &str = "heading";
/// Wire name for code blocks.
const CODE_BLOCK: &str = "code_block";
/// Wire name for horizontal rules.
const HORIZONTAL_RULE: &str = "horizontal_rule";
/// Wire name for bullet lists.
const BULLET_LIST: &str = "bullet_list";

/// Supported block types for insertion via MCP.
pub enum BlockKind_ui {
    /// A paragraph block.
    Paragraph,
    /// A heading block (with level).
    Heading,
    /// A fenced code block.
    CodeBlock,
    /// A horizontal rule.
    HorizontalRule,
    /// A bullet list.
    BulletList,
}

impl BlockKind_ui {
    /// Parse a block type string into a BlockKind.
    pub fn parse(s: &str) -> Result<Self, String> {
        match s {
            PARAGRAPH => Ok(Self::Paragraph),
            HEADING => Ok(Self::Heading),
            CODE_BLOCK => Ok(Self::CodeBlock),
            HORIZONTAL_RULE => Ok(Self::HorizontalRule),
            BULLET_LIST => Ok(Self::BulletList),
            other => Err(format!("unknown block type '{}'", other)),
        }
    }
}
