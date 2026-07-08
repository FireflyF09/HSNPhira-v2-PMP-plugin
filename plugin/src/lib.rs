#![allow(dead_code, unused_variables)]
//! HSNPhira v2 Frontend Web API Plugin for Phira-mp+
//!
//! WIT/component-model plugin. Registers HTTP routes via host API and handles
//! requests via the `on_api` export.
//!
//! Build: cargo build --target wasm32-unknown-unknown --release

phira_plugin_sdk::wit_bindgen!("phira-plugin-v2");

// Export the Guest implementation as WASM exports (required by wit-bindgen 0.58)
export!(HSNPhiraPlugin);

use serde_json::{json, Value};

// In wit-bindgen 0.58, the generated import module is at this path.
// The exported trait uses `&self` (not `&mut self`) and String fields.
use crate::phira::plugin::phira_host;

struct HSNPhiraPlugin;

fn host_api(method: &str, args: &[Value]) -> Result<Value, String> {
    let wit_args: Vec<JsonValue> = args.iter().map(json_value_to_wit).collect();
    match phira_host::api_call(method, &wit_args) {
        ApiResult::Ok(value) => Ok(wit_json_to_serde(&value)),
        ApiResult::Error(e) => Err(e),
    }
}

fn register_route(path: &str) {
    let _ = host_api("http.register_route", &[json!({"path": path, "plugin": "hsnphira-v2"})]);
}

impl Guest for HSNPhiraPlugin {
    fn init() -> Result<(), String> {
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
        let result = match method.as_str() {
            "/newapi/rooms/info" => json!({"rooms": [], "player_count": 0}),
            "/newapi/rooms/history" => json!([]),
            "/api/rooms/info/:name" => json!({"error": "not_found"}),
            "/chart/:id/rank" => json!({"ranks": []}),
            "/topchart/chart_rank/:chart_id" => json!({"ranks": []}),
            "/topchart/hot_rank/:timeRange" => json!({"charts": []}),
            "/user_rank/:timeRange" => json!({"users": []}),
            "/rankapi/playtime_leaderboard" => json!({"leaderboard": []}),
            "/config/version.json" => json!({"version": "0.1.0"}),
            _ => json!({"error": format!("unknown route: {method}")}),
        };
        ApiResult::Ok(json_value_to_wit(&result))
    }
}

fn json_value_to_wit(value: &Value) -> JsonValue {
    match value {
        Value::Null => JsonValue::Null,
        Value::Bool(b) => JsonValue::Flag(*b),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() { JsonValue::Integer(i) }
            else if let Some(f) = n.as_f64() { JsonValue::Float(f) }
            else { JsonValue::Text(n.to_string()) }
        }
        Value::String(s) => JsonValue::Text(s.clone()),
        Value::Array(arr) => JsonValue::Array(serde_json::to_string(arr).unwrap_or_default()),
        Value::Object(obj) => JsonValue::Object(serde_json::to_string(obj).unwrap_or_default()),
    }
}

fn wit_json_to_serde(value: &JsonValue) -> Value {
    match value {
        JsonValue::Null => Value::Null,
        JsonValue::Flag(b) => Value::Bool(*b),
        JsonValue::Integer(i) => json!(*i),
        JsonValue::Float(f) => json!(*f),
        JsonValue::Text(s) => Value::String(s.clone()),
        JsonValue::Array(s) | JsonValue::Object(s) => {
            serde_json::from_str(s).unwrap_or(Value::String(s.clone()))
        }
    }
}
