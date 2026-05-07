/// AI panel state management.
/// 
/// Manages all signals and state for the AI panel including
/// provider settings, chat history, and UI state.

use crate::ai::types::{ChatMessage, Provider};
use crate::shared::storage::get_item;
use leptos::prelude::*;

/// Complete state container for the AI panel
pub struct PanelState {
    // Provider & API settings
    pub provider: ReadSignal<Provider>,
    pub set_provider: WriteSignal<Provider>,
    pub api_key: ReadSignal<String>,
    pub set_api_key: WriteSignal<String>,
    pub model: ReadSignal<String>,
    pub set_model: WriteSignal<String>,
    
    // Chat state
    pub messages: ReadSignal<Vec<ChatMessage>>,
    pub set_messages: WriteSignal<Vec<ChatMessage>>,
    pub input: ReadSignal<String>,
    pub set_input: WriteSignal<String>,
    pub loading: ReadSignal<bool>,
    pub set_loading: WriteSignal<bool>,
    
    // UI state
    pub setup_done: ReadSignal<bool>,
    pub set_setup_done: WriteSignal<bool>,
    
    // Resize state
    pub dragging: ReadSignal<bool>,
    pub set_dragging: WriteSignal<bool>,
    pub drag_start_x: ReadSignal<i32>,
    pub set_drag_start_x: WriteSignal<i32>,
    pub drag_start_w: ReadSignal<u32>,
    pub set_drag_start_w: WriteSignal<u32>,
    
    // Context chips
    pub context_chips: ReadSignal<Vec<(String, usize, String)>>,
    pub set_context_chips: WriteSignal<Vec<(String, usize, String)>>,
    
    // File mentions
    pub mentioned_files: ReadSignal<Vec<String>>,
    pub set_mentioned_files: WriteSignal<Vec<String>>,
    
    // Mention autocomplete
    pub mention_suggestions: ReadSignal<Vec<String>>,
    pub set_mention_suggestions: WriteSignal<Vec<String>>,
    pub mention_active: ReadSignal<bool>,
    pub set_mention_active: WriteSignal<bool>,
    pub mention_selected: ReadSignal<usize>,
    pub set_mention_selected: WriteSignal<usize>,
}

impl PanelState {
    /// Initialize panel state from localStorage
    pub fn new() -> Self {
        let saved_prov = get_item("logan_provider").unwrap_or_default();
        let init_prov = Provider::from_str(&saved_prov);
        let init_key = get_item(init_prov.ls_key()).unwrap_or_default();
        let init_model = get_item("logan_model").unwrap_or_default();
        let init_ready = !init_key.is_empty();

        let (provider, set_provider) = signal(init_prov);
        let (api_key, set_api_key) = signal(init_key);
        let (model, set_model) = signal(init_model);
        let (input, set_input) = signal(String::new());
        let (messages, set_messages) = signal(Vec::<ChatMessage>::new());
        let (loading, set_loading) = signal(false);
        let (setup_done, set_setup_done) = signal(init_ready);
        let (dragging, set_dragging) = signal(false);
        let (drag_start_x, set_drag_start_x) = signal(0i32);
        let (drag_start_w, set_drag_start_w) = signal(0u32);
        let (context_chips, set_context_chips) = signal(Vec::new());
        let (mentioned_files, set_mentioned_files) = signal(Vec::new());
        let (mention_suggestions, set_mention_suggestions) = signal(Vec::new());
        let (mention_active, set_mention_active) = signal(false);
        let (mention_selected, set_mention_selected) = signal(0usize);

        // Set initial greeting if configured
        if init_ready {
            set_messages.set(vec![ChatMessage::assistant(
                "Hi! I'm Logan, your AI log analysis assistant. Open a log file and ask me anything.".into()
            )]);
        }

        Self {
            provider, set_provider,
            api_key, set_api_key,
            model, set_model,
            messages, set_messages,
            input, set_input,
            loading, set_loading,
            setup_done, set_setup_done,
            dragging, set_dragging,
            drag_start_x, set_drag_start_x,
            drag_start_w, set_drag_start_w,
            context_chips, set_context_chips,
            mentioned_files, set_mentioned_files,
            mention_suggestions, set_mention_suggestions,
            mention_active, set_mention_active,
            mention_selected, set_mention_selected,
        }
    }
}
