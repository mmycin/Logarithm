/// Logan action handler.
/// 
/// Handles actions sent from the file viewer to the AI panel,
/// such as adding context or explaining log lines.

use crate::shared::types::LoganAction;
use leptos::prelude::*;

/// Create the Logan action effect handler
pub fn create_logan_action_handler(
    logan_action: ReadSignal<Option<LoganAction>>,
    set_logan_action: WriteSignal<Option<LoganAction>>,
    set_context_chips: WriteSignal<Vec<(String, usize, String)>>,
    set_input: WriteSignal<String>,
    setup_done: ReadSignal<bool>,
    set_setup_done: WriteSignal<bool>,
    send_message: impl Fn() + 'static,
) {
    Effect::new(move |_| {
        let Some(action) = logan_action.get() else { return };
        set_logan_action.set(None);

        match action {
            LoganAction::AddContext { file, line, text } => {
                set_context_chips.update(|chips| {
                    chips.push((file, line, text));
                });
                if !setup_done.get() {
                    set_setup_done.set(true);
                }
            }
            LoganAction::AddMultipleContext { items } => {
                set_context_chips.update(|chips| {
                    for (file, line, text) in items {
                        chips.push((file, line, text));
                    }
                });
                if !setup_done.get() {
                    set_setup_done.set(true);
                }
            }
            LoganAction::Explain { file, line, text } => {
                let msg = format!(
                    "Explain this log line from {file}:{line}\n\n```\n{text}\n```"
                );
                set_input.set(msg);
                set_context_chips.set(Vec::new());
                if !setup_done.get() {
                    set_setup_done.set(true);
                }
                send_message();
            }
        }
    });
}
