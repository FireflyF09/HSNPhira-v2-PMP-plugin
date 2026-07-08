//! HSNPhira v2 Frontend Web API Plugin for Phira-mp+
//!
//! WIT/component-model plugin. Registers HTTP routes via host API and handles
//! requests via the `on_api` export.
//!
//! Room/player data is fetched from the host via `phira_host::api_call`.
//!
//! Build: cargo build --target wasm32-unknown-unknown --release
//! Then:  wasm-tools component new <file>.wasm -o <file>.component.wasm

phira_plugin_sdk::wit_bindgen!("phira-plugin-v2");
export!(HSNPhiraPlugin);

use serde_json::{json, Value};
use crate::phira::plugin::phira_host;

struct HSNPhiraPlugin;

/// Call the host's generic API query (server_state_query dispatch).
fn host_api(method: &str, args: &[Value]) -> Result<Value, String> {
    let wit_args: Vec<JsonValue> = args.iter().map(json_value_to_wit).collect();
    match phira_host::api_call(method, &wit_args) {
        ApiResult::Ok(value) => Ok(wit_json_to_serde(&value)),
        ApiResult::Error(e) => Err(e),
    }
}

fn register_route(path: &str) {
    let _ = host_api("http.register_route", &[json!({"path": path, "plugin": "hsnphira-v2-pmp-plugin"})]);
}

impl Guest for HSNPhiraPlugin {
    fn init() -> Result<(), String> {
        for path in &[
            "/newapi/rooms/info",
            "/newapi/rooms/history/:room_id",
            "/api/rooms/info/:name",
            "/config/version.json",
        ] {
            register_route(path);
        }
        // Register SSE stream for /newapi/rooms/listen.
        let _ = host_api("sse.register_stream", &[json!({
            "path": "/newapi/rooms/listen",
            "plugin": "hsnphira-v2-pmp-plugin",
            "event_types": ["RoomCreate", "RoomJoin", "RoomLeave",
                "RoomModify", "GameEnd", "RoundComplete"],
        })]);
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

    fn on_api(method: String, args: Vec<JsonValue>) -> ApiResult {
        let _serde_args: Vec<Value> = args.iter().map(wit_json_to_serde).collect();
        let result = match method.as_str() {
            "/newapi/rooms/info" => {
                match host_api("rooms.list", &[]) {
                    Ok(rooms) => {
                        let rooms_arr = rooms.as_array().cloned().unwrap_or_default();
                        let player_count: usize = rooms_arr.iter()
                            .filter_map(|r| r.get("player_count").and_then(|v| v.as_u64()))
                            .sum::<u64>() as usize;
                        json!({"rooms": rooms_arr, "player_count": player_count})
                    }
                    Err(e) => json!({"error": e, "rooms": [], "player_count": 0}),
                }
            }
            "/newapi/rooms/history/:room_id" => {
                let room_id = _serde_args.get(0).and_then(|v| v.as_str()).unwrap_or("");
                if room_id.is_empty() {
                    json!({"error": "missing room_id"})
                } else {
                    match host_api("room.history", &[json!(room_id)]) {
                        Ok(data) => data,
                        Err(e) => json!({"error": e, "rounds": []}),
                    }
                }
            }
            "/api/rooms/info/:name" => {
                let name = _serde_args.get(0).and_then(|v| v.as_str()).unwrap_or("");
                if name.is_empty() {
                    json!({"error": "missing room name"})
                } else {
                    match host_api("rooms.by_name", &[json!(name)]) {
                        Ok(data) => data,
                        Err(_) => json!({"error": "not_found"}),
                    }
                }
            }
            "/config/version.json" => json!({
                "version": "0.1.0",
                "plugin": "hsnphira-v2-pmp-plugin",
                "phira_mp_plus": env!("CARGO_PKG_VERSION"),
            }),
            "sse:translate" => {
                let obj = _serde_args.get(0)
                    .and_then(|v| v.as_object()).cloned().unwrap_or_default();
                let raw_type = obj.get("event_type")
                    .and_then(|v| v.as_str()).unwrap_or("").to_string();
                let raw_data: Value = obj.get("data")
                    .and_then(|v| v.as_str())
                    .and_then(|s| serde_json::from_str(s).ok())
                    .unwrap_or(json!({}));
                let (hsn_type, hsn_data): (&str, Value) = match raw_type.as_str() {
                    "RoomCreate" => ("create_room", json!({
                        "room": raw_data.get("room_id"), "room_uuid": raw_data.get("room_uuid"),
                    })),
                    "RoomJoin" | "RoomEnter" => ("join_room", json!({
                        "room": raw_data.get("room_id"), "user": raw_data.get("user_id"),
                    })),
                    "RoomLeave" => ("leave_room", json!({
                        "room": raw_data.get("room_id"), "user": raw_data.get("user_id"),
                    })),
                    "RoomModify" => ("update_room", json!({
                        "room": raw_data.get("room_id"), "data": raw_data,
                    })),
                    "GameEnd" => ("player_score", json!({
                        "room": raw_data.get("room_id"), "record": raw_data.get("game_result"),
                    })),
                    "RoundComplete" => ("start_round", json!({
                        "room": raw_data.get("room_id"), "chart_id": raw_data.get("chart_id"),
                    })),
                    _ => ("", json!(null)),
                };
                let mut payload = json!({"type": hsn_type});
                if let Some(obj) = hsn_data.as_object() {
                    payload.as_object_mut().unwrap().extend(obj.clone());
                }
                payload
            }
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
