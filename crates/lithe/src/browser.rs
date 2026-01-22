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

pub async fn call_server<Args, Ret>(fn_name: &str, args: Args) -> Ret
where
    Args: serde::Serialize,
    Ret: serde::de::DeserializeOwned,
{
    #[cfg(target_arch = "wasm32")]
    {
        use crate::rpc::{RpcRequest, RpcResponse};
        use wasm_bindgen::JsCast;
        use web_sys::{Request, RequestInit, RequestMode, Response};

        let args_value = serde_json::to_value(args).unwrap();
        let rpc_req = RpcRequest {
            function: fn_name.to_string(),
            args: args_value,
        };
        let body = serde_json::to_string(&rpc_req).unwrap();

        let opts = RequestInit::new();
        opts.set_method("POST");
        opts.set_mode(RequestMode::Cors);
        opts.set_body(&js_sys::JsString::from(body));

        let request = Request::new_with_str_and_init("/api/lithe-rpc", &opts).unwrap();
        request
            .headers()
            .set("Content-Type", "application/json")
            .unwrap();

        let window = web_sys::window().unwrap();
        let resp_value = wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request))
            .await
            .unwrap();
        let resp: Response = resp_value.dyn_into().unwrap();

        let text = wasm_bindgen_futures::JsFuture::from(resp.text().unwrap())
            .await
            .unwrap()
            .as_string()
            .unwrap();
        let rpc_res: RpcResponse =
            serde_json::from_str(&text).expect("Failed to parse RPC response");

        serde_json::from_value(rpc_res.result).expect("Failed to deserialize result")
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (fn_name, args);
        unreachable!("call_server should only be called from WASM")
    }
}
