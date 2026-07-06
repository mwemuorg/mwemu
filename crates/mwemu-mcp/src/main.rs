//! mwemu-mcp — a Model Context Protocol server exposing the mwemu emulator.
//!
//! Lifecycle-oriented: a client opens a session (picking an architecture),
//! configures it, prepares memory, then emulates step by step over MCP tools —
//! as if scripting pymwemu, but driven by an MCP client.
//!
//! Transports:
//!   - stdio (default): newline-delimited JSON-RPC, like radare2-mcp.
//!   - HTTP (optional `http` feature, `--http <addr>`): loopback only.
//!
//! Security: on stdio you are trusted locally and path-based tools are enabled.
//! Over the network transport a sandbox is on by default — path-based tools are
//! refused, emulated networking stays off, and the operator presets any trusted
//! paths at startup. `--unsafe` lifts the sandbox. Note that strong isolation of
//! untrusted samples also wants OS-level confinement (container with
//! `--network none`, read-only rootfs, non-root); see the README.

mod jsonrpc;
mod server;
mod tools;
mod transport;
mod util;

use std::process::ExitCode;

use server::Server;
use transport::{StdioTransport, Transport};

struct Options {
    http: Option<String>,
    allow_disk: bool,
    allow_disk_set: bool,
    maps: Option<String>,
    log: String,
}

/// Install a stderr logger so libmwemu's `log::trace!`/`info!` output (e.g. the
/// instruction trace gated by mwemu_config `verbose`) actually appears. Without
/// this every `log::*` call in libmwemu is a silent no-op. RUST_LOG, if set,
/// wins over the `--log` level.
fn init_logger(level: &str) {
    use std::str::FromStr;
    let mut b = env_logger::Builder::new();
    if std::env::var("RUST_LOG").is_ok() {
        b.parse_env("RUST_LOG");
    } else {
        b.filter_level(log::LevelFilter::from_str(level).unwrap_or(log::LevelFilter::Off));
    }
    b.target(env_logger::Target::Stderr);
    let _ = b.try_init();
}

fn print_help() {
    eprintln!(
        "mwemu-mcp {} — MCP server for the mwemu emulator\n\n\
         USAGE:\n    mwemu-mcp [OPTIONS]\n\n\
         OPTIONS:\n\
         \x20   --http <addr>     Serve over loopback HTTP instead of stdio (needs the\n\
         \x20                     'http' build feature). Address must be 127.0.0.1/::1.\n\
         \x20   --unsafe          Allow filesystem-path tools (load by path, maps folder)\n\
         \x20                     even over the network transport. Default off for --http.\n\
         \x20   --safe            Force the sandbox on (also on stdio).\n\
         \x20   --maps <folder>   Preset a trusted maps folder applied on mwemu_open.\n\
         \x20   --log <level>     Log level to stderr: off|error|warn|info|debug|trace.\n\
         \x20                     Needed to see libmwemu's instruction trace (with verbose).\n\
         \x20   -h, --help        Show this help.\n\n\
         By default (stdio) path tools are enabled; over --http they are sandboxed.",
        env!("CARGO_PKG_VERSION")
    );
}

fn parse_args() -> Result<Options, String> {
    let mut opts = Options {
        http: None,
        allow_disk: true,
        allow_disk_set: false,
        maps: None,
        log: "off".to_string(),
    };
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-h" | "--help" => {
                print_help();
                std::process::exit(0);
            }
            "--http" => {
                opts.http = Some(args.next().ok_or("--http requires an address")?);
            }
            "--unsafe" => {
                opts.allow_disk = true;
                opts.allow_disk_set = true;
            }
            "--safe" => {
                opts.allow_disk = false;
                opts.allow_disk_set = true;
            }
            "--maps" => {
                opts.maps = Some(args.next().ok_or("--maps requires a folder")?);
            }
            "--log" => {
                opts.log = args.next().ok_or("--log requires a level (off|info|debug|trace)")?;
            }
            other => return Err(format!("unknown argument: {other}")),
        }
    }
    Ok(opts)
}

/// True if a `host:port` address binds to loopback only.
fn is_loopback_addr(addr: &str) -> bool {
    let host = if let Some(stripped) = addr.strip_prefix('[') {
        // [::1]:port
        stripped.split(']').next().unwrap_or("")
    } else {
        addr.rsplit_once(':').map(|(h, _)| h).unwrap_or(addr)
    };
    host == "127.0.0.1" || host == "::1" || host == "localhost"
}

/// On unix, save the real stdout fd and point fd 1 at stderr, so libmwemu's own
/// prints (api calls, maps dumps, ...) never corrupt the JSON-RPC channel. The
/// returned File is the original stdout, used by the stdio transport for
/// protocol output.
#[cfg(unix)]
fn redirect_stdout_to_stderr() -> std::fs::File {
    use std::os::fd::FromRawFd;
    // SAFETY: `dup`/`dup2` are plain libc fd calls. `saved` is a fresh fd from
    // `dup(1)` that nothing else owns, so `File::from_raw_fd` taking ownership of
    // it is sound (no double-close, no aliasing of an existing `File`).
    unsafe {
        let saved = libc::dup(1);
        libc::dup2(2, 1);
        std::fs::File::from_raw_fd(saved)
    }
}

fn main() -> ExitCode {
    let opts = match parse_args() {
        Ok(o) => o,
        Err(e) => {
            eprintln!("error: {e}\n");
            print_help();
            return ExitCode::FAILURE;
        }
    };

    // Decide the disk policy. Default depends on transport: stdio is trusted
    // (disk on), HTTP is sandboxed (disk off) unless explicitly overridden.
    init_logger(&opts.log);

    let using_http = opts.http.is_some();
    let allow_disk = if opts.allow_disk_set {
        opts.allow_disk
    } else {
        !using_http
    };

    let mut srv = Server::new(allow_disk, opts.maps);

    if let Some(addr) = opts.http {
        if !is_loopback_addr(&addr) {
            eprintln!(
                "error: --http address '{addr}' is not loopback. Only 127.0.0.1/::1/localhost \
                 are allowed; put a reverse proxy in front for remote exposure."
            );
            return ExitCode::FAILURE;
        }
        #[cfg(feature = "http")]
        {
            // Keep libmwemu noise off the real stdout; HTTP carries the protocol.
            #[cfg(unix)]
            let _saved = redirect_stdout_to_stderr();
            eprintln!(
                "mwemu-mcp: serving MCP over http://{addr} (loopback, sandbox={})",
                if allow_disk { "off" } else { "on" }
            );
            let mut t = match transport::HttpTransport::bind(&addr) {
                Ok(t) => t,
                Err(e) => {
                    eprintln!("error: cannot bind {addr}: {e}");
                    return ExitCode::FAILURE;
                }
            };
            let mut handler = |line: &str| srv.handle(line);
            if let Err(e) = t.serve(&mut handler) {
                eprintln!("error: http transport failed: {e}");
                return ExitCode::FAILURE;
            }
            return ExitCode::SUCCESS;
        }
        #[cfg(not(feature = "http"))]
        {
            eprintln!(
                "error: --http requires building with the 'http' feature: \
                 cargo build -p mwemu-mcp --features http"
            );
            return ExitCode::FAILURE;
        }
    }

    // Default: stdio transport.
    #[cfg(unix)]
    let out = redirect_stdout_to_stderr();
    #[cfg(not(unix))]
    let out = std::io::stdout();

    eprintln!(
        "mwemu-mcp {}: serving MCP over stdio (disk={})",
        env!("CARGO_PKG_VERSION"),
        if allow_disk { "on" } else { "sandboxed" }
    );

    let mut t = StdioTransport::new(out);
    let mut handler = |line: &str| srv.handle(line);
    if let Err(e) = t.serve(&mut handler) {
        eprintln!("error: stdio transport failed: {e}");
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}
