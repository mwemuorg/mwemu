//! Transports carry newline/HTTP-framed JSON-RPC messages to and from the
//! dispatcher. Both implementations are blocking and single-threaded, matching
//! mwemu's synchronous, one-session-at-a-time model.

use std::io::{self, BufRead, Write};

/// A transport drives the serve loop, handing each raw request string to
/// `handler` and writing back whatever it returns (a notification returns
/// `None` and produces no reply).
pub trait Transport {
    fn serve(&mut self, handler: &mut dyn FnMut(&str) -> Option<String>) -> io::Result<()>;
}

/// stdio transport: newline-delimited JSON on stdin/stdout. `out` is the real
/// stdout file descriptor (saved before we redirect libmwemu's own prints to
/// stderr) so emulator output never corrupts the protocol channel.
pub struct StdioTransport<W: Write> {
    out: W,
}

impl<W: Write> StdioTransport<W> {
    pub fn new(out: W) -> Self {
        Self { out }
    }
}

impl<W: Write> Transport for StdioTransport<W> {
    fn serve(&mut self, handler: &mut dyn FnMut(&str) -> Option<String>) -> io::Result<()> {
        let stdin = io::stdin();
        let mut lock = stdin.lock();
        let mut line = String::new();
        loop {
            line.clear();
            let n = lock.read_line(&mut line)?;
            if n == 0 {
                break; // EOF: client closed the pipe
            }
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            if let Some(resp) = handler(trimmed) {
                self.out.write_all(resp.as_bytes())?;
                self.out.write_all(b"\n")?;
                self.out.flush()?;
            }
        }
        Ok(())
    }
}

/// Loopback-only HTTP transport (optional `http` feature). One JSON-RPC message
/// per POST body, reply is `application/json`. Bind address is validated to be a
/// loopback address by the caller; the `Host` header is re-validated per request
/// to blunt DNS-rebinding from a browser pointed at localhost.
#[cfg(feature = "http")]
pub struct HttpTransport {
    server: tiny_http::Server,
}

#[cfg(feature = "http")]
impl HttpTransport {
    pub fn bind(addr: &str) -> io::Result<Self> {
        let server = tiny_http::Server::http(addr)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
        Ok(Self { server })
    }
}

#[cfg(feature = "http")]
fn host_is_local(host: &str) -> bool {
    // Strip optional port, then compare against loopback names/addresses.
    let h = host.rsplit_once(':').map(|(a, _)| a).unwrap_or(host);
    let h = h.trim_matches(|c| c == '[' || c == ']');
    h == "localhost" || h == "127.0.0.1" || h == "::1"
}

#[cfg(feature = "http")]
impl Transport for HttpTransport {
    fn serve(&mut self, handler: &mut dyn FnMut(&str) -> Option<String>) -> io::Result<()> {
        for mut req in self.server.incoming_requests() {
            // DNS-rebinding guard: only serve requests whose Host is loopback.
            let host_ok = req
                .headers()
                .iter()
                .find(|h| h.field.equiv("Host"))
                .map(|h| host_is_local(h.value.as_str()))
                .unwrap_or(true);
            if !host_ok {
                let _ = req.respond(tiny_http::Response::from_string("forbidden").with_status_code(403));
                continue;
            }
            if *req.method() != tiny_http::Method::Post {
                let _ = req.respond(
                    tiny_http::Response::from_string("use POST with a JSON-RPC body")
                        .with_status_code(405),
                );
                continue;
            }
            let mut body = String::new();
            if req.as_reader().read_to_string(&mut body).is_err() {
                let _ = req.respond(tiny_http::Response::from_string("bad body").with_status_code(400));
                continue;
            }
            let reply = handler(body.trim()).unwrap_or_default();
            let header =
                tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap();
            let _ = req.respond(tiny_http::Response::from_string(reply).with_header(header));
        }
        Ok(())
    }
}
