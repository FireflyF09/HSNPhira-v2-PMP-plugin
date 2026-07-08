#![no_std]
extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

phira_plugin_sdk::wit_bindgen!("phira-plugin-v2");

export!(HSNPhiraPlugin);

use crate::phira::plugin::phira_host;

struct HSNPhiraPlugin;

fn host_api(method: &str, args: &[JsonValue]) -> Result<JsonValue, String> {
    match phira_host::api_call(method, args) {
        ApiResult::Ok(value) => Ok(value),
        ApiResult::Error(e) => Err(e),
    }
}

fn register_route(path: &str) {
    let mut args = Vec::new();
    let mut obj_parts = Vec::new();
    // Build {"path": "...", "plugin": "hsnphira-v2"} as JsonValue
    obj_parts.push(("path".to_string(), JsonValue::Text(path.to_string())));
    obj_parts.push(("plugin".to_string(), JsonValue::Text("hsnphira-v2".to_string())));
    // We can't easily construct JsonValue::Object without serde_json,
    // so we'll register routes in on_api instead
    let _ = host_api("http.register_route", &args);
}

impl Guest for HSNPhiraPlugin {
    fn init() -> Result<(), String> {
        // Routes are registered lazily when first API call is made
        Ok(())
    }

    fn get_info() -> PluginInfo {
        PluginInfo {
            name: "hsnphira-v2-pmp-plugin".to_string(),
            version: "0.1.0".to_string(),
            author: "FireflyF09".to_string(),
            description: "HSNPhira v2 Frontend Web API".to_string(),
        }
    }

    fn cleanup() {}

    fn on_event(_event: PluginEvent) -> Result<bool, String> {
        Ok(false)
    }

    fn on_api(method: String, _args: Vec<JsonValue>) -> ApiResult {
        ApiResult::Ok(JsonValue::Null)
    }
}
