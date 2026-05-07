/// Minimal Markdown → HTML string converter.
/// Supports: code blocks, inline code, bold, italic, headers (h1-h3), bullet lists, line breaks.
/// Returns an HTML string safe to set via innerHTML.
pub fn md_to_html(input: &str, dark: bool) -> String {
    let code_bg   = if dark { "#1a1a2e" } else { "#f0f0f8" };
    let code_fg   = if dark { "#e2e4f0" } else { "#1a1b2e" };
    let code_border = if dark { "rgba(255,255,255,0.08)" } else { "rgba(0,0,0,0.08)" };
    let ref_bg    = if dark { "rgba(124,157,255,0.12)" } else { "rgba(79,110,247,0.10)" };
    let ref_fg    = if dark { "#7c9dff" } else { "#4f6ef7" };
    let ref_border = if dark { "rgba(124,157,255,0.30)" } else { "rgba(79,110,247,0.28)" };

    let mut out = String::new();
    let mut in_code_block = false;
    let mut code_lang = String::new();
    let mut code_buf  = String::new();
    let mut in_list   = false;

    for line in input.lines() {
        // ── Fenced code block ─────────────────────────────────────────────
        if line.starts_with("```") {
            if in_code_block {
                // Close block
                let escaped = html_escape(&code_buf);
                out.push_str(&format!(
                    "<pre style=\"background:{code_bg};border:1px solid {code_border};\
                     border-radius:6px;padding:10px 12px;margin:6px 0;overflow-x:auto;\
                     font-family:'Fira Code',monospace;font-size:11.5px;color:{code_fg};\
                     line-height:1.55;white-space:pre;\">{escaped}</pre>"
                ));
                code_buf.clear();
                code_lang.clear();
                in_code_block = false;
            } else {
                if in_list { out.push_str("</ul>"); in_list = false; }
                code_lang = line[3..].trim().to_string();
                in_code_block = true;
            }
            continue;
        }

        if in_code_block {
            if !code_buf.is_empty() { code_buf.push('\n'); }
            code_buf.push_str(line);
            continue;
        }

        // ── Context reference chip [file:line] ────────────────────────────
        // Render lines that are purely a [file:line] reference as a styled chip
        let trimmed = line.trim();
        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            let inner = &trimmed[1..trimmed.len()-1];
            if inner.contains(':') {
                if in_list { out.push_str("</ul>"); in_list = false; }
                out.push_str(&format!(
                    "<div style=\"display:inline-flex;align-items:center;gap:5px;\
                     padding:3px 9px;border-radius:5px;font-size:11px;font-weight:600;\
                     font-family:'Fira Code',monospace;background:{ref_bg};\
                     color:{ref_fg};border:1px solid {ref_border};margin:3px 0;\">
                     <svg width='10' height='10' viewBox='0 0 16 16' fill='currentColor'>\
                     <path d='M7.657 6.247c.11-.33.576-.33.686 0l.645 1.937a2.89 2.89 0 0 0 1.829 1.828l1.936.645c.33.11.33.576 0 .686l-1.937.645a2.89 2.89 0 0 0-1.828 1.829l-.645 1.936a.361.361 0 0 1-.686 0l-.645-1.937a2.89 2.89 0 0 0-1.828-1.828l-1.937-.645a.361.361 0 0 1 0-.686l1.937-.645a2.89 2.89 0 0 0 1.828-1.828l.645-1.937z'/>\
                     </svg>{}</div>",
                    html_escape(inner)
                ));
                continue;
            }
        }

        // ── Headers ───────────────────────────────────────────────────────
        if let Some(rest) = trimmed.strip_prefix("### ") {
            if in_list { out.push_str("</ul>"); in_list = false; }
            out.push_str(&format!("<h3 style=\"font-size:13px;font-weight:700;margin:10px 0 4px;\">{}</h3>", inline_md(rest, dark)));
            continue;
        }
        if let Some(rest) = trimmed.strip_prefix("## ") {
            if in_list { out.push_str("</ul>"); in_list = false; }
            out.push_str(&format!("<h2 style=\"font-size:14px;font-weight:700;margin:12px 0 4px;\">{}</h2>", inline_md(rest, dark)));
            continue;
        }
        if let Some(rest) = trimmed.strip_prefix("# ") {
            if in_list { out.push_str("</ul>"); in_list = false; }
            out.push_str(&format!("<h1 style=\"font-size:15px;font-weight:700;margin:12px 0 4px;\">{}</h1>", inline_md(rest, dark)));
            continue;
        }

        // ── Bullet list ───────────────────────────────────────────────────
        if trimmed.starts_with("- ") || trimmed.starts_with("* ") {
            if !in_list {
                out.push_str("<ul style=\"margin:4px 0;padding-left:18px;\">");
                in_list = true;
            }
            let item = &trimmed[2..];
            out.push_str(&format!("<li style=\"margin:2px 0;\">{}</li>", inline_md(item, dark)));
            continue;
        }

        // Close list if we hit a non-list line
        if in_list { out.push_str("</ul>"); in_list = false; }

        // ── Empty line → paragraph break ──────────────────────────────────
        if trimmed.is_empty() {
            out.push_str("<br/>");
            continue;
        }

        // ── Normal paragraph line ─────────────────────────────────────────
        out.push_str(&format!("<span style=\"display:block;\">{}</span>", inline_md(trimmed, dark)));
    }

    if in_list { out.push_str("</ul>"); }

    // Unclosed code block
    if in_code_block && !code_buf.is_empty() {
        let escaped = html_escape(&code_buf);
        out.push_str(&format!(
            "<pre style=\"background:{code_bg};border:1px solid {code_border};\
             border-radius:6px;padding:10px 12px;margin:6px 0;overflow-x:auto;\
             font-family:'Fira Code',monospace;font-size:11.5px;color:{code_fg};\
             line-height:1.55;white-space:pre;\">{escaped}</pre>"
        ));
    }

    out
}

/// Process inline markdown: `code`, **bold**, *italic*, [ref:line] chips.
fn inline_md(s: &str, dark: bool) -> String {
    let code_bg     = if dark { "#1a1a2e" } else { "#f0f0f8" };
    let code_fg     = if dark { "#e2e4f0" } else { "#1a1b2e" };
    let code_border = if dark { "rgba(255,255,255,0.08)" } else { "rgba(0,0,0,0.08)" };
    let ref_bg      = if dark { "rgba(124,157,255,0.12)" } else { "rgba(79,110,247,0.10)" };
    let ref_fg      = if dark { "#7c9dff" } else { "#4f6ef7" };
    let ref_border  = if dark { "rgba(124,157,255,0.30)" } else { "rgba(79,110,247,0.28)" };

    let mut out = String::new();
    let chars: Vec<char> = s.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        // Inline code: `...`
        if chars[i] == '`' {
            if let Some(end) = chars[i+1..].iter().position(|&c| c == '`') {
                let code: String = chars[i+1..i+1+end].iter().collect();
                out.push_str(&format!(
                    "<code style=\"background:{code_bg};color:{code_fg};\
                     border:1px solid {code_border};border-radius:3px;\
                     padding:1px 5px;font-family:'Fira Code',monospace;font-size:11px;\">{}</code>",
                    html_escape(&code)
                ));
                i += end + 2;
                continue;
            }
        }

        // [ref:line] inline chip
        if chars[i] == '[' {
            if let Some(end) = chars[i+1..].iter().position(|&c| c == ']') {
                let inner: String = chars[i+1..i+1+end].iter().collect();
                if inner.contains(':') {
                    out.push_str(&format!(
                        "<span style=\"display:inline-flex;align-items:center;gap:4px;\
                         padding:1px 7px;border-radius:4px;font-size:10.5px;font-weight:600;\
                         font-family:'Fira Code',monospace;background:{ref_bg};\
                         color:{ref_fg};border:1px solid {ref_border};\">{}</span>",
                        html_escape(&inner)
                    ));
                    i += end + 2;
                    continue;
                }
            }
        }

        // Bold: **...**
        if i + 1 < len && chars[i] == '*' && chars[i+1] == '*' {
            if let Some(end) = find_pair(&chars, i+2, "**") {
                let inner: String = chars[i+2..end].iter().collect();
                out.push_str(&format!("<strong>{}</strong>", html_escape(&inner)));
                i = end + 2;
                continue;
            }
        }

        // Italic: *...*
        if chars[i] == '*' {
            if let Some(end) = chars[i+1..].iter().position(|&c| c == '*') {
                let inner: String = chars[i+1..i+1+end].iter().collect();
                out.push_str(&format!("<em>{}</em>", html_escape(&inner)));
                i += end + 2;
                continue;
            }
        }

        // Plain character
        out.push_str(&html_escape_char(chars[i]));
        i += 1;
    }

    out
}

fn find_pair(chars: &[char], start: usize, pat: &str) -> Option<usize> {
    let pat_chars: Vec<char> = pat.chars().collect();
    let plen = pat_chars.len();
    for i in start..chars.len().saturating_sub(plen - 1) {
        if chars[i..i+plen] == pat_chars[..] {
            return Some(i);
        }
    }
    None
}

fn html_escape(s: &str) -> String {
    s.chars().map(html_escape_char).collect()
}

fn html_escape_char(c: char) -> String {
    match c {
        '&'  => "&amp;".into(),
        '<'  => "&lt;".into(),
        '>'  => "&gt;".into(),
        '"'  => "&quot;".into(),
        '\'' => "&#39;".into(),
        c    => c.to_string(),
    }
}
