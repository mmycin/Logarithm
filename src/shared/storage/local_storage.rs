/// Safe localStorage access functions.
/// 
/// Wraps browser localStorage API with error handling.

/// Get an item from localStorage
/// 
/// Returns None if the key doesn't exist or if localStorage is unavailable.
pub fn get_item(key: &str) -> Option<String> {
    web_sys::window()?
        .local_storage()
        .ok()??
        .get_item(key)
        .ok()?
}

/// Set an item in localStorage
/// 
/// Silently fails if localStorage is unavailable.
pub fn set_item(key: &str, value: &str) {
    if let Some(Ok(Some(storage))) = web_sys::window().map(|w| w.local_storage()) {
        let _ = storage.set_item(key, value);
    }
}

/// Remove an item from localStorage
/// 
/// Silently fails if localStorage is unavailable.
pub fn remove_item(key: &str) {
    if let Some(Ok(Some(storage))) = web_sys::window().map(|w| w.local_storage()) {
        let _ = storage.remove_item(key);
    }
}
