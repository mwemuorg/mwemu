//! Minimal JSON-RPC 2.0 helpers (the wire format MCP speaks).
//!
//! This is deliberately tiny and synchronous: a request is a single JSON object
//! with `method`, optional `id` and optional `params`; the response echoes the
//! `id`. Notifications (no `id`) get no response.

use serde_json::{json, Value};

/// Build a successful JSON-RPC response string.
pub fn success(id: Value, result: Value) -> String {
    json!({ "jsonrpc": "2.0", "id": id, "result": result }).to_string()
}

/// Build a JSON-RPC error response string.
pub fn error(id: Value, code: i64, message: &str) -> String {
    json!({
        "jsonrpc": "2.0",
        "id": id,
        "error": { "code": code, "message": message }
    })
    .to_string()
}

// Standard JSON-RPC error codes used by this server.
pub const PARSE_ERROR: i64 = -32700;
pub const METHOD_NOT_FOUND: i64 = -32601;
