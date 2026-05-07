/// Inline markdown processing.
/// 
/// Handles inline markdown syntax: `code`, **bold**, *italic*,
/// and [ref:line] context chips.

use super::colors::MarkdownColors;
use super::escape::{html_escape, html_escape_char};

/// Process inline markdown syntax
pub fn inline_md(s: &str, dark: bool) -> String {
    let colors = MarkdownColors::for_theme(dark);
    let mut out = String::new();
    let chars: Vec<char> = s.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        if let Some((html, skip)) = try_inline_code(&chars, i, &colors) {
            out.push_str(&html);
            i += skip;
        } else if let Some((html, skip)) = try_ref_chip(&chars, i, &colors) {
            out.push_str(&html);
            i += skip;
        } else if let Some((html, skip)) = try_bold(&chars, i) {
            out.push_str(&html);
            i += skip;
        } else if let Some((html, skip)) = try_italic(&chars, i) {
            out.push_str(&html);
            i += skip;
        } else {
            out.push_str(&html_escape_char(chars[i]));
            i += 1;
        }
    }

    out
}

/// Try to parse inline code: `...`
fn try_inline_code(chars: &[char], i: usize, colors: &MarkdownColors) -> Option<(String, usize)> {
    if chars[i] != '`' {
        return None;
    }
    
    let end = chars[i + 1..].iter().position(|&c| c == '`')?;
    let code: String = chars[i + 1..i + 1 + end].iter().collect();
    
    let html = format!(
        "<code style=\"background:{};color:{};\
         border:1px solid {};border-radius:3px;\
         padding:1px 5px;font-family:'Fira Code',monospace;font-size:11px;\">{}</code>",
        colors.code_bg, colors.code_fg, colors.code_border, html_escape(&code)
    );
    
    Some((html, end + 2))
}

/// Try to parse reference chip: [file:line]
fn try_ref_chip(chars: &[char], i: usize, colors: &MarkdownColors) -> Option<(String, usize)> {
    if chars[i] != '[' {
        return None;
    }
    
    let end = chars[i + 1..].iter().position(|&c| c == ']')?;
    let inner: String = chars[i + 1..i + 1 + end].iter().collect();
    
    if !inner.contains(':') {
        return None;
    }
    
    let html = format!(
        "<span style=\"display:inline-flex;align-items:center;gap:4px;\
         padding:1px 7px;border-radius:4px;font-size:10.5px;font-weight:600;\
         font-family:'Fira Code',monospace;background:{};\
         color:{};border:1px solid {};\">{}</span>",
        colors.ref_bg, colors.ref_fg, colors.ref_border, html_escape(&inner)
    );
    
    Some((html, end + 2))
}

/// Try to parse bold: **...**
fn try_bold(chars: &[char], i: usize) -> Option<(String, usize)> {
    if i + 1 >= chars.len() || chars[i] != '*' || chars[i + 1] != '*' {
        return None;
    }
    
    let end = find_pair(chars, i + 2, "**")?;
    let inner: String = chars[i + 2..end].iter().collect();
    let html = format!("<strong>{}</strong>", html_escape(&inner));
    
    Some((html, end + 2 - i))
}

/// Try to parse italic: *...*
fn try_italic(chars: &[char], i: usize) -> Option<(String, usize)> {
    if chars[i] != '*' {
        return None;
    }
    
    let end = chars[i + 1..].iter().position(|&c| c == '*')?;
    let inner: String = chars[i + 1..i + 1 + end].iter().collect();
    let html = format!("<em>{}</em>", html_escape(&inner));
    
    Some((html, end + 2))
}

/// Find matching pattern in character array
fn find_pair(chars: &[char], start: usize, pat: &str) -> Option<usize> {
    let pat_chars: Vec<char> = pat.chars().collect();
    let plen = pat_chars.len();
    
    for i in start..chars.len().saturating_sub(plen - 1) {
        if chars[i..i + plen] == pat_chars[..] {
            return Some(i);
        }
    }
    
    None
}
