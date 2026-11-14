use lazy_static::lazy_static;
use std::sync::Mutex;

use crate::emu;
use crate::emu::Emu;

/// Argument and result adaptation for Windows API hooks. The engine has already
/// removed the return address. On x86 an argument index denotes a DWORD slot;
/// on x64 it denotes a parameter position (including the four register slots).
pub struct WinApiAbi;

impl WinApiAbi {
    const DWORD_BYTES: u64 = size_of::<u32>() as u64;
    const QWORD_BYTES: u64 = size_of::<u64>() as u64;
    const X64_REGISTER_ARGS: usize = 4;
    const ARM64_REGISTER_ARGS: usize = 8;

    #[inline]
    pub fn arg(emu: &Emu, index: usize) -> u64 {
        Self::try_arg(emu, index).unwrap_or(0)
    }

    pub fn try_arg(emu: &Emu, index: usize) -> Option<u64> {
        if emu.cfg.arch.is_aarch64() {
            if index < Self::ARM64_REGISTER_ARGS {
                Some(emu.regs_aarch64().x[index])
            } else {
                let offset =
                    ((index - Self::ARM64_REGISTER_ARGS) as u64).checked_mul(Self::QWORD_BYTES)?;
                let address = emu.regs_aarch64().sp.checked_add(offset)?;
                emu.maps.read_qword(address)
            }
        } else if !emu.cfg.arch.is_64bits() {
            let offset = (index as u64).checked_mul(Self::DWORD_BYTES)?;
            let address = emu.regs().get_esp().checked_add(offset)?;
            emu.maps.read_dword(address).map(u64::from)
        } else {
            match index {
                0 => Some(emu.regs().rcx),
                1 => Some(emu.regs().rdx),
                2 => Some(emu.regs().r8),
                3 => Some(emu.regs().r9),
                _ => {
                    // The register slots account for the x64 shadow space.
                    let offset = (index as u64).checked_mul(Self::QWORD_BYTES)?;
                    let address = emu.regs().rsp.checked_add(offset)?;
                    emu.maps.read_qword(address)
                }
            }
        }
    }

    /// A 64-bit integer occupies two consecutive DWORD slots on x86.
    pub fn arg64(emu: &Emu, index: usize) -> u64 {
        if emu.cfg.arch.is_64bits() {
            Self::arg(emu, index)
        } else {
            let address = emu.regs().get_esp() + index as u64 * Self::DWORD_BYTES;
            emu.maps.read_qword(address).unwrap_or(0)
        }
    }

    /// Typed FP arguments need both the x64 parameter position and the x86
    /// DWORD offset: preceding doubles consume two x86 slots, but one on x64.
    pub fn f64_arg(emu: &Emu, index: usize, x86_word: usize) -> f64 {
        f64::from_bits(Self::fp_bits(emu, index, x86_word, true))
    }

    pub fn f32_arg(emu: &Emu, index: usize, x86_word: usize) -> f32 {
        f32::from_bits(Self::fp_bits(emu, index, x86_word, false) as u32)
    }

    fn fp_bits(emu: &Emu, index: usize, x86_word: usize, double: bool) -> u64 {
        if !emu.cfg.arch.is_64bits() {
            if double {
                Self::arg64(emu, x86_word)
            } else {
                Self::arg(emu, x86_word)
            }
        } else if index < Self::X64_REGISTER_ARGS {
            match index {
                0 => emu.regs().xmm0 as u64,
                1 => emu.regs().xmm1 as u64,
                2 => emu.regs().xmm2 as u64,
                _ => emu.regs().xmm3 as u64,
            }
        } else {
            Self::arg(emu, index)
        }
    }

    #[inline]
    pub fn set_ret(emu: &mut Emu, value: u64) {
        if emu.cfg.arch.is_aarch64() {
            emu.regs_aarch64_mut().x[0] = value;
        } else {
            emu.regs_mut().rax = value;
        }
    }

    /// Return a QWORD in EDX:EAX on x86 and in the integer return register on x64.
    pub fn set_ret64(emu: &mut Emu, value: u64) {
        if emu.cfg.arch.is_64bits() {
            Self::set_ret(emu, value);
        } else {
            emu.regs_mut().rax = value as u32 as u64;
            emu.regs_mut().rdx = value >> u32::BITS;
        }
    }

    pub fn set_f64_ret(emu: &mut Emu, value: f64) {
        if emu.cfg.arch.is_64bits() {
            emu.regs_mut().xmm0 = value.to_bits() as u128;
        } else {
            // A normal C return pushes a new x87 value; it does not replace the
            // caller's previous ST0 (unlike the _CI* intrinsic convention).
            emu.fpu_mut().push_f64(value);
        }
    }

    /// Only stdcall hooks discard arguments. Count DWORDs, not parameters:
    /// doubles and 64-bit integers occupy two words on x86. cdecl must not call this.
    pub fn stdcall_ret(emu: &mut Emu, words: usize) {
        if !emu.cfg.arch.is_64bits() {
            for _ in 0..words {
                emu.stack_pop32(false);
            }
        }
    }
}

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

pub fn handler_close(hndl: u64) -> bool {
    let mut handles = HANDLERS.lock().unwrap();
    let idx = match handles.iter().position(|h| h.id == hndl) {
        Some(i) => i,
        None => return false,
    };
    handles.remove(idx);
    true
}

pub fn handler_print() {
    let hndls = HANDLERS.lock().unwrap();
    for h in hndls.iter() {
        log::trace!("{:x} {}", h.id, h.uri);
    }
}

pub fn handler_exist(hndl: u64) -> bool {
    let handles = HANDLERS.lock().unwrap();
    handles.iter().position(|h| h.id == hndl).is_some()
}

pub fn handler_put_bytes(hndl: u64, data: &[u8]) {
    let mut handles = HANDLERS.lock().unwrap();
    if let Some(idx) = handles.iter().position(|h| h.id == hndl) {
        handles[idx].data = data.to_vec()
    }
}

pub fn handler_get_uri(hndl: u64) -> String {
    let handles = HANDLERS.lock().unwrap();
    match handles.iter().position(|h| h.id == hndl) {
        Some(idx) => handles[idx].uri.clone(),
        None => String::new(),
    }
}

pub fn handler_find_by_uri(uri: &str) -> Option<u64> {
    let handles = HANDLERS.lock().unwrap();
    handles.iter().find(|h| h.uri == uri).map(|h| h.id)
}

pub fn handler_seek(hndl: u64, offset: i64, origin: u32) -> i64 {
    let mut handles = HANDLERS.lock().unwrap();
    if let Some(h) = handles.iter_mut().find(|h| h.id == hndl) {
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

pub fn handler_tell(hndl: u64) -> i64 {
    let handles = HANDLERS.lock().unwrap();
    handles
        .iter()
        .find(|h| h.id == hndl)
        .map(|h| h.pos)
        .unwrap_or(-1)
}

pub fn handler_eof(hndl: u64) -> bool {
    let handles = HANDLERS.lock().unwrap();
    handles
        .iter()
        .find(|h| h.id == hndl)
        .map(|h| h.eof)
        .unwrap_or(true)
}

pub fn handler_set_eof(hndl: u64) {
    let mut handles = HANDLERS.lock().unwrap();
    if let Some(h) = handles.iter_mut().find(|h| h.id == hndl) {
        h.eof = true;
    }
}

pub fn handler_get_bytes(hndl: u64) -> Vec<u8> {
    let handles = HANDLERS.lock().unwrap();
    match handles.iter().position(|h| h.id == hndl) {
        Some(idx) => handles[idx].data.clone(),
        None => vec![],
    }
}

pub fn handler_read_bytes(hndl: u64, count: usize) -> Vec<u8> {
    let mut handles = HANDLERS.lock().unwrap();
    if let Some(idx) = handles.iter().position(|h| h.id == hndl) {
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

pub fn handler_write_bytes(hndl: u64, data: &[u8]) {
    let mut handles = HANDLERS.lock().unwrap();
    if let Some(idx) = handles.iter().position(|h| h.id == hndl) {
        let pos = handles[idx].pos as usize;
        let needed = pos + data.len();
        if needed > handles[idx].data.len() {
            handles[idx].data.resize(needed, 0);
        }
        handles[idx].data[pos..pos + data.len()].copy_from_slice(data);
        handles[idx].pos = (pos + data.len()) as i64;
    }
}

pub fn handler_size(hndl: u64) -> i64 {
    let handles = HANDLERS.lock().unwrap();
    match handles.iter().position(|h| h.id == hndl) {
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
