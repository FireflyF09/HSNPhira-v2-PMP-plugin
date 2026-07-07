//! HSNPhira v2 Frontend Web API Plugin for Phira-mp+
//!
//! WIT/component-model plugin that registers HTTP routes and handles
//! requests via the `http.register_route` host API and `on_api` export.
//!
//! Build: cargo build --target wasm32-unknown-unknown --release

phira_plugin_sdk::wit_bindgen!("phira-plugin-v2");

use serde_json::{json, Value};

struct HSNPhiraPlugin;

/// Call host API and return JSON result.
fn host_api(method: &str, args: &[Value]) -> Result<Value, String> {
    let wit_args: Vec<JsonValue> = args.iter().map(json_value_to_wit).collect();
    match phira_host::api_call(method, &wit_args) {
        ApiResult::Ok(value) => Ok(wit_json_to_serde(&value)),
        ApiResult::Error(e) => Err(e),
    }
}

/// Register an HTTP route that calls this plugin's on_api.
fn register_route(path: &str) {
    // http.register_route(path, plugin_name):
    // server stores route→plugin mapping, forwards requests to plugin on_api
    let _ = host_api("http.register_route", &[json!({"path": path, "plugin": "hsnphira-v2"})]);
}

impl Guest for HSNPhiraPlugin {
    fn init(&mut self) -> Result<(), String> {
        // Register all HSNPhira v2 API routes
        for path in &[
            "/newapi/rooms/info",
            "/newapi/rooms/history",
            "/newapi/rooms/listen",
            "/api/rooms/info/:name",
            "/chart/:id/rank",
            "/topchart/chart_rank/:chart_id",
            "/topchart/hot_rank/:timeRange",
            "/user_rank/:timeRange",
            "/rankapi/playtime_leaderboard",
            "/config/version.json",
        ] {
            register_route(path);
        }
        Ok(())
    }

    fn get_info(&mut self) -> PluginInfo {
        PluginInfo {
            name: "hsnphira-v2-pmp-plugin",
            version: "0.1.0",
            author: "FireflyF09",
            description: "HSNPhira v2 Frontend Web API",
        }
    }

    fn cleanup(&mut self) {}

    fn on_event(&mut self, _event: PluginEvent) -> Result<bool, String> {
        Ok(false)
    }

    fn on_api(&mut self, method: String, args: Vec<JsonValue>) -> ApiResult {
        let serde_args: Vec<Value> = args.iter().map(wit_json_to_serde).collect();
        let result = match method.as_str() {
            "/newapi/rooms/info" => rooms_list(),
            "/newapi/rooms/history" => rooms_history(&serde_args),
            "/api/rooms/info/:name" => room_info(&serde_args),
            "/chart/:id/rank" => chart_rank(&serde_args),
            "/topchart/chart_rank/:chart_id" => chart_rank_detail(&serde_args),
            "/topchart/hot_rank/:timeRange" => hot_rank(&serde_args),
            "/user_rank/:timeRange" => user_rank(&serde_args),
            "/rankapi/playtime_leaderboard" => playtime_leaderboard(),
            "/config/version.json" => config_version(),
            _ => json!({"error": format!("unknown route: {method}")}),
        };
        ApiResult::Ok(json_value_to_wit(&result))
    }
}

// ── Route handlers ──

fn rooms_list() -> Value {
    json!({"rooms": [], "player_count": 0})
}
fn rooms_history(_args: &[Value]) -> Value { json!([]) }
fn room_info(_args: &[Value]) -> Value { json!({"error": "not_found"}) }
fn chart_rank(_args: &[Value]) -> Value { json!({"ranks": []}) }
fn chart_rank_detail(_args: &[Value]) -> Value { json!({"ranks": []}) }
fn hot_rank(_args: &[Value]) -> Value { json!({"charts": []}) }
fn user_rank(_args: &[Value]) -> Value { json!({"users": []}) }
fn playtime_leaderboard() -> Value { json!({"leaderboard": []}) }
fn config_version() -> Value { json!({"version": "0.1.0"}) }

// ── JSON <-> WIT JsonValue converters (same as server's wit_host.rs) ──

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
