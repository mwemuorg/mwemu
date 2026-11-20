use std::sync::atomic::Ordering;

use serde::{Deserialize, Serialize};

use crate::api::abi::unix::{ArgumentError, UnixApi};
use crate::emu::Emu;
use crate::maps::mem64::Permission;
use crate::maps::virtual_memory::PAGE_SIZE;
use crate::serialization::Serialization;

pub(crate) mod memory;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApiCall {
    pub library: String,
    pub name: String,
    pub address: u64,
    pub pc: u64,
    pub stack_pointer: u64,
    pub thread_id: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApiStopReason {
    InvalidArgument(ArgumentError),
    InvalidReturnAddress,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApiFault {
    pub call: ApiCall,
    pub reason: ApiStopReason,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum MemoryAccess {
    Read,
    Write,
    ObjectBounds,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryError {
    pub address: u64,
    pub size: u64,
    pub access: MemoryAccess,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryFault {
    pub call: ApiCall,
    pub error: MemoryError,
}

pub(crate) enum ApiError {
    Argument(ArgumentError),
    Memory(MemoryError),
}

impl From<ArgumentError> for ApiError {
    fn from(error: ArgumentError) -> Self {
        Self::Argument(error)
    }
}

impl From<MemoryError> for ApiError {
    fn from(error: MemoryError) -> Self {
        Self::Memory(error)
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ApiState {
    pub fault: Option<ApiFault>,
    // Keep the published V5 layout. NativeSnapshot stores memory faults separately.
    #[serde(skip)]
    pub memory_fault: Option<MemoryFault>,
    #[serde(skip)]
    entry: Option<ApiCall>,
}

impl ApiState {
    pub fn stopped(&self) -> bool {
        self.fault.is_some() || self.memory_fault.is_some()
    }

    /// Clearing diagnostics does not repair the frame or resume the call.
    pub fn clear_fault(&mut self) {
        self.fault = None;
        self.memory_fault = None;
        self.entry = None;
    }
}

pub(crate) fn begin_call(
    emu: &mut Emu,
    library: &str,
    name: &str,
    address: u64,
) -> Option<ApiCall> {
    let call = ApiCall {
        library: library.into(),
        name: name.into(),
        address,
        pc: emu.pc(),
        stack_pointer: emu.sp(),
        thread_id: emu.current_thread_id,
    };
    emu.unix_api.entry.replace(call)
}

pub(crate) fn end_call(emu: &mut Emu, previous: Option<ApiCall>) {
    emu.unix_api.entry = previous;
}

/// Direct gateway calls also need diagnostics, but intercepted calls already
/// retain the frame from before consuming the return address.
pub(crate) fn enter_gateway(emu: &mut Emu, library: &str, name: &str, address: u64) -> bool {
    if emu.unix_api.entry.is_some() {
        return false;
    }
    begin_call(emu, library, name, address);
    true
}

fn current_call(emu: &Emu) -> ApiCall {
    emu.unix_api.entry.clone().unwrap_or_else(|| ApiCall {
        library: "unknown".into(),
        name: "direct handler".into(),
        address: emu.pc(),
        pc: emu.pc(),
        stack_pointer: emu.sp(),
        thread_id: emu.current_thread_id,
    })
}

fn halt(emu: &mut Emu, call: &ApiCall) {
    emu.set_pc(call.pc);
    emu.set_sp(call.stack_pointer);
    emu.force_break = true;
    emu.is_running.store(0, Ordering::Relaxed);
    if emu.cfg.dump_on_exit
        && let Some(filename) = emu.cfg.dump_filename.as_deref()
    {
        Serialization::dump(emu, filename);
    }
}

fn stop(emu: &mut Emu, reason: ApiStopReason) {
    if emu.unix_api.stopped() {
        return;
    }
    let call = current_call(emu);
    log::error!(
        "stopping at {}!{} (0x{:x}): {:?}",
        call.library,
        call.name,
        call.address,
        reason
    );
    emu.unix_api.fault = Some(ApiFault {
        call: call.clone(),
        reason,
    });
    halt(emu, &call);
}

pub(crate) fn argument_fault(emu: &mut Emu, error: ArgumentError) {
    stop(emu, ApiStopReason::InvalidArgument(error));
}

pub(crate) fn fault(emu: &mut Emu, error: impl Into<ApiError>) {
    match error.into() {
        ApiError::Argument(error) => argument_fault(emu, error),
        ApiError::Memory(error) => {
            if emu.unix_api.stopped() {
                return;
            }
            let call = current_call(emu);
            log::error!(
                "stopping at {}!{} (0x{:x}): {:?}",
                call.library,
                call.name,
                call.address,
                error
            );
            emu.unix_api.memory_fault = Some(MemoryFault {
                call: call.clone(),
                error,
            });
            halt(emu, &call);
        }
    }
}

pub(crate) fn consume_return(emu: &mut Emu) -> bool {
    let result = if emu.cfg.arch.is_aarch64() {
        Some((emu.regs_aarch64().x[30], emu.sp()))
    } else {
        emu.maps
            .read_qword(emu.sp())
            .zip(emu.sp().checked_add(size_of::<u64>() as u64))
    };
    if let Some((address, sp)) = result {
        emu.gateway_return = address;
        emu.set_sp(sp);
        emu.set_pc(address);
        true
    } else {
        stop(emu, ApiStopReason::InvalidReturnAddress);
        false
    }
}

/// End a handler on a frame or memory error; internal fallible helpers use `?`.
macro_rules! api_try {
    ($emu:expr, $read:expr) => {
        match $read {
            Ok(value) => value,
            Err(error) => {
                $crate::api::unix::fault($emu, error);
                return;
            }
        }
    };
}

pub(crate) use api_try;

pub(crate) fn errno_location(emu: &mut Emu, name: &str) {
    let call = UnixApi;
    let address = errno_address(emu).unwrap_or_else(|| {
        log::error!("{name}: cannot allocate thread-local errno");
        0
    });
    log::info!(
        "{}** {} Unix API {name} => {address:#x} {}",
        emu.colors.light_red,
        emu.pos,
        emu.colors.nc
    );
    call.set_return(emu, address);
}

pub(crate) fn errno_address(emu: &mut Emu) -> Option<u64> {
    let name = format!("unix_errno_{:x}", emu.current_thread().id);
    if let Some(map) = emu.maps.get_map_by_name(&name) {
        return Some(map.get_base());
    }
    let size = size_of::<i32>() as u64;
    let base = emu.maps.alloc(size)?;
    emu.maps
        .create_map(&name, base, size, Permission::READ_WRITE)
        .ok()?;
    if !emu.maps.write_bytes(base, &0i32.to_le_bytes()) {
        emu.maps.dealloc(base);
        return None;
    }
    Some(base)
}

pub(crate) fn getpid(emu: &mut Emu) {
    let call = UnixApi;
    log::info!("{} Unix API getpid: {}", emu.pos, emu.processes.pid);
    call.set_return(emu, emu.processes.pid);
}

pub(crate) fn getppid(emu: &mut Emu) {
    let call = UnixApi;
    log::info!("{} Unix API getppid: {}", emu.pos, emu.processes.ppid);
    call.set_return(emu, emu.processes.ppid);
}

pub(crate) fn getpagesize(emu: &mut Emu) {
    let call = UnixApi;
    call.set_return(emu, PAGE_SIZE);
}

pub(crate) fn sysconf(emu: &mut Emu) {
    const DARWIN_SC_NGROUPS_MAX: i32 = 4;
    const DARWIN_SC_PAGESIZE: i32 = 29;
    const LINUX_SC_NGROUPS_MAX: i32 = 3;
    const LINUX_SC_PAGESIZE: i32 = 30;
    const EINVAL: u32 = 22;
    let call = UnixApi;
    let name = api_try!(emu, call.arg_i32(emu, 0));
    let value = match (emu.os.is_linux(), name) {
        (true, LINUX_SC_PAGESIZE) | (false, DARWIN_SC_PAGESIZE) => PAGE_SIZE,
        (true, LINUX_SC_NGROUPS_MAX) => 65536,
        (false, DARWIN_SC_NGROUPS_MAX) => 16,
        _ => {
            if let Some(address) = errno_address(emu) {
                api_try!(emu, memory::write(emu, address, &EINVAL.to_le_bytes()));
            }
            u64::MAX
        }
    };
    log::trace!("{} sysconf({name}): {value:#x}", emu.pos);
    call.set_return(emu, value);
}
