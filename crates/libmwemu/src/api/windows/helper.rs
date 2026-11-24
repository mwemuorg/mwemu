use lazy_static::lazy_static;
use std::sync::Mutex;

use crate::emu;

pub struct Handler {
    id: u64,
    uri: String,
    data: Vec<u8>,
    pos: i64,
    eof: bool,
}

impl Handler {
    fn new(id: u64, uri: &str) -> Handler {
        Handler {
            id,
            uri: uri.to_string(),
            data: vec![],
            pos: 0,
            eof: false,
        }
    }
}

lazy_static! {
    static ref HANDLERS: Mutex<Vec<Handler>> = Mutex::new(Vec::new());
    static ref SOCKETS: Mutex<Vec<u64>> = Mutex::new(vec![0; 0]);
}

pub fn handler_reset() {
    let mut handles = HANDLERS.lock().unwrap();
    handles.clear();
}

pub fn socket_reset() {
    let mut sockets = SOCKETS.lock().unwrap();
    sockets.clear();
}

/// Allocate a Linux pipe, event, or socket alongside the process's virtual files.
pub(crate) fn linux_descriptor(
    limit: u32,
    occupied: impl Fn(u32) -> bool,
    uri: Option<&str>,
) -> Option<u64> {
    let mut handles = HANDLERS.lock().ok()?;
    let mut sockets = SOCKETS.lock().ok()?;
    let fd = (3..limit).find(|&fd| {
        !occupied(fd)
            && !handles.iter().any(|h| h.id == fd as u64)
            && !sockets.contains(&(fd as u64))
    })? as u64;
    if let Some(uri) = uri {
        let index = handles.partition_point(|h| h.id < fd);
        handles.insert(index, Handler::new(fd, uri));
    } else {
        let index = sockets.partition_point(|&id| id < fd);
        sockets.insert(index, fd);
    }
    Some(fd)
}

pub fn handler_create(uri: &str) -> u64 {
    let mut handles = HANDLERS.lock().unwrap();

    // Start at 3 so allocated file descriptors never collide with the standard
    // streams (0=stdin, 1=stdout, 2=stderr) on the Linux syscall path; harmless
    // for Windows handles, which carry no special meaning at 1/2.
    let new_id: u64 = if handles.is_empty() {
        3
    } else {
        let last_id = handles[handles.len() - 1].id;
        last_id + 1
    };

    let new_handler = Handler::new(new_id, uri);

    handles.push(new_handler);
    new_id
}

pub fn handler_close(handle: u64) -> bool {
    let mut handles = HANDLERS.lock().unwrap();
    let idx = match handles.iter().position(|h| h.id == handle) {
        Some(i) => i,
        None => return false,
    };
    handles.remove(idx);
    true
}

pub fn handler_print() {
    let handles = HANDLERS.lock().unwrap();
    for h in handles.iter() {
        log::trace!("{:x} {}", h.id, h.uri);
    }
}

pub fn handler_exist(handle: u64) -> bool {
    let handles = HANDLERS.lock().unwrap();
    handles.iter().position(|h| h.id == handle).is_some()
}

pub fn handler_put_bytes(handle: u64, data: &[u8]) {
    let mut handles = HANDLERS.lock().unwrap();
    if let Some(idx) = handles.iter().position(|h| h.id == handle) {
        handles[idx].data = data.to_vec()
    }
}

pub fn handler_get_uri(handle: u64) -> String {
    let handles = HANDLERS.lock().unwrap();
    match handles.iter().position(|h| h.id == handle) {
        Some(idx) => handles[idx].uri.clone(),
        None => String::new(),
    }
}

pub fn handler_find_by_uri(uri: &str) -> Option<u64> {
    let handles = HANDLERS.lock().unwrap();
    handles.iter().find(|h| h.uri == uri).map(|h| h.id)
}

pub fn handler_seek(handle: u64, offset: i64, origin: u32) -> i64 {
    let mut handles = HANDLERS.lock().unwrap();
    if let Some(h) = handles.iter_mut().find(|h| h.id == handle) {
        let new_pos = match origin {
            0 => offset,                       // SEEK_SET
            1 => h.pos + offset,               // SEEK_CUR
            2 => h.data.len() as i64 + offset, // SEEK_END
            _ => return -1,
        };
        h.pos = new_pos.max(0);
        h.eof = false;
        h.pos
    } else {
        -1
    }
}

pub fn handler_tell(handle: u64) -> i64 {
    let handles = HANDLERS.lock().unwrap();
    handles
        .iter()
        .find(|h| h.id == handle)
        .map(|h| h.pos)
        .unwrap_or(-1)
}

pub fn handler_eof(handle: u64) -> bool {
    let handles = HANDLERS.lock().unwrap();
    handles
        .iter()
        .find(|h| h.id == handle)
        .map(|h| h.eof)
        .unwrap_or(true)
}

pub fn handler_set_eof(handle: u64) {
    let mut handles = HANDLERS.lock().unwrap();
    if let Some(h) = handles.iter_mut().find(|h| h.id == handle) {
        h.eof = true;
    }
}

pub fn handler_get_bytes(handle: u64) -> Vec<u8> {
    let handles = HANDLERS.lock().unwrap();
    match handles.iter().position(|h| h.id == handle) {
        Some(idx) => handles[idx].data.clone(),
        None => vec![],
    }
}

pub fn handler_read_bytes(handle: u64, count: usize) -> Vec<u8> {
    let mut handles = HANDLERS.lock().unwrap();
    if let Some(idx) = handles.iter().position(|h| h.id == handle) {
        let pos = handles[idx].pos as usize;
        let end = (pos + count).min(handles[idx].data.len());
        if pos >= handles[idx].data.len() {
            handles[idx].eof = true;
            return vec![];
        }
        let result = handles[idx].data[pos..end].to_vec();
        handles[idx].pos = end as i64;
        if end >= handles[idx].data.len() {
            handles[idx].eof = true;
        }
        result
    } else {
        vec![]
    }
}

pub fn handler_write_bytes(handle: u64, data: &[u8]) {
    let mut handles = HANDLERS.lock().unwrap();
    if let Some(idx) = handles.iter().position(|h| h.id == handle) {
        let pos = handles[idx].pos as usize;
        let needed = pos + data.len();
        if needed > handles[idx].data.len() {
            handles[idx].data.resize(needed, 0);
        }
        handles[idx].data[pos..pos + data.len()].copy_from_slice(data);
        handles[idx].pos = (pos + data.len()) as i64;
    }
}

pub fn handler_size(handle: u64) -> i64 {
    let handles = HANDLERS.lock().unwrap();
    match handles.iter().position(|h| h.id == handle) {
        Some(idx) => handles[idx].data.len() as i64,
        None => -1,
    }
}

pub fn socket_create() -> u64 {
    let mut sockets = SOCKETS.lock().unwrap();

    let new_socket: u64 = if sockets.is_empty() {
        sockets.push(0); // stdin
        sockets.push(1); // stdout
        sockets.push(2); // stderr
        3 // first available socket
    } else {
        let last_socket = sockets[sockets.len() - 1];
        last_socket + 1
    };

    sockets.push(new_socket);
    new_socket
}

pub fn socket_close(sock: u64) -> bool {
    let mut sockets = SOCKETS.lock().unwrap();
    let idx = match sockets.iter().position(|s| *s == sock) {
        Some(i) => i,
        None => return false,
    };
    sockets.remove(idx);
    true
}

pub fn socket_exist(sock: u64) -> bool {
    let sockets = SOCKETS.lock().unwrap();
    sockets.iter().position(|s| *s == sock).is_some()
}

/// Milliseconds returned by GetTickCount[64].
///
/// Default: the current tick, then the clock advances 1 ms, so every read sees
/// time moving. With `cfg.realistic_tick` it behaves like Windows: the value only
/// changes on the timer interrupt (64 Hz, 15.625 ms) and reading never advances
/// time, so back-to-back reads are equal (protectors treat a counter that moves
/// on every call as an emulator).
pub fn read_tick_count(emu: &mut emu::Emu) -> u64 {
    if emu.cfg.realistic_tick {
        let interrupts = emu.tick as u64 * 64 / 1000;
        return interrupts * 1000 / 64;
    }
    let tick = emu.tick as u64;
    advance_tick(emu, 1);
    tick
}

pub fn advance_tick(emu: &mut emu::Emu, millis: u64) {
    let time_advance = if millis == 0 {
        // Sleep(0) just yields CPU - advance by small amount (microseconds)
        1 + (emu.tick % 3) // 1-3 ticks of variance
    } else {
        millis as usize
    };

    emu.tick += time_advance;
}
