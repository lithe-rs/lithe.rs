//! Browser API wrappers that work on WASM and are no-ops on the server.
//!
//! This module provides safe wrappers around common browser APIs that can be
//! called from `#[client]` functions without needing `#[cfg(target_arch = "wasm32")]`.

/// Shows an alert dialog with the given message.
/// No-op on the server.
#[inline]
pub fn alert(message: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        web_sys::window()
            .expect("no window")
            .alert_with_message(message)
            .expect("alert failed");
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = message;
    }
}

/// Logs a message to the browser console.
/// No-op on the server.
#[inline]
pub fn console_log(message: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        web_sys::console::log_1(&message.into());
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = message;
    }
}

/// Logs a warning to the browser console.
/// No-op on the server.
#[inline]
pub fn console_warn(message: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        web_sys::console::warn_1(&message.into());
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = message;
    }
}

/// Logs an error to the browser console.
/// No-op on the server.
#[inline]
pub fn console_error(message: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        web_sys::console::error_1(&message.into());
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = message;
    }
}

/// Returns the current window location href.
/// Returns None on the server.
#[inline]
pub fn location_href() -> Option<String> {
    #[cfg(target_arch = "wasm32")]
    {
        web_sys::window().and_then(|w| w.location().href().ok())
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        None
    }
}

/// Navigates to a new URL.
/// No-op on the server.
#[inline]
pub fn navigate(url: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            let _ = window.location().set_href(url);
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = url;
    }
}

/// Gets the document element by ID.
/// Returns None on the server.
#[cfg(target_arch = "wasm32")]
#[inline]
pub fn get_element_by_id(id: &str) -> Option<web_sys::Element> {
    web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.get_element_by_id(id))
}

/// Sets the inner HTML of an element by ID.
/// No-op on the server.
#[inline]
pub fn set_inner_html(id: &str, html: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(el) = get_element_by_id(id) {
            el.set_inner_html(html);
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (id, html);
    }
}

/// Gets the inner HTML of an element by ID.
/// Returns None on the server.
#[inline]
pub fn get_inner_html(id: &str) -> Option<String> {
    #[cfg(target_arch = "wasm32")]
    {
        get_element_by_id(id).map(|el| el.inner_html())
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = id;
        None
    }
}
