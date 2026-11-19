use std::sync::atomic::Ordering;

use serde::{Deserialize, Serialize};

use crate::api::abi::windows::{
    ArgumentError,
    CallingConvention::Stdcall,
    WindowsAbi,
    WindowsAbi::{WinApi, WinInterrupt, WinSyscall, WinSysenter},
};
use crate::api::windows::common::nt_api;
use crate::arch::Arch;
use crate::config::Config;
use crate::emu::Emu;
use crate::serialization::Serialization;
use crate::windows::constants::errors::*;
use crate::windows::constants::system::{FALSE, TRUE};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApiMode {
    #[default]
    Strict,
    SimulateFail,
    SimulateSuccess,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApiRoute {
    Address,
    Import,
    Gateway,
    Syscall,
    Kernel,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApiCall {
    pub library: String,
    pub name: String,
    /// Export/stub address, or the raw service number for a syscall.
    pub address: u64,
    pub route: ApiRoute,
    pub pc: u64,
    pub stack_pointer: u64,
    pub thread_id: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApiStopReason {
    StrictPolicy,
    UnknownCleanup,
    InvalidFrame,
    InvalidMemory,
    BanzaiDisabled,
    InvalidArgument(ArgumentError),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApiDisposition {
    Stopped(ApiStopReason),
    SimulatedFailure,
    SimulatedSuccess,
    SimulatedGeneric,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApiOutcome {
    pub call: ApiCall,
    pub mode: ApiMode,
    pub disposition: ApiDisposition,
    pub result: Option<u64>,
    /// None preserves LastError; Some records the value written by the model.
    pub last_error: Option<u64>,
}

/// Bounded diagnostics and an explicit API policy selection.
/// An unsupported stop remains pending until the caller clears it deliberately.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ApiPolicy {
    pub mode: Option<ApiMode>,
    pub last_outcome: Option<ApiOutcome>,
    pub simulation_count: u64,
    #[serde(skip)]
    pub(crate) entry: Option<ApiCall>,
}

impl ApiPolicy {
    pub fn effective_mode(&self, cfg: &Config) -> ApiMode {
        if cfg.stop_on_unimplemented {
            ApiMode::Strict
        } else {
            self.mode.unwrap_or_default()
        }
    }

    pub fn stopped(&self) -> bool {
        self.last_outcome
            .as_ref()
            .is_some_and(|outcome| matches!(outcome.disposition, ApiDisposition::Stopped(_)))
    }

    /// Clearing a stop does not repair the unsupported call or resume it.
    pub fn clear_outcome(&mut self) {
        self.last_outcome = None;
        self.entry = None;
    }
}

impl Emu {
    /// Select API policy without changing Banzai settings.
    pub fn set_api_mode(&mut self, mode: ApiMode) {
        self.api_policy.mode = Some(mode);
    }

    pub fn api_mode(&self) -> ApiMode {
        self.api_policy.effective_mode(&self.cfg)
    }
}

pub(crate) fn begin_call(emu: &mut Emu, library: &str, name: &str, address: u64, route: ApiRoute) {
    let name = if name.is_empty() {
        emu.export_indexes
            .resolve_address_module(library, address)
            .unwrap_or("")
    } else {
        name
    };
    emu.api_policy.entry = Some(ApiCall {
        library: library.to_string(),
        name: name.to_string(),
        address,
        route,
        pc: emu.pc(),
        stack_pointer: emu.sp(),
        thread_id: emu.current_thread_id,
    });
}

/// Complete diagnostic bookkeeping; handlers retain ownership of ABI cleanup.
pub(crate) fn finish_call(emu: &mut Emu) {
    if !emu.api_policy.stopped() {
        if emu.cfg.arch.is_x86()
            && !emu
                .api_policy
                .entry
                .as_ref()
                .is_some_and(|entry| matches!(entry.route, ApiRoute::Syscall | ApiRoute::Kernel))
        {
            emu.call_stack_mut().pop();
        }
        emu.api_policy.entry = None;
    }
}

pub(crate) fn enter_gateway(
    emu: &mut Emu,
    library: &str,
    name: &str,
    address: u64,
    route: ApiRoute,
) {
    if emu.api_policy.entry.is_none() {
        begin_call(emu, library, name, address, route);
    }
}

fn call_context(emu: &Emu, library: &str, name: &str, address: u64) -> ApiCall {
    if let Some(mut call) = emu.api_policy.entry.clone() {
        // The interceptor may know only the section until export resolution.
        if call.name.is_empty() {
            call.name = name.to_string();
        }
        return call;
    }
    ApiCall {
        library: library.to_string(),
        name: name.to_string(),
        address,
        route: ApiRoute::Gateway,
        pc: emu.pc(),
        stack_pointer: emu.sp(),
        thread_id: emu.current_thread_id,
    }
}

fn stop(emu: &mut Emu, call: ApiCall, reason: ApiStopReason) {
    log::error!(
        "stopping at {}!{} (0x{:x}), {:?}: {:?}",
        call.library,
        call.name,
        call.address,
        call.route,
        reason
    );
    // Interception can already have consumed the return address. Preserve the
    // entry frame for inspection, without pretending that the guest returned.
    emu.set_pc(call.pc);
    emu.set_sp(call.stack_pointer);
    emu.api_policy.last_outcome = Some(ApiOutcome {
        call,
        mode: emu.api_mode(),
        disposition: ApiDisposition::Stopped(reason),
        result: None,
        last_error: None,
    });
    emu.force_break = true;
    emu.is_running.store(0, Ordering::Relaxed);
    if emu.cfg.dump_on_exit
        && let Some(filename) = emu.cfg.dump_filename.as_deref()
    {
        Serialization::dump(emu, filename);
    }
}

pub(crate) fn argument_fault(emu: &mut Emu, error: ArgumentError) {
    let context = if !matches!(error.abi, WinApi(_))
        && !emu
            .api_policy
            .entry
            .as_ref()
            .is_some_and(|entry| entry.route == ApiRoute::Syscall)
    {
        ApiCall {
            library: "ntdll".into(),
            name: format!("syscall 0x{:x}", emu.regs().get_eax()),
            address: emu.regs().get_eax(),
            route: ApiRoute::Syscall,
            pc: emu.pc(),
            stack_pointer: emu.sp(),
            thread_id: emu.current_thread_id,
        }
    } else {
        call_context(emu, "unknown", "direct handler", emu.pc())
    };
    stop(emu, context, ApiStopReason::InvalidArgument(error));
}

/// API interception consumes the return address before invoking old handlers.
/// Validate it first and retain the original entry frame for controlled stops.
pub(crate) fn consume_return(emu: &mut Emu) -> bool {
    let return_address = if emu.cfg.arch.is_aarch64() {
        Some((emu.regs_aarch64().x[30], emu.sp()))
    } else if emu.cfg.arch.is_64bits() {
        emu.maps
            .read_qword(emu.sp())
            .zip(emu.sp().checked_add(size_of::<u64>() as u64))
    } else {
        emu.maps.read_dword(emu.sp()).map(u64::from).zip(
            emu.sp()
                .checked_add(size_of::<u32>() as u64)
                .filter(|sp| *sp <= u32::MAX as u64),
        )
    };
    if let Some((address, sp)) = return_address {
        emu.gateway_return = address;
        emu.set_sp(sp);
        emu.set_pc(address);
        true
    } else {
        let call = call_context(emu, "unknown", "unknown", 0);
        stop(emu, call, ApiStopReason::InvalidFrame);
        false
    }
}

#[derive(Clone, Copy, Debug)]
enum Simulation {
    NtFailure,
    NtSuccess,
    Win32Failure,
    FileTimeUtc,
    GenericZero,
}

struct Recipe {
    x86_words: u32,
    arguments: u32,
    simulation: Simulation,
}

fn recipe(library: &str, name: &str, mode: ApiMode, route: ApiRoute) -> Option<Recipe> {
    let library = library.trim_end_matches(".dll");
    if (matches!(route, ApiRoute::Syscall | ApiRoute::Kernel)
        || ["ntdll", "kernel32", "kernelbase"].contains(&library))
        && let Some(signature) = nt_api::signature(name)
    {
        let simulation = match mode {
            ApiMode::SimulateFail => Simulation::NtFailure,
            ApiMode::SimulateSuccess => Simulation::NtSuccess,
            ApiMode::Strict => return None,
        };
        return Some(Recipe {
            x86_words: signature.x86_words,
            arguments: signature.arguments,
            simulation,
        });
    }
    if ["kernel32", "kernelbase"].contains(&library)
        && matches!(name, "FileTimeToLocalFileTime" | "LocalFileTimeToFileTime")
    {
        let simulation = match mode {
            ApiMode::SimulateFail => Simulation::Win32Failure,
            ApiMode::SimulateSuccess => Simulation::FileTimeUtc,
            ApiMode::Strict => return None,
        };
        return Some(Recipe {
            x86_words: 2,
            arguments: 2,
            simulation,
        });
    }
    None
}

fn generic_recipe(emu: &mut Emu, name: &str, call: WindowsAbi) -> Result<Recipe, ApiStopReason> {
    let x86_words = if emu.cfg.arch.is_64bits() || !matches!(call, WinApi(_)) {
        0
    } else if name.starts_with("Nt") || name.starts_with("Zw") {
        // Fallback metadata cannot override the subsystem's signature checks.
        nt_api::signature(name)
            .ok_or(ApiStopReason::UnknownCleanup)?
            .x86_words
    } else {
        if !emu.cfg.skip_unimplemented {
            return Err(ApiStopReason::BanzaiDisabled);
        }
        if emu.banzai.stack_words(name).is_none() {
            // Also support enabling Banzai after initialization.
            emu.banzai
                .load_defaults(&emu.cfg.get_maps_folder("banzai.csv"));
        }
        emu.banzai
            .stack_words(name)
            .ok_or(ApiStopReason::UnknownCleanup)?
    };
    Ok(Recipe {
        x86_words,
        // Generic native calls do not inspect arguments or output pointers.
        arguments: 0,
        simulation: Simulation::GenericZero,
    })
}

fn checked_frame(emu: &Emu, recipe: &Recipe, call: WindowsAbi) -> Option<u64> {
    if (call == WinSyscall && !emu.cfg.arch.is_x64())
        || (matches!(call, WinSysenter | WinInterrupt) && emu.cfg.arch != Arch::X86)
    {
        return None;
    }
    let (next_sp, slots) = if emu.cfg.arch.is_64bits() {
        (emu.sp(), recipe.arguments)
    } else if !matches!(call, WinApi(_)) {
        (emu.sp(), recipe.x86_words)
    } else {
        let next_sp = emu
            .sp()
            .checked_add(recipe.x86_words as u64 * size_of::<u32>() as u64)?;
        if next_sp > u32::MAX as u64 {
            return None;
        }
        (next_sp, recipe.x86_words)
    };
    (0..slots)
        .all(|i| call.arg(emu, i as usize).is_ok())
        .then_some(next_sp)
}

impl Simulation {
    fn apply(self, emu: &mut Emu, call: WindowsAbi) -> Result<(u64, Option<u64>), ApiStopReason> {
        match self {
            // A declared unsupported-operation failure, before any output effects.
            Self::NtFailure => Ok((STATUS_NOT_IMPLEMENTED, None)),
            // Success status only: no output, object, or scheduler state is changed.
            Self::NtSuccess => Ok((STATUS_SUCCESS, None)),
            Self::Win32Failure => Ok((FALSE, Some(ERROR_CALL_NOT_IMPLEMENTED))),
            // Legacy best-effort return; output buffers and LastError stay unchanged.
            Self::GenericZero => Ok((0, None)),
            Self::FileTimeUtc => {
                let input = call.arg(emu, 0).map_err(ApiStopReason::InvalidArgument)?;
                let output = call.arg(emu, 1).map_err(ApiStopReason::InvalidArgument)?;
                let size = size_of::<u64>() as u64;
                if input == 0
                    || output == 0
                    || input == output
                    || input.checked_add(size).is_none()
                    || output.checked_add(size).is_none()
                    || !emu.maps.validate_write_range(output, size)
                {
                    return Err(ApiStopReason::InvalidMemory);
                }
                // The environment uses UTC without daylight saving. Preserve all
                // FILETIME bits and leave LastError unchanged on successful conversion.
                let value = emu
                    .maps
                    .read_qword(input)
                    .ok_or(ApiStopReason::InvalidMemory)?;
                if !emu.maps.write_qword(output, value) {
                    return Err(ApiStopReason::InvalidMemory);
                }
                Ok((TRUE, None))
            }
        }
    }
}

pub(crate) fn unsupported(emu: &mut Emu, library: &str, name: &str, address: u64) {
    unsupported_with_abi(emu, library, name, address, WinApi(Stdcall));
}

pub(crate) fn unsupported_with_abi(
    emu: &mut Emu,
    library: &str,
    name: &str,
    address: u64,
    call: WindowsAbi,
) {
    if emu.api_policy.stopped() {
        return;
    }
    let context = call_context(emu, library, name, address);
    let mode = emu.api_mode();
    if mode == ApiMode::Strict {
        if emu
            .api_policy
            .mode
            .is_some_and(|mode| mode != ApiMode::Strict)
        {
            log::warn!("strict Windows API policy takes precedence over simulation settings");
        }
        stop(emu, context, ApiStopReason::StrictPolicy);
        return;
    }
    let recipe = recipe(
        &context.library.to_ascii_lowercase(),
        &context.name,
        mode,
        context.route,
    )
    .map(Ok)
    .unwrap_or_else(|| generic_recipe(emu, &context.name, call));
    let recipe = match recipe {
        Ok(recipe) => recipe,
        Err(reason) => {
            stop(emu, context, reason);
            return;
        }
    };
    let Some(next_sp) = checked_frame(emu, &recipe, call) else {
        stop(emu, context, ApiStopReason::InvalidFrame);
        return;
    };
    let (result, last_error) = match recipe.simulation.apply(emu, call) {
        Ok(result) => result,
        Err(reason) => {
            stop(emu, context, reason);
            return;
        }
    };
    if let Some(error) = last_error {
        emu.set_last_error(error);
    }
    emu.set_sp(next_sp);
    call.set_return(emu, result);
    emu.api_policy.simulation_count = emu.api_policy.simulation_count.saturating_add(1);
    log::warn!(
        "simulating {}!{}: {:?}, model={:?}, result=0x{:x}, LastError={:?} (None preserves)",
        context.library,
        context.name,
        mode,
        recipe.simulation,
        result,
        last_error
    );
    emu.api_policy.last_outcome = Some(ApiOutcome {
        call: context,
        mode,
        disposition: if matches!(recipe.simulation, Simulation::GenericZero) {
            ApiDisposition::SimulatedGeneric
        } else if mode == ApiMode::SimulateFail {
            ApiDisposition::SimulatedFailure
        } else {
            ApiDisposition::SimulatedSuccess
        },
        result: Some(result),
        last_error,
    });
}

/// Stop a unit-returning handler on a frame error. Fallible semantic helpers
/// propagate the error to this boundary instead of returning an API status.
macro_rules! api_try {
    ($emu:expr, $read:expr) => {
        match $read {
            Ok(value) => value,
            Err(error) => {
                $crate::api::windows::policy::argument_fault($emu, error);
                return;
            }
        }
    };
}

pub(crate) use api_try;
