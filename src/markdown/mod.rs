/// Markdown to HTML converter module.
/// 
/// Provides a minimal markdown parser that converts markdown syntax
/// to styled HTML for display in the AI chat interface.
/// 
/// Supports: code blocks, inline code, bold, italic, headers,
/// bullet lists, and context reference chips.

mod colors;
mod escape;
mod inline;
mod parser;

pub use parser::md_to_html;
