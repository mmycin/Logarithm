/// Design tokens for theming.
/// 
/// Defines color palettes for dark and light themes with
/// semantic naming for consistent UI appearance.

pub struct ColorTokens {
    pub bg_base: &'static str,
    pub bg_surface: &'static str,
    pub bg_elevated: &'static str,
    pub bg_input: &'static str,
    pub bg_active: &'static str,
    pub text_primary: &'static str,
    pub text_secondary: &'static str,
    pub text_muted: &'static str,
    pub border: &'static str,
    pub border_subtle: &'static str,
    pub accent: &'static str,
    pub accent_bg: &'static str,
    pub accent_border: &'static str,
}

/// Dark theme color tokens
pub const DARK: ColorTokens = ColorTokens {
    bg_base: "#0d0d0d",
    bg_surface: "#151515",
    bg_elevated: "#1d1d1d",
    bg_input: "#1a1a1a",
    bg_active: "#0d0d0d",
    text_primary: "#e8e8e8",
    text_secondary: "#b8b8b8",
    text_muted: "#707070",
    border: "#2a2a2a",
    border_subtle: "#1f1f1f",
    accent: "#7c9dff",
    accent_bg: "#7c9dff18",
    accent_border: "#7c9dff40",
};

/// Light theme color tokens
pub const LIGHT: ColorTokens = ColorTokens {
    bg_base: "#ffffff",
    bg_surface: "#f8f8f8",
    bg_elevated: "#f0f0f0",
    bg_input: "#fafafa",
    bg_active: "#ffffff",
    text_primary: "#1a1a1a",
    text_secondary: "#4a4a4a",
    text_muted: "#909090",
    border: "#d8d8d8",
    border_subtle: "#e8e8e8",
    accent: "#5a7dd8",
    accent_bg: "#5a7dd818",
    accent_border: "#5a7dd840",
};
