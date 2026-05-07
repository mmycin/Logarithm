/// Theme enumeration for the application.
/// 
/// Supports Dark and Light modes with corresponding color tokens.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Theme {
    /// Dark theme with light text on dark backgrounds
    Dark,
    /// Light theme with dark text on light backgrounds
    Light,
}

impl Theme {
    /// Check if the current theme is dark
    pub fn is_dark(&self) -> bool {
        matches!(self, Theme::Dark)
    }

    /// Check if the current theme is light
    pub fn is_light(&self) -> bool {
        matches!(self, Theme::Light)
    }
}
