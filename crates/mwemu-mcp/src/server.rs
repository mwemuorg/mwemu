//! The MCP server: holds the single emulator session and the security policy,
//! and routes MCP methods (`initialize`, `tools/list`, `tools/call`, `ping`).

use std::panic::AssertUnwindSafe;

use serde_json::{json, Value};

use libmwemu::emu::Emu;

use crate::jsonrpc;
use crate::tools;

/// Single-session, mono-emulator server state.
pub struct Server {
    /// The currently open emulator (None until `mwemu_open`).
    pub emu: Option<Emu>,
    /// Whether tools that touch the real filesystem (load by path, save, ...)
    /// are allowed. True on local stdio; false on the network transport unless
    /// the operator passed `--unsafe`.
    pub allow_disk: bool,
    /// Operator-preset maps folder applied on `mwemu_open` (trusted, set at
    /// startup with `--maps`), so clients never choose a path in sandbox mode.
    pub default_maps: Option<String>,
}

impl Server {
    pub fn new(allow_disk: bool, default_maps: Option<String>) -> Self {
        Self {
            emu: None,
            allow_disk,
            default_maps,
        }
    }

    // --- helpers used by tool handlers ---------------------------------------

    /// Borrow the open emulator, or a friendly error if no session is open.
    pub fn emu(&self) -> Result<&Emu, String> {
        self.emu
            .as_ref()
            .ok_or_else(|| "no session open: call mwemu_open first".to_string())
    }

    /// Mutably borrow the open emulator.
    pub fn emu_mut(&mut self) -> Result<&mut Emu, String> {
        self.emu
            .as_mut()
            .ok_or_else(|| "no session open: call mwemu_open first".to_string())
    }

    /// Gate a filesystem-touching operation behind the disk policy.
    pub fn require_disk(&self) -> Result<(), String> {
        if self.allow_disk {
            Ok(())
        } else {
            Err("disk access is disabled in sandbox mode (network transport). \
                 Provide code as bytes (mwemu_load_code_bytes), or start the \
                 server with --unsafe to allow path-based tools."
                .to_string())
        }
    }

    // --- JSON-RPC / MCP dispatch ---------------------------------------------

    /// Handle one raw JSON-RPC message; return the reply string, or None for a
    /// notification (no `id`).
    pub fn handle(&mut self, line: &str) -> Option<String> {
        let v: Value = match serde_json::from_str(line) {
            Ok(v) => v,
            Err(e) => {
                return Some(jsonrpc::error(
                    Value::Null,
                    jsonrpc::PARSE_ERROR,
                    &format!("parse error: {e}"),
                ))
            }
        };

        let id = v.get("id").cloned();
        let method = v.get("method").and_then(|m| m.as_str()).unwrap_or("");
        let params = v.get("params").cloned().unwrap_or(Value::Null);

        match method {
            "initialize" => Some(jsonrpc::success(id.unwrap_or(Value::Null), self.initialize(&params))),
            // notifications: no reply
            "notifications/initialized" | "initialized" | "notifications/cancelled" => None,
            "ping" => Some(jsonrpc::success(id.unwrap_or(Value::Null), json!({}))),
            "tools/list" => Some(jsonrpc::success(id.unwrap_or(Value::Null), self.tools_list())),
            "tools/call" => {
                Some(jsonrpc::success(id.unwrap_or(Value::Null), self.tools_call(&params)))
            }
            _ => match id {
                Some(id) => Some(jsonrpc::error(
                    id,
                    jsonrpc::METHOD_NOT_FOUND,
                    &format!("method not found: {method}"),
                )),
                None => None,
            },
        }
    }

    fn initialize(&self, params: &Value) -> Value {
        // Echo the client's protocol version when present (spec-recommended).
        let proto = params
            .get("protocolVersion")
            .and_then(|v| v.as_str())
            .unwrap_or("2025-06-18");
        json!({
            "protocolVersion": proto,
            "capabilities": { "tools": { "listChanged": false } },
            "serverInfo": {
                "name": "mwemu-mcp",
                "version": env!("CARGO_PKG_VERSION")
            },
            "instructions": "Emulate x86/x64/arm64 with mwemu. Lifecycle: mwemu_open \
                (pick arch) -> configure (mwemu_config) -> prepare (mwemu_alloc / \
                mwemu_write_* / mwemu_set_reg / load) -> emulate (mwemu_step / \
                mwemu_run / mwemu_call) -> inspect (mwemu_regs / mwemu_read_mem / \
                mwemu_disassemble / mwemu_maps) -> mwemu_close. Addresses may be \
                given as numbers or hex strings like \"0x401000\"."
        })
    }

    fn tools_list(&self) -> Value {
        let list: Vec<Value> = tools::TOOLS
            .iter()
            .map(|t| {
                json!({
                    "name": t.name,
                    "description": t.description,
                    "inputSchema": (t.schema)(),
                })
            })
            .collect();
        json!({ "tools": list })
    }

    fn tools_call(&mut self, params: &Value) -> Value {
        let name = params.get("name").and_then(|v| v.as_str()).unwrap_or("");
        let args = params
            .get("arguments")
            .cloned()
            .unwrap_or_else(|| json!({}));

        let tool = match tools::TOOLS.iter().find(|t| t.name == name) {
            Some(t) => t,
            None => return call_error(format!("unknown tool: {name}")),
        };

        // libmwemu loaders/decoders are full of `.unwrap()` and can panic on
        // malformed input. Contain it here so one bad call can't take down a
        // long-running server.
        let outcome =
            std::panic::catch_unwind(AssertUnwindSafe(|| (tool.handler)(self, &args)));
        let result = match outcome {
            Ok(r) => r,
            Err(_) => Err("internal emulator panic (input may be malformed); \
                          the session may be unstable, try mwemu_close then mwemu_open"
                .to_string()),
        };

        match result {
            Ok(v) => call_ok(v),
            Err(e) => call_error(e),
        }
    }
}

/// Wrap a tool's structured value into an MCP `CallToolResult`.
fn call_ok(v: Value) -> Value {
    let text = match &v {
        Value::String(s) => s.clone(),
        _ => serde_json::to_string_pretty(&v).unwrap_or_else(|_| v.to_string()),
    };
    let mut result = json!({
        "content": [ { "type": "text", "text": text } ],
        "isError": false,
    });
    if v.is_object() {
        result["structuredContent"] = v;
    }
    result
}

/// Tool execution error: reported as a normal result with `isError: true`, per
/// the MCP spec (reserved for tool failures, not protocol errors).
fn call_error(msg: String) -> Value {
    json!({
        "content": [ { "type": "text", "text": msg } ],
        "isError": true,
    })
}
