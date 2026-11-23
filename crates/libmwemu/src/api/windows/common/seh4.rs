use goblin::pe::dll_characteristic::IMAGE_DLLCHARACTERISTICS_GUARD_CF;

use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::arch::Arch;
use crate::emu::Emu;
use crate::exception::seh32;
use crate::exception::types::{ExceptionRecord, ExceptionType, MemoryAccess};
use crate::maps::mem64::Permission;
use crate::winapi::policy::{self, ApiStopReason};
use crate::windows::constants::exceptions::*;

const API: &str = "_except_handler4_common";
const TOPMOST_SCOPE: u32 = u32::MAX - 1;
const NO_GS_COOKIE: u32 = u32::MAX - 1;
const ENCODED_TABLE: u32 = 8;
const TRY_LEVEL: u32 = 12;
const FRAME_POINTER: u32 = 16;
const SCOPE_HEADER: u32 = 16;
const SCOPE_SIZE: u32 = 12;
const SCOPE_FILTER: u32 = 4;
const DWORD: u32 = size_of::<u32>() as u32;

// Saved at negative offsets from the compiler's registration record.
const EXCEPTION_POINTERS_SLOT: u32 = 4;

#[derive(Clone, Copy)]
enum Failure {
    Memory(u32, MemoryAccess),
    InvalidFrame,
    Unwind,
    Stopped,
}

fn read(emu: &Emu, addr: u32) -> Result<u32, Failure> {
    if emu
        .maps
        .can_access_range(u64::from(addr), DWORD as usize, Permission::READ)
        && let Some(value) = emu.maps.read_dword(u64::from(addr))
    {
        return Ok(value);
    }
    Err(Failure::Memory(addr, MemoryAccess::Read))
}

fn write(emu: &mut Emu, addr: u32, value: u32) -> Result<(), Failure> {
    if emu
        .maps
        .validate_write_range(u64::from(addr), u64::from(DWORD))
        && emu.maps.write_dword(u64::from(addr), value)
    {
        return Ok(());
    }
    Err(Failure::Memory(addr, MemoryAccess::Write))
}

struct ScopeFrame<'a> {
    library: &'a str,
    registration: u32,
    cookie_check: u32,
    table: u32,
}

impl ScopeFrame<'_> {
    fn frame_pointer(&self) -> u32 {
        self.registration.wrapping_add(FRAME_POINTER)
    }

    fn callback(&self, emu: &mut Emu, addr: u32) -> Result<u32, Failure> {
        if !emu
            .maps
            .can_access_range(u64::from(addr), 1, Permission::EXECUTE)
        {
            return Err(Failure::Memory(addr, MemoryAccess::Execute));
        }
        policy::invoke_callback(emu, self.library, API, u64::from(addr), [])
            .map(|result| result as u32)
            .ok_or(Failure::Stopped)
    }

    fn validate_cookies(&self, emu: &mut Emu) -> Result<(), Failure> {
        let ebp = self.frame_pointer();
        for offset in [0, 2 * DWORD] {
            let slot = self.table.wrapping_add(offset);
            let cookie = read(emu, slot)?;
            if offset == 0 && cookie == NO_GS_COOKIE {
                continue;
            }
            let xor = read(emu, slot.wrapping_add(DWORD))?;
            let value = read(emu, ebp.wrapping_add(cookie))? ^ ebp.wrapping_add(xor);
            // The caller supplies the image's __fastcall security-cookie check.
            emu.regs_mut().rcx = u64::from(value);
            self.callback(emu, self.cookie_check)?;
        }
        Ok(())
    }

    fn search(
        &self,
        emu: &mut Emu,
        record: u32,
        context: u32,
    ) -> Result<ExceptionDisposition, Failure> {
        self.validate_cookies(emu)?;
        let flags = read(
            emu,
            record.wrapping_add(ExceptionRecord::FLAGS_OFFSET as u32),
        )?;
        let mut level = read(emu, self.registration.wrapping_add(TRY_LEVEL))?;
        if flags
            & (EXCEPTION_UNWINDING
                | EXCEPTION_EXIT_UNWIND
                | EXCEPTION_TARGET_UNWIND
                | EXCEPTION_COLLIDED_UNWIND)
            != 0
        {
            return if level == TOPMOST_SCOPE {
                Ok(ExceptionDisposition::ContinueSearch)
            } else {
                Err(Failure::Unwind)
            };
        }

        let pointers = (emu.sp() as u32)
            .checked_sub(2 * DWORD)
            .ok_or(Failure::InvalidFrame)?;
        write(emu, pointers, record)?;
        write(emu, pointers + DWORD, context)?;
        write(
            emu,
            self.registration.wrapping_sub(EXCEPTION_POINTERS_SLOT),
            pointers,
        )?;
        emu.set_sp(u64::from(pointers));
        let mut revalidate = false;
        let mut disposition = ExceptionDisposition::ContinueSearch;
        while level != TOPMOST_SCOPE {
            let entry = level
                .checked_mul(SCOPE_SIZE)
                .and_then(|offset| self.table.checked_add(SCOPE_HEADER)?.checked_add(offset))
                .ok_or(Failure::InvalidFrame)?;
            let enclosing = read(emu, entry)?;
            if enclosing != TOPMOST_SCOPE && enclosing >= level {
                return Err(Failure::InvalidFrame);
            }
            let filter = read(emu, entry.wrapping_add(SCOPE_FILTER))?;
            if filter != 0 {
                emu.regs_mut().rbp = u64::from(self.frame_pointer());
                let result = self.callback(emu, filter)? as i32;
                revalidate = true;
                if result < 0 {
                    disposition = ExceptionDisposition::ContinueExecution;
                    break;
                }
                if result > 0 {
                    return Err(Failure::Unwind);
                }
            }
            level = enclosing;
        }
        if revalidate {
            self.validate_cookies(emu)?;
        }
        Ok(disposition)
    }
}

/// Models the search pass for an active x86 dispatcher frame. Accepting a scope
/// requires both OS and compiler-local unwind before control can enter its handler.
pub(crate) fn dispatch(emu: &mut Emu, library: &str, name: &str) -> bool {
    if name != API || emu.cfg.arch != Arch::X86 || !seh32::is_active(emu) {
        return false;
    }
    let call = WinApi(Cdecl);
    let args: [u64; 6] = match call.args(emu) {
        Ok(args) => args,
        Err(error) => {
            policy::argument_fault(emu, error);
            return true;
        }
    };
    let [
        cookie,
        cookie_check,
        record,
        registration,
        context,
        _dispatcher,
    ] = args.map(|arg| arg as u32);
    let sp = emu.sp();
    let regs = emu.regs();
    let nonvolatile = [regs.rbx, regs.rsi, regs.rdi, regs.rbp];
    log_red!(
        emu,
        "{library}!{API} registration={registration:#x} exception_record={record:#x} context={context:#x}"
    );
    let result =
        (|| {
            if u64::from(context) != emu.eh_ctx()
                || Some(u64::from(record)) != seh32::active_record(emu)
            {
                return Err(Failure::InvalidFrame);
            }
            if emu.pe32.as_ref().is_some_and(|pe| {
                pe.opt.dll_characteristics & IMAGE_DLLCHARACTERISTICS_GUARD_CF != 0
            }) {
                // The CRT validates indirect targets against the process CFG policy.
                return Err(Failure::Unwind);
            }
            let table = read(emu, registration.wrapping_add(ENCODED_TABLE))? ^ read(emu, cookie)?;
            ScopeFrame {
                library,
                registration,
                cookie_check,
                table,
            }
            .search(emu, record, context)
        })();
    if matches!(result, Err(Failure::Stopped)) {
        return true;
    }
    [
        emu.regs_mut().rbx,
        emu.regs_mut().rsi,
        emu.regs_mut().rdi,
        emu.regs_mut().rbp,
    ] = nonvolatile;
    emu.set_sp(sp);
    match result {
        Ok(disposition) => call.set_return(emu, disposition as u64),
        Err(Failure::Memory(addr, access)) => emu.memory_exception(
            ExceptionType::BadAddressDereferencing,
            u64::from(addr),
            if matches!(access, MemoryAccess::Execute) {
                1
            } else {
                DWORD as usize
            },
            access,
        ),
        Err(Failure::InvalidFrame) => {
            policy::invalid_unwind(emu, library, API, ApiStopReason::InvalidFrame)
        }
        Err(Failure::Unwind) => {
            policy::invalid_unwind(emu, library, API, ApiStopReason::UnsupportedUnwind)
        }
        Err(Failure::Stopped) => {}
    }
    true
}
