/// Color scheme for markdown rendering.
/// 
/// Provides theme-aware colors for code blocks, inline code,
/// and reference chips.

pub struct MarkdownColors {
    pub code_bg: &'static str,
    pub code_fg: &'static str,
    pub code_border: &'static str,
    pub ref_bg: &'static str,
    pub ref_fg: &'static str,
    pub ref_border: &'static str,
}

impl MarkdownColors {
    /// Get colors for the specified theme
    pub fn for_theme(dark: bool) -> Self {
        if dark {
            Self {
                code_bg: "#1a1a2e",
                code_fg: "#e2e4f0",
                code_border: "rgba(255,255,255,0.08)",
                ref_bg: "rgba(124,157,255,0.12)",
                ref_fg: "#7c9dff",
                ref_border: "rgba(124,157,255,0.30)",
            }
        } else {
            Self {
                code_bg: "#f0f0f8",
                code_fg: "#1a1b2e",
                code_border: "rgba(0,0,0,0.08)",
                ref_bg: "rgba(79,110,247,0.10)",
                ref_fg: "#4f6ef7",
                ref_border: "rgba(79,110,247,0.28)",
            }
        }
    }
}
