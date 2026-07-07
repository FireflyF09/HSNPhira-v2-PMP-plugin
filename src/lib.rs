//! HSNPhira v2 Frontend Web API Plugin for Phira-mp+
//!
//! JSON bridge plugin. The server forwards HTTP requests to this plugin's
//! phira_on_api with (method=path, args=path_params).
//!
//! Build: cargo build --target wasm32-unknown-unknown --release

use serde_json::Value;

phira_plugin_sdk::plugin_entry!(init, get_info, cleanup, on_event, on_api);

// ── Plugin lifecycle ──

fn init() -> i32 {
    // Register routes via host_api_call during init
    // (host_api_call is a stub in the SDK — real implementation calls server)
    0
}

fn get_info() {}
fn cleanup() {}
fn on_event(_ptr: i32, _len: i32) -> i32 { 0 }

// ── Output buffer for phira_on_api results ──

const OUTPUT_SIZE: usize = 65536;
static mut OUTPUT_BUF: [u8; OUTPUT_SIZE] = [0; OUTPUT_SIZE];

// ── API dispatch ──

fn on_api(method_ptr: i32, method_len: i32, args_ptr: i32, args_len: i32) -> i64 {
    let method = unsafe {
        let slice = std::slice::from_raw_parts(method_ptr as *const u8, method_len as usize);
        String::from_utf8_lossy(slice).to_string()
    };
    let args_json = unsafe {
        let slice = std::slice::from_raw_parts(args_ptr as *const u8, args_len as usize);
        String::from_utf8_lossy(slice).to_string()
    };
    let args: Vec<Value> = serde_json::from_str(&args_json).unwrap_or_default();

    let result = match method.as_str() {
        "/newapi/rooms/info" => rooms_list(),
        "/newapi/rooms/history" => rooms_history(&args),
        "/api/rooms/info/:name" => room_info(&args),
        "/chart/:id/rank" => chart_rank(&args),
        "/topchart/chart_rank/:chart_id" => chart_rank_detail(&args),
        "/topchart/hot_rank/:timeRange" => hot_rank(&args),
        "/user_rank/:timeRange" => user_rank(&args),
        "/rankapi/playtime_leaderboard" => playtime_leaderboard(),
        "/config/version.json" => config_version(),
        _ => serde_json::json!({"error": format!("unknown route: {method}")}),
    };

    let output = serde_json::to_string(&result).unwrap_or_else(|_| "null".to_string());
    let bytes = output.as_bytes();
    let len = bytes.len().min(OUTPUT_SIZE);

    unsafe {
        OUTPUT_BUF[..len].copy_from_slice(&bytes[..len]);
        ((len as i64) << 32) | (OUTPUT_BUF.as_ptr() as i64 & 0xffff_ffff)
    }
}

// ── Route handlers (stubs) ──

fn rooms_list() -> Value {
    Value::Array(vec![])
}

fn rooms_history(_args: &[Value]) -> Value {
    Value::Array(vec![])
}

fn room_info(_args: &[Value]) -> Value {
    serde_json::json!({"error": "not_found"})
}

fn chart_rank(_args: &[Value]) -> Value {
    serde_json::json!({"ranks": []})
}

fn chart_rank_detail(_args: &[Value]) -> Value {
    serde_json::json!({"ranks": []})
}

fn hot_rank(_args: &[Value]) -> Value {
    serde_json::json!({"charts": []})
}

fn user_rank(_args: &[Value]) -> Value {
    serde_json::json!({"users": []})
}

fn playtime_leaderboard() -> Value {
    serde_json::json!({"leaderboard": []})
}

fn config_version() -> Value {
    serde_json::json!({"version": "0.1.0", "build_time": "auto"})
}
