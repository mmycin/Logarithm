/// Tauri command invocation helper.
/// 
/// Provides a typed interface for calling Tauri backend commands.

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// Invoke a Tauri command from the frontend
    /// 
    /// # Arguments
    /// * `cmd` - The command name
    /// * `args` - The command arguments as a JsValue
    /// 
    /// # Returns
    /// A JsValue containing the command result
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    pub async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

/// Re-export for convenience
pub use invoke as invoke_tauri;
