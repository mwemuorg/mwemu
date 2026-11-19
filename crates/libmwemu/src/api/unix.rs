use std::sync::atomic::Ordering;

use serde::{Deserialize, Serialize};

use crate::api::abi::unix::ArgumentError;
use crate::emu::Emu;
use crate::serialization::Serialization;

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

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ApiState {
    pub fault: Option<ApiFault>,
    #[serde(skip)]
    entry: Option<ApiCall>,
}

impl ApiState {
    pub fn stopped(&self) -> bool {
        self.fault.is_some()
    }

    /// Clearing diagnostics does not repair the frame or resume the call.
    pub fn clear_fault(&mut self) {
        self.fault = None;
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

fn stop(emu: &mut Emu, reason: ApiStopReason) {
    if emu.unix_api.stopped() {
        return;
    }
    let call = emu.unix_api.entry.clone().unwrap_or_else(|| ApiCall {
        library: "unknown".into(),
        name: "direct handler".into(),
        address: emu.pc(),
        pc: emu.pc(),
        stack_pointer: emu.sp(),
        thread_id: emu.current_thread_id,
    });
    log::error!(
        "stopping at {}!{} (0x{:x}): {:?}",
        call.library,
        call.name,
        call.address,
        reason
    );
    emu.set_pc(call.pc);
    emu.set_sp(call.stack_pointer);
    emu.unix_api.fault = Some(ApiFault { call, reason });
    emu.force_break = true;
    emu.is_running.store(0, Ordering::Relaxed);
    if emu.cfg.dump_on_exit
        && let Some(filename) = emu.cfg.dump_filename.as_deref()
    {
        Serialization::dump(emu, filename);
    }
}

pub(crate) fn argument_fault(emu: &mut Emu, error: ArgumentError) {
    stop(emu, ApiStopReason::InvalidArgument(error));
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

/// End a handler on a frame error; internal fallible helpers use `?`.
macro_rules! api_try {
    ($emu:expr, $read:expr) => {
        match $read {
            Ok(value) => value,
            Err(error) => {
                $crate::api::unix::argument_fault($emu, error);
                return;
            }
        }
    };
}

pub(crate) use api_try;
