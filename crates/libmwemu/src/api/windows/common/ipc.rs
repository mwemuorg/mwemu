mod state;

use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::maps::mem64::Permission;
use crate::winapi::common::{
    file_names,
    memory::{writable_pointer, write_pointer},
};
use crate::winapi::policy::{api_try, unsupported};
use crate::windows::constants::{
    errors::*,
    files::*,
    process::INVALID_HANDLE_VALUE_64,
    security::{GENERIC_READ, GENERIC_WRITE},
};
pub(crate) use state::IpcState;
use state::{Endpoint, Kind};

fn finish(emu: &mut Emu, result: Result<u64, u64>, failure: u64, slots: usize) {
    let call = WinApi(Stdcall);
    let value = result.unwrap_or_else(|err| {
        emu.set_last_error(err);
        failure
    });
    call.cleanup_args(emu, slots);
    call.set_return(emu, value);
}

fn endpoint(emu: &mut Emu, channel: u64, server: bool, read: bool, write: bool) -> u64 {
    let key = emu.handle_management.insert_ipc_handle();
    emu.ipc.endpoints.insert(
        key,
        Endpoint {
            channel,
            server,
            read,
            write,
        },
    );
    u64::from(key)
}

pub(crate) fn close(emu: &mut Emu, handle: u64) -> bool {
    emu.ipc.close(handle) && emu.handle_management.close_handle(handle)
}

fn namespace(path: &str) -> Option<Result<(Kind, String), u64>> {
    let lower = path.to_ascii_lowercase();
    let unc = lower.strip_prefix(r"\\")?;
    let (host, rest) = unc.split_once('\\')?;
    let (device, name) = rest.split_once('\\')?;
    let kind = match device {
        "pipe" => Kind::Named,
        "mailslot" => Kind::Mailslot,
        _ => return None,
    };
    Some(if !matches!(host, "." | "?") {
        Err(ERROR_NOT_SUPPORTED)
    } else if name.is_empty() {
        Err(ERROR_INVALID_NAME)
    } else {
        Ok((kind, format!("{device}\\{name}")))
    })
}

pub(crate) fn open(
    emu: &mut Emu,
    path: &str,
    access: u32,
    disposition: u32,
    flags: u32,
) -> Option<Result<u64, u64>> {
    let name = namespace(path)?;
    Some((|| {
        let (kind, name) = name?;
        if disposition as usize != OPEN_EXISTING || flags as usize & FILE_FLAG_OVERLAPPED != 0 {
            return Err(ERROR_NOT_SUPPORTED);
        }
        let id = emu.ipc.named(&name).ok_or(ERROR_FILE_NOT_FOUND)?;
        let c = emu.ipc.channels.get_mut(&id).ok_or(ERROR_FILE_NOT_FOUND)?;
        let read = access & GENERIC_READ != 0;
        let write = access & GENERIC_WRITE != 0;
        if access & !(GENERIC_READ | GENERIC_WRITE) != 0 {
            return Err(ERROR_NOT_SUPPORTED);
        }
        if (read && c.mode & PIPE_ACCESS_OUTBOUND == 0)
            || (write && c.mode & PIPE_ACCESS_INBOUND == 0)
        {
            return Err(ERROR_ACCESS_DENIED);
        }
        if kind == Kind::Named && c.was_connected {
            return Err(ERROR_PIPE_BUSY);
        }
        c.was_connected = true;
        c.clients += 1;
        Ok(endpoint(emu, id, false, read, write))
    })())
}

fn read_name(emu: &Emu, pointer: u64, wide: bool, kind: Kind) -> Result<String, u64> {
    let chars = file_names::encoding(emu, wide).read(emu, pointer)?;
    let path = String::from_utf16(&chars).map_err(|_| ERROR_INVALID_NAME)?;
    let (actual, name) = namespace(&path).ok_or(ERROR_INVALID_NAME)??;
    if actual != kind {
        return Err(ERROR_INVALID_NAME);
    }
    Ok(name)
}

fn create_pipe(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let read = api_try!(emu, call.arg(emu, 0));
    let write = api_try!(emu, call.arg(emu, 1));
    let attributes = api_try!(emu, call.arg(emu, 2));
    let _size = api_try!(emu, call.arg_u32(emu, 3));
    let result = (|| {
        if read == 0 || write == 0 || !writable_pointer(emu, read) || !writable_pointer(emu, write)
        {
            return Err(ERROR_NOACCESS);
        }
        if attributes != 0 {
            return Err(ERROR_NOT_SUPPORTED);
        }
        let id = emu
            .ipc
            .create(Kind::Anonymous, None, PIPE_ACCESS_INBOUND, 0, 0)?;
        let reader = endpoint(emu, id, true, true, false);
        let writer = endpoint(emu, id, false, false, true);
        let channel = emu.ipc.channels.get_mut(&id).ok_or(ERROR_INVALID_HANDLE)?;
        channel.clients = 1;
        channel.was_connected = true;
        write_pointer(emu, read, reader);
        write_pointer(emu, write, writer);
        Ok(1)
    })();
    finish(emu, result, 0, 4);
}

fn create_named(emu: &mut Emu, wide: bool, mailslot: bool) {
    let call = WinApi(Stdcall);
    let name = api_try!(emu, call.arg(emu, 0));
    let (kind, mode, max_message, timeout, attrs, supported, slots) = if mailslot {
        let size = api_try!(emu, call.arg_u32(emu, 1));
        let timeout = api_try!(emu, call.arg_u32(emu, 2));
        (
            Kind::Mailslot,
            PIPE_ACCESS_INBOUND,
            size,
            timeout,
            api_try!(emu, call.arg(emu, 3)),
            true,
            4,
        )
    } else {
        let mode = api_try!(emu, call.arg_u32(emu, 1));
        let flags = api_try!(emu, call.arg_u32(emu, 2));
        let instances = api_try!(emu, call.arg_u32(emu, 3));
        let _output = api_try!(emu, call.arg_u32(emu, 4));
        let _input = api_try!(emu, call.arg_u32(emu, 5));
        let timeout = api_try!(emu, call.arg_u32(emu, 6));
        let attrs = api_try!(emu, call.arg(emu, 7));
        (
            Kind::Named,
            mode,
            0,
            timeout,
            attrs,
            flags == 0 && (1..=3).contains(&mode) && instances == 1,
            8,
        )
    };
    let result = read_name(emu, name, wide, kind).and_then(|name| {
        if attrs != 0 || !supported {
            return Err(ERROR_NOT_SUPPORTED);
        }
        let id = emu
            .ipc
            .create(kind, Some(name), mode, max_message, timeout)?;
        Ok(endpoint(
            emu,
            id,
            true,
            mode & PIPE_ACCESS_INBOUND != 0,
            mode & PIPE_ACCESS_OUTBOUND != 0,
        ))
    });
    finish(emu, result, INVALID_HANDLE_VALUE_64, slots);
}

fn connect(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    let overlapped = api_try!(emu, call.arg(emu, 1));
    let error = match emu
        .ipc
        .endpoint(handle)
        .filter(|e| e.server)
        .and_then(|e| emu.ipc.channels.get(&e.channel))
    {
        Some(c) if c.kind == Kind::Named && c.clients != 0 && overlapped == 0 => {
            ERROR_PIPE_CONNECTED
        }
        Some(c) if c.kind == Kind::Named => ERROR_NOT_SUPPORTED,
        _ => ERROR_INVALID_HANDLE,
    };
    finish(emu, Err(error), 0, 2);
}

fn disconnect(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let h = api_try!(emu, call.arg(emu, 0));
    let result = emu.ipc.disconnect(h).map(|()| 1);
    finish(emu, result, 0, 1);
}

fn mailslot_info(emu: &mut Emu, set: bool) {
    let call = WinApi(Stdcall);
    let h = api_try!(emu, call.arg(emu, 0));
    let key = emu.ipc.endpoint(h).filter(|e| e.server).map(|e| e.channel);
    let (result, slots) = if set {
        let timeout = api_try!(emu, call.arg_u32(emu, 1));
        (
            key.and_then(|id| emu.ipc.channels.get_mut(&id))
                .filter(|c| c.kind == Kind::Mailslot)
                .map(|c| {
                    c.timeout = timeout;
                    1
                })
                .ok_or(ERROR_INVALID_HANDLE),
            2,
        )
    } else {
        let mut outputs = [0; 4];
        for (i, p) in outputs.iter_mut().enumerate() {
            *p = api_try!(emu, call.arg(emu, i + 1));
        }
        let result = (|| {
            let c = key
                .and_then(|id| emu.ipc.channels.get(&id))
                .filter(|c| c.kind == Kind::Mailslot)
                .ok_or(ERROR_INVALID_HANDLE)?;
            if !outputs
                .iter()
                .all(|p| *p == 0 || emu.maps.validate_write_range(*p, 4))
            {
                return Err(ERROR_NOACCESS);
            }
            let values = [
                c.max_message,
                c.messages
                    .front()
                    .map_or(MAILSLOT_NO_MESSAGE, |m| m.len() as u32),
                c.messages.len() as u32,
                c.timeout,
            ];
            for (p, v) in outputs.into_iter().zip(values) {
                if p != 0 {
                    emu.maps.write_dword(p, v);
                }
            }
            Ok(1)
        })();
        (result, 5)
    };
    finish(emu, result, 0, slots);
}

fn peek(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    let buffer = api_try!(emu, call.arg(emu, 1));
    let size = api_try!(emu, call.arg_u32(emu, 2)) as usize;
    let outputs = [
        api_try!(emu, call.arg(emu, 3)),
        api_try!(emu, call.arg(emu, 4)),
        api_try!(emu, call.arg(emu, 5)),
    ];
    let result = (|| {
        let end = emu.ipc.endpoint(handle).ok_or(ERROR_INVALID_HANDLE)?;
        let c = emu
            .ipc
            .channels
            .get(&end.channel)
            .ok_or(ERROR_PIPE_NOT_CONNECTED)?;
        if c.kind == Kind::Mailslot || !end.read {
            return Err(ERROR_INVALID_HANDLE);
        }
        let queue = if end.server { &c.inbound } else { &c.outbound };
        if queue.is_empty() && (!c.server_open || c.was_connected && c.clients == 0) {
            return Err(ERROR_BROKEN_PIPE);
        }
        let n = if buffer == 0 {
            0
        } else {
            size.min(queue.len())
        };
        if !outputs
            .iter()
            .all(|p| *p == 0 || emu.maps.validate_write_range(*p, 4))
            || n != 0 && !emu.maps.validate_write_range(buffer, n as u64)
        {
            return Err(ERROR_NOACCESS);
        }
        if n != 0 {
            let data: Vec<_> = queue.iter().copied().take(n).collect();
            emu.maps.write_bytes(buffer, &data);
        }
        for (p, v) in outputs.into_iter().zip([n, queue.len(), 0]) {
            if p != 0 {
                emu.maps.write_dword(p, v as u32);
            }
        }
        Ok(1)
    })();
    finish(emu, result, 0, 6);
}

pub(crate) fn transfer(
    emu: &mut Emu,
    handle: u64,
    buffer: u64,
    size: usize,
    write: bool,
) -> Option<Result<usize, u64>> {
    let end = emu.ipc.endpoint(handle)?;
    let result = (|| {
        if write {
            if size != 0 && !emu.maps.can_access_range(buffer, size, Permission::READ) {
                return Err(ERROR_NOACCESS);
            }
            let bytes = if size == 0 {
                &[][..]
            } else {
                &emu.maps
                    .try_read_bytes(buffer, size)
                    .ok_or(ERROR_NOACCESS)?[..]
            };
            emu.ipc.write(end, bytes, emu.maps.max_alloc_size as usize)
        } else {
            let bytes = emu.ipc.read(end, size)?;
            if !bytes.is_empty() && !emu.maps.write_bytes(buffer, &bytes) {
                return Err(ERROR_NOACCESS);
            }
            Ok(bytes.len())
        }
    })();
    Some(result)
}

pub(crate) fn dispatch(api: &str, library: &str, emu: &mut Emu) -> bool {
    match api {
        "CreatePipe" => {
            log_red!(emu, "{library}!{api}");
            create_pipe(emu)
        }
        "CreateNamedPipeA" => {
            log_red!(emu, "{library}!{api}");
            create_named(emu, false, false)
        }
        "CreateNamedPipeW" => {
            log_red!(emu, "{library}!{api}");
            create_named(emu, true, false)
        }
        "CreateMailslotA" => {
            log_red!(emu, "{library}!{api}");
            create_named(emu, false, true)
        }
        "CreateMailslotW" => {
            log_red!(emu, "{library}!{api}");
            create_named(emu, true, true)
        }
        "GetMailslotInfo" => {
            log_red!(emu, "{library}!{api}");
            mailslot_info(emu, false)
        }
        "SetMailslotInfo" => {
            log_red!(emu, "{library}!{api}");
            mailslot_info(emu, true)
        }
        "DisconnectNamedPipe" => {
            log_red!(emu, "{library}!{api}");
            disconnect(emu)
        }
        "ConnectNamedPipe" => {
            log_red!(emu, "{library}!{api}");
            connect(emu)
        }
        "PeekNamedPipe" => {
            log_red!(emu, "{library}!{api}");
            peek(emu)
        }
        "CallNamedPipeA"
        | "CallNamedPipeW"
        | "GetNamedPipeAttribute"
        | "GetNamedPipeClientComputerNameA"
        | "GetNamedPipeClientComputerNameW"
        | "GetNamedPipeClientProcessId"
        | "GetNamedPipeClientSessionId"
        | "GetNamedPipeHandleStateA"
        | "GetNamedPipeHandleStateW"
        | "GetNamedPipeInfo"
        | "GetNamedPipeServerProcessId"
        | "GetNamedPipeServerSessionId"
        | "SetNamedPipeAttribute"
        | "SetNamedPipeHandleState"
        | "TransactNamedPipe"
        | "WaitNamedPipeA"
        | "WaitNamedPipeW" => unsupported(emu, library, api, emu.pc()),
        _ => return false,
    }
    true
}
