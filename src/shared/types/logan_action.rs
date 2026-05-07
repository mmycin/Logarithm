/// Actions sent from FileViewer to AiPanel.
/// 
/// These actions represent user interactions with log entries
/// that trigger AI assistant functionality.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoganAction {
    /// Add a single line as context chip (does NOT auto-send)
    AddContext {
        file: String,
        line: usize,
        text: String,
    },
    
    /// Add multiple lines as context chips (does NOT auto-send)
    AddMultipleContext {
        items: Vec<(String, usize, String)>,
    },
    
    /// Add a line and immediately send an "Explain" request
    Explain {
        file: String,
        line: usize,
        text: String,
    },
}
