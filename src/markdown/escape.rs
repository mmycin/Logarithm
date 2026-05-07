/// HTML escaping utilities.
/// 
/// Prevents XSS by escaping HTML special characters.

/// Escape a full string for safe HTML insertion
pub fn html_escape(s: &str) -> String {
    s.chars().map(html_escape_char).collect()
}

/// Escape a single character
pub fn html_escape_char(c: char) -> String {
    match c {
        '&' => "&amp;".into(),
        '<' => "&lt;".into(),
        '>' => "&gt;".into(),
        '"' => "&quot;".into(),
        '\'' => "&#39;".into(),
        c => c.to_string(),
    }
}
