/// Main markdown parser.
/// 
/// Converts markdown text to styled HTML with support for:
/// - Fenced code blocks (```)
/// - Headers (# ## ###)
/// - Bullet lists (- *)
/// - Context reference chips [file:line]
/// - Inline formatting (via inline module)

use super::colors::MarkdownColors;
use super::escape::html_escape;
use super::inline::inline_md;

/// Convert markdown to HTML
/// 
/// # Arguments
/// * `input` - The markdown text to convert
/// * `dark` - Whether to use dark theme colors
/// 
/// # Returns
/// HTML string safe for innerHTML
pub fn md_to_html(input: &str, dark: bool) -> String {
    let colors = MarkdownColors::for_theme(dark);
    let mut out = String::new();
    let mut state = ParserState::default();

    for line in input.lines() {
        process_line(line, &mut out, &mut state, &colors, dark);
    }

    finalize_output(&mut out, &state, &colors);
    out
}

#[derive(Default)]
struct ParserState {
    in_code_block: bool,
    code_buf: String,
    in_list: bool,
}

fn process_line(
    line: &str,
    out: &mut String,
    state: &mut ParserState,
    colors: &MarkdownColors,
    dark: bool,
) {
    // Handle code block fences
    if line.starts_with("```") {
        handle_code_fence(out, state, colors);
        return;
    }

    // Accumulate code block content
    if state.in_code_block {
        if !state.code_buf.is_empty() {
            state.code_buf.push('\n');
        }
        state.code_buf.push_str(line);
        return;
    }

    let trimmed = line.trim();

    // Context reference chip
    if is_ref_chip(trimmed) {
        close_list_if_needed(out, state);
        render_ref_chip(out, trimmed, colors);
        return;
    }

    // Headers
    if let Some(html) = try_header(trimmed, dark) {
        close_list_if_needed(out, state);
        out.push_str(&html);
        return;
    }

    // Bullet list
    if trimmed.starts_with("- ") || trimmed.starts_with("* ") {
        if !state.in_list {
            out.push_str("<ul style=\"margin:4px 0;padding-left:18px;\">");
            state.in_list = true;
        }
        let item = &trimmed[2..];
        out.push_str(&format!("<li style=\"margin:2px 0;\">{}</li>", inline_md(item, dark)));
        return;
    }

    close_list_if_needed(out, state);

    // Empty line
    if trimmed.is_empty() {
        out.push_str("<br/>");
        return;
    }

    // Normal paragraph
    out.push_str(&format!("<span style=\"display:block;\">{}</span>", inline_md(trimmed, dark)));
}

fn handle_code_fence(out: &mut String, state: &mut ParserState, colors: &MarkdownColors) {
    if state.in_code_block {
        let escaped = html_escape(&state.code_buf);
        out.push_str(&format!(
            "<pre style=\"background:{};border:1px solid {};\
             border-radius:6px;padding:10px 12px;margin:6px 0;overflow-x:auto;\
             font-family:'Fira Code',monospace;font-size:11.5px;color:{};\
             line-height:1.55;white-space:pre;\">{}</pre>",
            colors.code_bg, colors.code_border, colors.code_fg, escaped
        ));
        state.code_buf.clear();
        state.in_code_block = false;
    } else {
        state.in_code_block = true;
    }
}

fn is_ref_chip(trimmed: &str) -> bool {
    trimmed.starts_with('[') && trimmed.ends_with(']') && trimmed[1..trimmed.len() - 1].contains(':')
}

fn render_ref_chip(out: &mut String, trimmed: &str, colors: &MarkdownColors) {
    let inner = &trimmed[1..trimmed.len() - 1];
    out.push_str(&format!(
        "<div style=\"display:inline-flex;align-items:center;gap:5px;\
         padding:3px 9px;border-radius:5px;font-size:11px;font-weight:600;\
         font-family:'Fira Code',monospace;background:{};\
         color:{};border:1px solid {};margin:3px 0;\">\
         <img src='/public/LoganIcon.png' width='10' height='10' style='border-radius:2px;opacity:0.6' alt='Logan' />{}</div>",
        colors.ref_bg, colors.ref_fg, colors.ref_border, html_escape(inner)
    ));
}

fn try_header(trimmed: &str, dark: bool) -> Option<String> {
    if let Some(rest) = trimmed.strip_prefix("### ") {
        return Some(format!("<h3 style=\"font-size:13px;font-weight:700;margin:10px 0 4px;\">{}</h3>", inline_md(rest, dark)));
    }
    if let Some(rest) = trimmed.strip_prefix("## ") {
        return Some(format!("<h2 style=\"font-size:14px;font-weight:700;margin:12px 0 4px;\">{}</h2>", inline_md(rest, dark)));
    }
    if let Some(rest) = trimmed.strip_prefix("# ") {
        return Some(format!("<h1 style=\"font-size:15px;font-weight:700;margin:12px 0 4px;\">{}</h1>", inline_md(rest, dark)));
    }
    None
}

fn close_list_if_needed(out: &mut String, state: &mut ParserState) {
    if state.in_list {
        out.push_str("</ul>");
        state.in_list = false;
    }
}

fn finalize_output(out: &mut String, state: &ParserState, colors: &MarkdownColors) {
    if state.in_list {
        out.push_str("</ul>");
    }

    if state.in_code_block && !state.code_buf.is_empty() {
        let escaped = html_escape(&state.code_buf);
        out.push_str(&format!(
            "<pre style=\"background:{};border:1px solid {};\
             border-radius:6px;padding:10px 12px;margin:6px 0;overflow-x:auto;\
             font-family:'Fira Code',monospace;font-size:11.5px;color:{};\
             line-height:1.55;white-space:pre;\">{}</pre>",
            colors.code_bg, colors.code_border, colors.code_fg, escaped
        ));
    }
}
