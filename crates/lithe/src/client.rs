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

#[cfg(target_arch = "wasm32")]
#[inline]
pub fn get_element_by_id(id: &str) -> Option<web_sys::Element> {
    web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.get_element_by_id(id))
}

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
