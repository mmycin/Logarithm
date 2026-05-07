/// AI provider configuration.
/// 
/// Supports multiple AI providers (Gemini, OpenAI) with
/// provider-specific settings and API endpoints.

#[derive(Clone, PartialEq, Debug)]
pub enum Provider {
    Gemini,
    OpenAI,
}

impl Provider {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Gemini => "Gemini",
            Self::OpenAI => "OpenAI",
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            Self::Gemini => "gemini",
            Self::OpenAI => "openai",
        }
    }

    pub fn default_model(&self) -> &'static str {
        match self {
            Self::Gemini => "gemini-1.5-flash",
            Self::OpenAI => "gpt-4o-mini",
        }
    }

    pub fn key_label(&self) -> &'static str {
        match self {
            Self::Gemini => "Google AI API Key",
            Self::OpenAI => "OpenAI API Key",
        }
    }

    pub fn key_hint(&self) -> &'static str {
        match self {
            Self::Gemini => "AIza…",
            Self::OpenAI => "sk-…",
        }
    }

    pub fn docs_url(&self) -> &'static str {
        match self {
            Self::Gemini => "https://aistudio.google.com/app/apikey",
            Self::OpenAI => "https://platform.openai.com/api-keys",
        }
    }

    pub fn ls_key(&self) -> &'static str {
        match self {
            Self::Gemini => "logan_gemini_key",
            Self::OpenAI => "logan_openai_key",
        }
    }

    pub fn from_str(s: &str) -> Self {
        if s == "openai" {
            Self::OpenAI
        } else {
            Self::Gemini
        }
    }
}
