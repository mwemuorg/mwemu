/*
    CRT stdio and low-level I/O over the kernel32 file handles.

    A FILE* is a real ucrt stream struct in guest memory whose _file field
    holds a CRT file descriptor; each descriptor wraps a kernel32 file handle
    (what _get_osfhandle returns), so fopen'd files live in the same jailed
    file system as CreateFile. Text mode is not translated: streams behave as
    if opened with "b".

    errno is a single per-process cell, as for a single-threaded process.
*/

use lazy_static::lazy_static;
use std::io::{ErrorKind, SeekFrom};
use std::sync::Mutex;

use super::wincrt::{allocate_memory, release};
use crate::emu;
use crate::emu::object_handle::FileHandle;
use crate::windows::constants::{GENERIC_READ, GENERIC_WRITE};

const EOF: u64 = u32::MAX as u64; // (int)-1
const INVALID_HANDLE_VALUE: u64 = u64::MAX;

const ENOENT: u32 = 2;
const EBADF: u32 = 9;
const EACCES: u32 = 13;
const EEXIST: u32 = 17;
const EINVAL: u32 = 22;

const SEEK_SET: u64 = 0;
const SEEK_CUR: u64 = 1;
const SEEK_END: u64 = 2;

/// sizeof(__crt_stdio_stream_data) and its _file field.
const STREAM_SIZE: u64 = 0x58;
const STREAM_FILE_OFFSET: u64 = 0x18;

/// 0-2 are the standard streams.
const FIRST_FD: i32 = 3;

struct Stream {
    file: u64,
    fd: i32,
    append: bool,
    eof: bool,
    error: bool,
}

struct Descriptor {
    fd: i32,
    handle: u32,
}

#[derive(Default)]
struct State {
    streams: Vec<Stream>,
    descriptors: Vec<Descriptor>,
    errno: u64,
}

lazy_static! {
    static ref STATE: Mutex<State> = Mutex::new(State::default());
}

/// Run `api` if it is a CRT stdio API; false otherwise.
pub fn gateway(api: &str, emu: &mut emu::Emu) -> bool {
    match api {
        "fopen" | "_fsopen" => fopen(emu, api, false),
        "_wfopen" | "_wfsopen" => fopen(emu, api, true),
        "fopen_s" => fopen_s(emu, api, false),
        "_wfopen_s" => fopen_s(emu, api, true),
        "fclose" => fclose(emu),
        "fread" => fread(emu),
        "fwrite" => fwrite(emu),
        "fgetc" | "getc" => fgetc(emu, api),
        "fgets" => fgets(emu),
        "fseek" => fseek(emu, api, emu.regs().rdx as i32 as i64),
        "_fseeki64" => fseek(emu, api, emu.regs().rdx as i64),
        "ftell" => ftell(emu, api, true),
        "_ftelli64" => ftell(emu, api, false),
        "rewind" => rewind(emu),
        "feof" => stream_flag(emu, api, |s| s.eof),
        "ferror" => stream_flag(emu, api, |s| s.error),
        "clearerr" => clearerr(emu),
        "fflush" => fflush(emu),
        "_fileno" => fileno(emu),
        "_filelength" => filelength(emu, api, true),
        "_filelengthi64" => filelength(emu, api, false),
        "_get_osfhandle" => get_osfhandle(emu),
        "_errno" => errno_location(emu),
        "_set_errno" => set_errno_api(emu),
        "_get_errno" => get_errno_api(emu),
        _ => return false,
    }
    true
}

// --- errno ---

fn errno_cell(emu: &mut emu::Emu) -> u64 {
    let cell = STATE.lock().unwrap().errno;
    if cell != 0 && emu.maps.is_mapped(cell) {
        return cell;
    }
    let cell = allocate_memory(emu, 4).unwrap_or(0);
    emu.maps.write_dword(cell, 0);
    STATE.lock().unwrap().errno = cell;
    cell
}

fn set_errno(emu: &mut emu::Emu, value: u32) {
    let cell = errno_cell(emu);
    emu.maps.write_dword(cell, value);
}

fn errno_location(emu: &mut emu::Emu) {
    emu.regs_mut().rax = errno_cell(emu);
}

/* errno_t _set_errno(int value); */
fn set_errno_api(emu: &mut emu::Emu) {
    set_errno(emu, emu.regs().rcx as u32);
    emu.regs_mut().rax = 0;
}

/* errno_t _get_errno(int *pValue); */
fn get_errno_api(emu: &mut emu::Emu) {
    let out = emu.regs().rcx;
    if out == 0 {
        emu.regs_mut().rax = EINVAL as u64;
        return;
    }
    let cell = errno_cell(emu);
    let value = emu.maps.read_dword(cell).unwrap_or(0);
    emu.maps.write_dword(out, value);
    emu.regs_mut().rax = 0;
}

// --- streams ---

/// kernel32 handle behind a stream.
fn stream_handle(file: u64) -> Option<u32> {
    let state = STATE.lock().unwrap();
    let fd = state.streams.iter().find(|s| s.file == file)?.fd;
    state
        .descriptors
        .iter()
        .find(|d| d.fd == fd)
        .map(|d| d.handle)
}

fn with_stream<T>(file: u64, f: impl FnOnce(&mut Stream) -> T) -> Option<T> {
    let mut state = STATE.lock().unwrap();
    state.streams.iter_mut().find(|s| s.file == file).map(f)
}

fn read_text(emu: &emu::Emu, ptr: u64, wide: bool) -> String {
    if wide {
        emu.maps.read_wide_string(ptr)
    } else {
        emu.maps.read_string(ptr)
    }
}

/// Open `path` with an fopen mode string; the new FILE* or an errno value.
fn open(emu: &mut emu::Emu, path: &str, mode: &str) -> Result<u64, u32> {
    let update = mode.contains('+');
    let (disposition, mut access, append) = match mode.chars().next() {
        Some('r') => (3, GENERIC_READ, false), // OPEN_EXISTING
        Some('w') if mode.contains('x') => (1, GENERIC_WRITE, false), // CREATE_NEW
        Some('w') => (2, GENERIC_WRITE, false), // CREATE_ALWAYS
        Some('a') => (4, GENERIC_WRITE, true), // OPEN_ALWAYS
        _ => return Err(EINVAL),
    };
    if update {
        access |= GENERIC_READ | GENERIC_WRITE;
    }
    let file = FileHandle::new(path.to_string(), access, disposition, 0, 0).map_err(|e| match e
        .downcast_ref::<std::io::Error>()
        .map(|e| e.kind())
    {
        Some(ErrorKind::NotFound) => ENOENT,
        Some(ErrorKind::AlreadyExists) => EEXIST,
        Some(ErrorKind::PermissionDenied) => EACCES,
        _ => EINVAL,
    })?;
    let Some(stream) = allocate_memory(emu, STREAM_SIZE) else {
        return Err(EINVAL);
    };
    let handle = emu.handle_management.insert_file_handle(file);

    let mut state = STATE.lock().unwrap();
    let fd = (FIRST_FD..)
        .find(|fd| state.descriptors.iter().all(|d| d.fd != *fd))
        .unwrap();
    state.descriptors.push(Descriptor { fd, handle });
    state.streams.push(Stream {
        file: stream,
        fd,
        append,
        eof: false,
        error: false,
    });
    drop(state);

    emu.maps.write_bytes(stream, &[0; STREAM_SIZE as usize]);
    emu.maps.write_dword(stream + STREAM_FILE_OFFSET, fd as u32);
    Ok(stream)
}

/* FILE *fopen(const char *filename, const char *mode); */
fn fopen(emu: &mut emu::Emu, api: &str, wide: bool) {
    let path = read_text(emu, emu.regs().rcx, wide);
    let mode = read_text(emu, emu.regs().rdx, wide);
    let result = open(emu, &path, &mode);
    log_red!(emu, "wincrt!{} '{}' '{}' = {:x?}", api, path, mode, result);
    match result {
        Ok(stream) => emu.regs_mut().rax = stream,
        Err(errno) => {
            set_errno(emu, errno);
            emu.regs_mut().rax = 0;
        }
    }
}

/* errno_t fopen_s(FILE **pFile, const char *filename, const char *mode); */
fn fopen_s(emu: &mut emu::Emu, api: &str, wide: bool) {
    let out = emu.regs().rcx;
    let path_ptr = emu.regs().rdx;
    let mode_ptr = emu.regs().r8;
    if out == 0 || path_ptr == 0 || mode_ptr == 0 {
        set_errno(emu, EINVAL);
        emu.regs_mut().rax = EINVAL as u64;
        return;
    }
    let path = read_text(emu, path_ptr, wide);
    let mode = read_text(emu, mode_ptr, wide);
    let result = open(emu, &path, &mode);
    log_red!(emu, "wincrt!{} '{}' '{}' = {:x?}", api, path, mode, result);
    match result {
        Ok(stream) => {
            emu.maps.write_qword(out, stream);
            emu.regs_mut().rax = 0;
        }
        Err(errno) => {
            emu.maps.write_qword(out, 0);
            set_errno(emu, errno);
            emu.regs_mut().rax = errno as u64;
        }
    }
}

/* int fclose(FILE *stream); */
fn fclose(emu: &mut emu::Emu) {
    let file = emu.regs().rcx;
    let removed = {
        let mut state = STATE.lock().unwrap();
        match state.streams.iter().position(|s| s.file == file) {
            Some(idx) => {
                let fd = state.streams.remove(idx).fd;
                let d = state.descriptors.iter().position(|d| d.fd == fd);
                d.map(|d| state.descriptors.remove(d).handle)
            }
            None => None,
        }
    };
    log_red!(emu, "wincrt!fclose 0x{:x} ok: {}", file, removed.is_some());
    match removed {
        Some(handle) => {
            emu.handle_management.remove_file_handle(handle);
            release(emu, file);
            emu.regs_mut().rax = 0;
        }
        None => {
            set_errno(emu, EINVAL);
            emu.regs_mut().rax = EOF;
        }
    }
}

/// Read up to `len` bytes from a stream into guest memory; bytes read.
fn read_into(emu: &mut emu::Emu, file: u64, dst: u64, len: usize) -> Option<usize> {
    let handle = stream_handle(file)?;
    let mut buf = vec![0u8; len];
    let mut done = 0;
    let mut failed = false;
    if let Some(fh) = emu.handle_management.get_mut_file_handle(handle) {
        while done < len {
            match fh.read(&mut buf[done..]) {
                Ok(0) => break,
                Ok(n) => done += n,
                Err(_) => {
                    failed = true;
                    break;
                }
            }
        }
    }
    emu.maps.write_bytes(dst, &buf[..done]);
    with_stream(file, |s| {
        s.error |= failed;
        s.eof |= !failed && done < len;
    });
    Some(done)
}

/* size_t fread(void *buffer, size_t size, size_t count, FILE *stream); */
fn fread(emu: &mut emu::Emu) {
    let buf = emu.regs().rcx;
    let size = emu.regs().rdx;
    let count = emu.regs().r8;
    let file = emu.regs().r9;
    let total = size.saturating_mul(count) as usize;
    let items = if total == 0 {
        Some(0)
    } else {
        read_into(emu, file, buf, total).map(|n| n as u64 / size)
    };
    log_red!(
        emu,
        "wincrt!fread buf:0x{:x} size:{} count:{} fp:0x{:x} = {:?}",
        buf,
        size,
        count,
        file,
        items
    );
    if items.is_none() {
        set_errno(emu, EINVAL);
    }
    emu.regs_mut().rax = items.unwrap_or(0);
}

/* size_t fwrite(const void *buffer, size_t size, size_t count, FILE *stream); */
fn fwrite(emu: &mut emu::Emu) {
    let buf = emu.regs().rcx;
    let size = emu.regs().rdx;
    let count = emu.regs().r8;
    let file = emu.regs().r9;
    let total = size.saturating_mul(count) as usize;
    let data = emu.maps.try_read_bytes(buf, total).map(|b| b.to_vec());
    let append = with_stream(file, |s| s.append).unwrap_or(false);
    let written = match (stream_handle(file), data) {
        (Some(handle), Some(data)) => {
            emu.handle_management
                .get_mut_file_handle(handle)
                .and_then(|fh| {
                    if append {
                        fh.seek(SeekFrom::End(0)).ok()?;
                    }
                    fh.write(&data).ok()
                })
        }
        _ => None,
    };
    log_red!(
        emu,
        "wincrt!fwrite buf:0x{:x} size:{} count:{} fp:0x{:x} = {:?}",
        buf,
        size,
        count,
        file,
        written
    );
    match written {
        Some(n) if size != 0 => emu.regs_mut().rax = n as u64 / size,
        Some(_) => emu.regs_mut().rax = 0,
        None => {
            with_stream(file, |s| s.error = true);
            set_errno(emu, EINVAL);
            emu.regs_mut().rax = 0;
        }
    }
}

/* int fgetc(FILE *stream); */
fn fgetc(emu: &mut emu::Emu, api: &str) {
    let file = emu.regs().rcx;
    let Some(handle) = stream_handle(file) else {
        set_errno(emu, EINVAL);
        emu.regs_mut().rax = EOF;
        return;
    };
    let mut byte = [0u8; 1];
    let read = emu
        .handle_management
        .get_mut_file_handle(handle)
        .and_then(|fh| fh.read(&mut byte).ok());
    log_red!(
        emu,
        "wincrt!{} 0x{:x} = {:?}",
        api,
        file,
        read.map(|_| byte[0])
    );
    match read {
        Some(1) => emu.regs_mut().rax = byte[0] as u64,
        _ => {
            with_stream(file, |s| s.eof = true);
            emu.regs_mut().rax = EOF;
        }
    }
}

/* char *fgets(char *str, int numChars, FILE *stream); */
fn fgets(emu: &mut emu::Emu) {
    let dst = emu.regs().rcx;
    let max = emu.regs().rdx as i32;
    let file = emu.regs().r8;
    let Some(handle) = stream_handle(file).filter(|_| dst != 0 && max > 0) else {
        set_errno(emu, EINVAL);
        emu.regs_mut().rax = 0;
        return;
    };
    let mut line = Vec::new();
    if let Some(fh) = emu.handle_management.get_mut_file_handle(handle) {
        let mut byte = [0u8; 1];
        while line.len() + 1 < max as usize {
            match fh.read(&mut byte) {
                Ok(1) => {
                    line.push(byte[0]);
                    if byte[0] == b'\n' {
                        break;
                    }
                }
                _ => break,
            }
        }
    }
    let at_eof = !line.ends_with(b"\n") && line.len() + 1 < max as usize;
    with_stream(file, |s| s.eof |= at_eof);
    log_red!(
        emu,
        "wincrt!fgets 0x{:x} = {:?}",
        file,
        String::from_utf8_lossy(&line)
    );
    if line.is_empty() {
        emu.regs_mut().rax = 0;
        return;
    }
    line.push(0);
    emu.maps.write_bytes(dst, &line);
    emu.regs_mut().rax = dst;
}

/* int fseek(FILE *stream, long offset, int origin); */
fn fseek(emu: &mut emu::Emu, api: &str, offset: i64) {
    let file = emu.regs().rcx;
    let origin = emu.regs().r8 as u32 as u64;
    let target = match origin {
        SEEK_SET if offset >= 0 => Some(SeekFrom::Start(offset as u64)),
        SEEK_CUR => Some(SeekFrom::Current(offset)),
        SEEK_END => Some(SeekFrom::End(offset)),
        _ => None,
    };
    let pos = match (stream_handle(file), target) {
        (Some(handle), Some(target)) => emu
            .handle_management
            .get_mut_file_handle(handle)
            .and_then(|fh| fh.seek(target).ok()),
        _ => None,
    };
    log_red!(
        emu,
        "wincrt!{} 0x{:x} {} {} = {:?}",
        api,
        file,
        offset,
        origin,
        pos
    );
    if pos.is_some() {
        with_stream(file, |s| s.eof = false);
        emu.regs_mut().rax = 0;
    } else {
        set_errno(emu, EINVAL);
        emu.regs_mut().rax = EOF;
    }
}

/* long ftell(FILE *stream); __int64 _ftelli64(FILE *stream); */
fn ftell(emu: &mut emu::Emu, api: &str, long: bool) {
    let file = emu.regs().rcx;
    let pos = stream_handle(file).and_then(|handle| {
        emu.handle_management
            .get_mut_file_handle(handle)
            .map(|fh| fh.get_position())
    });
    log_red!(emu, "wincrt!{} 0x{:x} = {:?}", api, file, pos);
    match pos {
        Some(pos) if long && pos > i32::MAX as u64 => {
            set_errno(emu, EINVAL);
            emu.regs_mut().rax = EOF;
        }
        Some(pos) => emu.regs_mut().rax = pos,
        None => {
            set_errno(emu, EINVAL);
            emu.regs_mut().rax = if long { EOF } else { u64::MAX };
        }
    }
}

/* void rewind(FILE *stream); */
fn rewind(emu: &mut emu::Emu) {
    let file = emu.regs().rcx;
    log_red!(emu, "wincrt!rewind 0x{:x}", file);
    if let Some(handle) = stream_handle(file)
        && let Some(fh) = emu.handle_management.get_mut_file_handle(handle)
    {
        let _ = fh.seek(SeekFrom::Start(0));
    }
    with_stream(file, |s| {
        s.eof = false;
        s.error = false;
    });
}

/* int feof(FILE *stream); int ferror(FILE *stream); */
fn stream_flag(emu: &mut emu::Emu, api: &str, flag: fn(&Stream) -> bool) {
    let file = emu.regs().rcx;
    let set = with_stream(file, |s| flag(s));
    log_red!(emu, "wincrt!{} 0x{:x} = {:?}", api, file, set);
    if set.is_none() {
        set_errno(emu, EINVAL);
    }
    emu.regs_mut().rax = set.unwrap_or(false) as u64;
}

/* void clearerr(FILE *stream); */
fn clearerr(emu: &mut emu::Emu) {
    let file = emu.regs().rcx;
    log_red!(emu, "wincrt!clearerr 0x{:x}", file);
    with_stream(file, |s| {
        s.eof = false;
        s.error = false;
    });
}

/* int fflush(FILE *stream); — writes are unbuffered, NULL flushes all */
fn fflush(emu: &mut emu::Emu) {
    let file = emu.regs().rcx;
    let ok = file == 0 || stream_handle(file).is_some();
    log_red!(emu, "wincrt!fflush 0x{:x}", file);
    emu.regs_mut().rax = if ok { 0 } else { EOF };
}

/* int _fileno(FILE *stream); */
fn fileno(emu: &mut emu::Emu) {
    let file = emu.regs().rcx;
    let fd = with_stream(file, |s| s.fd);
    log_red!(emu, "wincrt!_fileno 0x{:x} = {:?}", file, fd);
    match fd {
        Some(fd) => emu.regs_mut().rax = fd as u64,
        None => {
            set_errno(emu, EINVAL);
            emu.regs_mut().rax = EOF;
        }
    }
}

fn fd_handle(fd: i32) -> Option<u32> {
    let state = STATE.lock().unwrap();
    state
        .descriptors
        .iter()
        .find(|d| d.fd == fd)
        .map(|d| d.handle)
}

/* long _filelength(int fd); __int64 _filelengthi64(int fd); */
fn filelength(emu: &mut emu::Emu, api: &str, long: bool) {
    let fd = emu.regs().rcx as i32;
    let len = fd_handle(fd).and_then(|handle| {
        emu.handle_management
            .get_mut_file_handle(handle)
            .and_then(|fh| fh.length().ok())
    });
    log_red!(emu, "wincrt!{} fd {} = {:?}", api, fd, len);
    match len {
        Some(len) if long && len > i32::MAX as u64 => {
            set_errno(emu, EINVAL);
            emu.regs_mut().rax = EOF;
        }
        Some(len) => emu.regs_mut().rax = len,
        None => {
            set_errno(emu, EBADF);
            emu.regs_mut().rax = if long { EOF } else { u64::MAX };
        }
    }
}

/* intptr_t _get_osfhandle(int fd); */
fn get_osfhandle(emu: &mut emu::Emu) {
    let fd = emu.regs().rcx as i32;
    let handle = fd_handle(fd);
    log_red!(emu, "wincrt!_get_osfhandle fd {} = {:?}", fd, handle);
    match handle {
        Some(handle) => emu.regs_mut().rax = handle as u64,
        None => {
            set_errno(emu, EBADF);
            emu.regs_mut().rax = INVALID_HANDLE_VALUE;
        }
    }
}
