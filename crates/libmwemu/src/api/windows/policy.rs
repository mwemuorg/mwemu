use std::sync::atomic::Ordering;

use serde::{Deserialize, Serialize};

use crate::api::windows::common::nt_api;
use crate::api::windows::helper::WinApiAbi;
use crate::config::Config;
use crate::emu::Emu;
use crate::err::MwemuError;
use crate::serialization::Serialization;
use crate::windows::constants::errors::*;
use crate::windows::constants::system::{FALSE, TRUE};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApiMode {
    #[default]
    Strict,
    SimulateFail,
    SimulateOptimistic,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApiRoute {
    Address,
    Import,
    Gateway,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApiCall {
    pub library: String,
    pub name: String,
    pub address: u64,
    pub route: ApiRoute,
    pub pc: u64,
    pub stack_pointer: u64,
    pub thread_id: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApiStopReason {
    StrictPolicy,
    MissingSimulation,
    InvalidFrame,
    InvalidMemory,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApiDisposition {
    Stopped(ApiStopReason),
    SimulatedFailure,
    SimulatedOptimistic,
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

/// Bounded diagnostics and an explicit override for legacy Config flags.
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
        } else if let Some(mode) = self.mode {
            mode
        } else if cfg.skip_unimplemented {
            ApiMode::SimulateFail
        } else {
            ApiMode::Strict
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
    /// Select API policy without changing instruction or memory Banzai settings.
    pub fn set_api_mode(&mut self, mode: ApiMode) {
        self.api_policy.mode = Some(mode);
    }

    pub fn api_mode(&self) -> ApiMode {
        self.api_policy.effective_mode(&self.cfg)
    }

    pub(crate) fn check_api_stop(&self) -> Result<(), MwemuError> {
        if let Some(outcome) = &self.api_policy.last_outcome
            && let ApiDisposition::Stopped(reason) = outcome.disposition
        {
            return Err(MwemuError::new(&format!(
                "unsupported Windows API {}!{} at 0x{:x}: {:?}",
                outcome.call.library, outcome.call.name, outcome.call.pc, reason
            )));
        }
        Ok(())
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
        if emu.cfg.arch.is_x86() {
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

fn call(emu: &Emu, library: &str, name: &str, address: u64) -> ApiCall {
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
        let call = call(emu, "unknown", "unknown", 0);
        stop(emu, call, ApiStopReason::InvalidFrame);
        false
    }
}

#[derive(Clone, Copy)]
enum Simulation {
    NtFailure,
    NtYield,
    Win32Failure,
    FileTimeUtc,
}

struct Recipe {
    x86_words: u32,
    arguments: u32,
    simulation: Simulation,
}

fn recipe(library: &str, name: &str, mode: ApiMode) -> Option<Recipe> {
    let library = library.trim_end_matches(".dll");
    if ["ntdll", "kernel32", "kernelbase"].contains(&library)
        && let Some(signature) = nt_api::signature(name)
    {
        let simulation = match mode {
            ApiMode::SimulateFail => Simulation::NtFailure,
            ApiMode::SimulateOptimistic if nt_api::normalize(name) == "NtYieldExecution" => {
                Simulation::NtYield
            }
            _ => return None,
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
            ApiMode::SimulateOptimistic => Simulation::FileTimeUtc,
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

fn checked_frame(emu: &Emu, recipe: &Recipe) -> Option<u64> {
    let (next_sp, slots) = if emu.cfg.arch.is_64bits() {
        (emu.sp(), recipe.arguments)
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
        .all(|i| WinApiAbi::try_arg(emu, i as usize).is_some())
        .then_some(next_sp)
}

impl Simulation {
    fn apply(self, emu: &mut Emu) -> Result<(u64, Option<u64>), ApiStopReason> {
        match self {
            // A declared unsupported-operation failure, before any output effects.
            Self::NtFailure => Ok((STATUS_NOT_IMPLEMENTED, None)),
            // Optimistic scheduling hint: no scheduler or object state is changed.
            Self::NtYield => Ok((STATUS_SUCCESS, None)),
            Self::Win32Failure => Ok((FALSE, Some(ERROR_CALL_NOT_IMPLEMENTED))),
            Self::FileTimeUtc => {
                let input = WinApiAbi::try_arg(emu, 0).ok_or(ApiStopReason::InvalidFrame)?;
                let output = WinApiAbi::try_arg(emu, 1).ok_or(ApiStopReason::InvalidFrame)?;
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
    if emu.api_policy.stopped() {
        return;
    }
    let call = call(emu, library, name, address);
    let mode = emu.api_mode();
    if mode == ApiMode::Strict {
        if emu.cfg.skip_unimplemented
            || emu
                .api_policy
                .mode
                .is_some_and(|mode| mode != ApiMode::Strict)
        {
            log::warn!("strict Windows API policy takes precedence over simulation settings");
        }
        stop(emu, call, ApiStopReason::StrictPolicy);
        return;
    }
    let Some(recipe) = recipe(&call.library.to_ascii_lowercase(), &call.name, mode) else {
        stop(emu, call, ApiStopReason::MissingSimulation);
        return;
    };
    let Some(next_sp) = checked_frame(emu, &recipe) else {
        stop(emu, call, ApiStopReason::InvalidFrame);
        return;
    };
    let (result, last_error) = match recipe.simulation.apply(emu) {
        Ok(result) => result,
        Err(reason) => {
            stop(emu, call, reason);
            return;
        }
    };
    if let Some(error) = last_error {
        emu.set_last_error(error);
    }
    emu.set_sp(next_sp);
    WinApiAbi::set_ret(emu, result);
    emu.api_policy.simulation_count = emu.api_policy.simulation_count.saturating_add(1);
    log::warn!(
        "simulating {}!{}: {:?}, result=0x{:x}, LastError={:?} (None preserves)",
        call.library,
        call.name,
        mode,
        result,
        last_error
    );
    emu.api_policy.last_outcome = Some(ApiOutcome {
        call,
        mode,
        disposition: if mode == ApiMode::SimulateFail {
            ApiDisposition::SimulatedFailure
        } else {
            ApiDisposition::SimulatedOptimistic
        },
        result: Some(result),
        last_error,
    });
}
