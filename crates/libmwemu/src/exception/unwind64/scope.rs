use super::{DispatcherContext, FrameResult, WORD, add, load, target::unwind_to_scope};
use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::exception::{handlers, types::ExceptionRecord};
use crate::maps::mem64::Permission;
use crate::winapi::policy::{ApiStopReason, api_try, invalid_unwind, invoke_callback};
use crate::windows::constants::exceptions::{
    EXCEPTION_EXIT_UNWIND, EXCEPTION_TARGET_UNWIND, EXCEPTION_UNWINDING, ExceptionDisposition,
};

const API: &str = "__C_specific_handler";
const DWORD: u64 = size_of::<u32>() as u64;
const ENTRY_SIZE: u64 = 4 * DWORD;
const MSVC_CPP_EXCEPTION: u32 = 0xe06d_7363;

#[derive(Clone, Copy)]
struct Scope {
    begin: u32,
    end: u32,
    handler: u32,
    target: u32,
}

impl Scope {
    fn contains(self, rva: u64) -> bool {
        (u64::from(self.begin)..u64::from(self.end)).contains(&rva)
    }
}

fn dword(emu: &Emu, address: u64) -> FrameResult<u32> {
    emu.maps
        .read_dword(address)
        .ok_or(ApiStopReason::InvalidMemory)
}

struct ScopeTable {
    entries: u64,
    count: u32,
}

impl ScopeTable {
    fn read(emu: &Emu, address: u64) -> FrameResult<Self> {
        let count = dword(emu, address)?;
        let entries = add(address, DWORD)?;
        let bytes = usize::try_from(u64::from(count) * ENTRY_SIZE)
            .map_err(|_| ApiStopReason::InvalidFrame)?;
        if count != 0 && !emu.maps.can_access_range(entries, bytes, Permission::READ) {
            return Err(ApiStopReason::InvalidMemory);
        }
        Ok(Self { count, entries })
    }

    fn get(&self, emu: &Emu, index: u32) -> FrameResult<Scope> {
        let entry = add(self.entries, u64::from(index) * ENTRY_SIZE)?;
        Ok(Scope {
            begin: dword(emu, entry)?,
            end: dword(emu, entry + DWORD)?,
            handler: dword(emu, entry + 2 * DWORD)?,
            target: dword(emu, entry + 3 * DWORD)?,
        })
    }
}

fn call_funclet(
    emu: &mut Emu,
    library: &str,
    address: u64,
    frame: u64,
    pointers: Option<(u64, u64)>,
) -> FrameResult<Option<i32>> {
    if !emu.maps.can_access_range(address, 1, Permission::EXECUTE) {
        return Err(ApiStopReason::InvalidMemory);
    }
    let caller = emu.pc();
    let sp = emu.sp();
    let scratch = sp
        .checked_sub(2 * WORD)
        .ok_or(ApiStopReason::InvalidFrame)?;
    if !emu.maps.validate_write_range(scratch, 2 * WORD) {
        return Err(ApiStopReason::InvalidMemory);
    }
    let argument = if let Some((record, context)) = pointers {
        emu.maps.write_qword(scratch, record);
        emu.maps.write_qword(scratch + WORD, context);
        scratch
    } else {
        1
    };
    emu.set_sp(scratch);
    // The funclet returns to this host call before the outer language handler returns.
    // A tail call into this export can otherwise reuse the dispatcher's pending return cookie.
    emu.set_pc(scratch);
    let Some(value) = invoke_callback(emu, library, API, address, [argument, frame]) else {
        return Ok(None);
    };
    emu.set_sp(sp);
    emu.set_pc(caller);
    Ok(Some(value as i32))
}

fn run(
    emu: &mut Emu,
    library: &str,
    [record, frame, context, dispatch]: [u64; 4],
) -> FrameResult<Option<ExceptionDisposition>> {
    if !emu
        .maps
        .can_access_range(dispatch, DispatcherContext::SIZE as usize, Permission::READ)
    {
        return Err(ApiStopReason::InvalidMemory);
    }
    let pc = load(&emu.maps, dispatch + DispatcherContext::CONTROL_PC)?;
    let image = load(&emu.maps, dispatch + DispatcherContext::IMAGE_BASE)?;
    let pc = pc.checked_sub(image).ok_or(ApiStopReason::InvalidFrame)?;
    let table = ScopeTable::read(
        emu,
        load(&emu.maps, dispatch + DispatcherContext::HANDLER_DATA)?,
    )?;
    let flags = dword(emu, add(record, ExceptionRecord::FLAGS_OFFSET)?)?;
    let unwind = flags & (EXCEPTION_UNWINDING | EXCEPTION_EXIT_UNWIND) != 0;
    let target = load(&emu.maps, dispatch + DispatcherContext::TARGET_IP)?.checked_sub(image);
    for index in dword(emu, dispatch + DispatcherContext::SCOPE_INDEX)?..table.count {
        let scope = table.get(emu, index)?;
        if !scope.contains(pc) {
            continue;
        }
        if unwind {
            if flags & EXCEPTION_TARGET_UNWIND != 0
                && let Some(target) = target
            {
                if scope.target != 0 && u64::from(scope.target) == target {
                    break;
                }
                // MSVC may split one protected scope across multiple table entries.
                let mut same_scope = false;
                for other in 0..table.count {
                    let other = table.get(emu, other)?;
                    if other.contains(target)
                        && (other.handler, other.target) == (scope.handler, scope.target)
                    {
                        same_scope = true;
                        break;
                    }
                }
                if same_scope {
                    break;
                }
            }
            if scope.target == 0 {
                if !emu
                    .maps
                    .write_dword(dispatch + DispatcherContext::SCOPE_INDEX, index + 1)
                {
                    return Err(ApiStopReason::InvalidMemory);
                }
                if call_funclet(
                    emu,
                    library,
                    add(image, u64::from(scope.handler))?,
                    frame,
                    None,
                )?
                .is_none()
                {
                    return Ok(None);
                }
            }
        } else if scope.target != 0 {
            let disposition = if scope.handler == 1 {
                1
            } else {
                let Some(value) = call_funclet(
                    emu,
                    library,
                    add(image, u64::from(scope.handler))?,
                    frame,
                    Some((record, context)),
                )?
                else {
                    return Ok(None);
                };
                value
            };
            if disposition < 0 {
                return Ok(Some(ExceptionDisposition::ContinueExecution));
            }
            if disposition > 0 {
                // MSVC can destruct a pending C++ exception before entering the handler.
                if dword(emu, record)? == MSVC_CPP_EXCEPTION {
                    return Err(ApiStopReason::UnsupportedUnwind);
                }
                let ip = add(image, u64::from(scope.target))?;
                if !emu.maps.can_access_range(ip, 1, Permission::EXECUTE) {
                    return Err(ApiStopReason::InvalidFrame);
                }
                let working = load(&emu.maps, dispatch + DispatcherContext::CONTEXT_RECORD)?;
                let history = load(&emu.maps, dispatch + DispatcherContext::HISTORY_TABLE)?;
                unwind_to_scope(emu, frame, ip, record, working, history)?;
                return Ok(None);
            }
        }
    }
    Ok(Some(ExceptionDisposition::ContinueSearch))
}

pub(crate) fn c_specific_handler(emu: &mut Emu, library: &str) {
    let call = WinApi(Stdcall);
    let args = api_try!(emu, call.args(emu));
    log_red!(
        emu,
        "{library}!{API} exception_record={:#x} establisher_frame={:#x} context={:#x} dispatcher={:#x}",
        args[0],
        args[1],
        args[2],
        args[3]
    );
    match run(emu, library, args) {
        Ok(Some(disposition)) => {
            call.set_return(emu, disposition as u64);
            let caller = emu.pc();
            if emu.host_call_return != Some(caller) && emu.eh_ctx() != 0 {
                handlers::handle_return(emu, caller);
            }
        }
        Err(reason) if !emu.api_stopped() && !emu.process_terminated => {
            invalid_unwind(emu, library, API, reason)
        }
        _ => {}
    }
}
