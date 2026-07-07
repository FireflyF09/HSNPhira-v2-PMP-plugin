//! HSNPhira v2 Frontend Web API Plugin for Phira-mp+
//!
//! Registers HTTP API endpoints required by HSNPhira v2 frontend via
//! the `http.register_route` host API call. The server handles each
//! route through `server_state_query` internally.
//!
//! Build: cargo build --target wasm32-unknown-unknown --release

phira_plugin_sdk::wit_bindgen!("phira-plugin-v2");

use serde_json::{json, Value};

struct HSNPhiraPlugin;

/// Call host API and return JSON result.
fn host_api(method: &str, args: &[Value]) -> Result<Value, String> {
    let wit_args: Vec<JsonValue> = args.iter().map(json_value_to_wit).collect();
    match host::api_call(method, &wit_args) {
        ApiResult::Ok(value) => Ok(wit_json_to_serde(&value)),
        ApiResult::Error(e) => Err(e),
    }
}

/// Register an HTTP route on the Phira-mp+ HTTP server.
fn register_route(path: &str) {
    let _ = host_api("http.register_route", &[json!({"path": path})]);
}

impl PhiraPluginV2 for HSNPhiraPlugin {
    fn init(&mut self) -> Result<(), String> {
        // 房间 API
        register_route("/newapi/rooms/info");
        register_route("/newapi/rooms/history");
        register_route("/newapi/rooms/listen");
        register_route("/api/rooms/info/:name");

        // 谱面排行 API
        register_route("/chart/:id/rank");
        register_route("/topchart/chart_rank/:chart_id");
        register_route("/topchart/hot_rank/:timeRange");

        // 用户排行 API
        register_route("/user_rank/:timeRange");
        register_route("/rankapi/playtime_leaderboard");

        // 配置/版本 API
        register_route("/config/version.json");

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
        match host_api(&method, &serde_args) {
            Ok(value) => ApiResult::Ok(json_value_to_wit(&value)),
            Err(e) => ApiResult::Error(e),
        }
    }
}

// ── JSON <-> WIT JsonValue converters ──

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
