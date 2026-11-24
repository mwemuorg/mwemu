use crate::api::abi::windows::WindowsAbi::WinSyscall;
use crate::api::windows::common::{process, threads};
use crate::emu::Emu;
use crate::maps::mem64::Permission;
use crate::winapi::libraries::kernel32::file_time::nt_performance;
use crate::winapi::policy::{self, api_try};
use crate::windows::constants::errors::*;
use crate::windows::constants::process::{
    NT_CURRENT_PROCESS, PROCESS_CONSOLE_HOST_PROCESS,
    PROCESS_RAISE_UM_EXCEPTION_ON_INVALID_HANDLE_CLOSE, PROCESS_SCHEDULER_SHARED_DATA,
    PROCESS_TLS_INFORMATION,
};
use crate::windows::constants::syscalls::*;
use crate::windows::structures::TEB64;

/// `NtQueryPerformanceCounter` — RCX = counter (PLARGE_INTEGER), RDX = frequency (PLARGE_INTEGER, optional).
/// Matches the usual x64 syscall stub used from `kernelbase!QueryPerformanceCounter` → ntdll.
pub fn nt_query_performance_counter(emu: &mut Emu) {
    nt_performance(emu, WinSyscall);
}

pub fn nt_query_information_thread(emu: &mut Emu) {
    process::query_information_thread(emu, WinSyscall);
}

/// `NtTerminateProcess` — x64: ProcessHandle, ExitStatus.
pub fn nt_terminate_process(emu: &mut Emu) {
    let process_handle = emu.regs().rcx;
    let exit_status = emu.regs().rdx;

    log_orange!(
        emu,
        "syscall 0x{:x}: NtTerminateProcess handle=0x{:x} status=0x{:x}",
        WIN64_NTTERMINATEPROCESS,
        process_handle,
        exit_status
    );

    // Current process: stop emulation. Ignoring this (e.g. for `--ssdt --init`) lets ntdll spin on
    // `NtRaiseException` after a failed Ldr path instead of exiting cleanly.

    if process_handle != !0 && process_handle != 0 {
        emu.regs_mut().rax = STATUS_ACCESS_DENIED;
        return;
    }

    // Always propagate termination — including when we are nested inside a
    // `call64` context (e.g. the LdrInitializeThunk bootstrap). The previous
    // behaviour was to silently swallow `NtTerminateProcess` at depth > 0 so
    // the outer `run()` could continue with the EXE entrypoint, but that
    // hides the fact that ntdll bailed out mid-init: the EXE then runs with
    // a partially-initialised loader and crashes downstream in a way that
    // looks unrelated.
    threads::force_exit_process(emu, exit_status as u32);
    emu.regs_mut().rax = STATUS_SUCCESS;
}

/// `NtRaiseException(PEXCEPTION_RECORD ExceptionRecord, PCONTEXT ThreadContext, BOOLEAN FirstChance)`
///
/// On real Windows the kernel re-dispatches the exception or restores the thread context and
/// **never returns** to the syscall caller.  We emulate this by reading CONTEXT.Rip (and key
/// registers) from the CONTEXT parameter and redirecting execution there.
pub fn nt_raise_exception(emu: &mut Emu) {
    let exception_record = emu.regs().r10; // first arg (R10 in syscall ABI)
    let context_ptr = emu.regs().rdx;
    let first_chance = emu.regs().r8;

    let exception_code = emu.maps.read_dword(exception_record).unwrap_or(0);

    log_orange!(
        emu,
        "syscall 0x{:x}: NtRaiseException record=0x{:x}, ctx=0x{:x}, first={}, code=0x{:x}",
        WIN64_NTRAISEEXCEPTION,
        exception_record,
        context_ptr,
        first_chance,
        exception_code
    );

    // Restore integer registers and RIP from the CONTEXT structure.
    // CONTEXT64 layout: Rax=+0x78, Rcx=+0x80, Rdx=+0x88, Rbx=+0x90, Rsp=+0x98,
    // Rbp=+0xA0, Rsi=+0xA8, Rdi=+0xB0, R8=+0xB8 … R15=+0xF0, Rip=+0xF8
    if context_ptr != 0 && emu.maps.is_mapped(context_ptr + 0xF8) {
        let ctx_rip = emu.maps.read_qword(context_ptr + 0xF8).unwrap_or(0);
        let ctx_rsp = emu.maps.read_qword(context_ptr + 0x98).unwrap_or(0);
        let ctx_rbp = emu.maps.read_qword(context_ptr + 0xA0).unwrap_or(0);
        let ctx_rax = emu.maps.read_qword(context_ptr + 0x78).unwrap_or(0);
        let ctx_rbx = emu.maps.read_qword(context_ptr + 0x90).unwrap_or(0);
        let ctx_rcx = emu.maps.read_qword(context_ptr + 0x80).unwrap_or(0);
        let ctx_rdx = emu.maps.read_qword(context_ptr + 0x88).unwrap_or(0);
        let ctx_rsi = emu.maps.read_qword(context_ptr + 0xA8).unwrap_or(0);
        let ctx_rdi = emu.maps.read_qword(context_ptr + 0xB0).unwrap_or(0);
        let ctx_r8 = emu.maps.read_qword(context_ptr + 0xB8).unwrap_or(0);
        let ctx_r9 = emu.maps.read_qword(context_ptr + 0xC0).unwrap_or(0);
        let ctx_r10 = emu.maps.read_qword(context_ptr + 0xC8).unwrap_or(0);
        let ctx_r11 = emu.maps.read_qword(context_ptr + 0xD0).unwrap_or(0);
        let ctx_r12 = emu.maps.read_qword(context_ptr + 0xD8).unwrap_or(0);
        let ctx_r13 = emu.maps.read_qword(context_ptr + 0xE0).unwrap_or(0);
        let ctx_r14 = emu.maps.read_qword(context_ptr + 0xE8).unwrap_or(0);
        let ctx_r15 = emu.maps.read_qword(context_ptr + 0xF0).unwrap_or(0);

        log::trace!(
            "NtRaiseException: restoring context, RIP=0x{:x}, RSP=0x{:x}",
            ctx_rip,
            ctx_rsp
        );

        let r = emu.regs_mut();
        r.rip = ctx_rip;
        r.rsp = ctx_rsp;
        r.rbp = ctx_rbp;
        r.rax = ctx_rax;
        r.rbx = ctx_rbx;
        r.rcx = ctx_rcx;
        r.rdx = ctx_rdx;
        r.rsi = ctx_rsi;
        r.rdi = ctx_rdi;
        r.r8 = ctx_r8;
        r.r9 = ctx_r9;
        r.r10 = ctx_r10;
        r.r11 = ctx_r11;
        r.r12 = ctx_r12;
        r.r13 = ctx_r13;
        r.r14 = ctx_r14;
        r.r15 = ctx_r15;

        // Tell the execution loop not to increment RIP (we set it explicitly).
        emu.force_reload = true;
    } else {
        log::trace!("NtRaiseException: invalid context pointer, returning STATUS_ACCESS_VIOLATION");
        emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
    }
}

/// `NtQuerySecurityAttributesToken` — syscall 0x167.
/// x64: RCX=`TokenHandle`, RDX=`Attributes` (PUNICODE_STRING array, may be NULL),
///      R8=`NumberOfAttributes`, R9=`Buffer` (PVOID),
///      `[rsp+0x28]`=`Length` (ULONG), `[rsp+0x30]`=`ReturnLength` (PULONG).
///
/// Returns an empty TOKEN_SECURITY_ATTRIBUTES_INFORMATION (Version=1, AttributeCount=0).
/// Called by ntdll during LdrInitializeThunk to probe token security attributes.
pub fn nt_query_security_attributes_token(emu: &mut Emu) {
    let token_handle = emu.regs().rcx;
    let _attributes = emu.regs().rdx;
    let num_attrs = emu.regs().r8;
    let buffer = emu.regs().r9;
    let rsp = emu.regs().rsp;
    let length = emu.maps.read_dword(rsp + 0x28).unwrap_or(0) as u64;
    let return_length_ptr = emu.maps.read_qword(rsp + 0x30).unwrap_or(0);

    log_orange!(
        emu,
        "syscall 0x{:x}: NtQuerySecurityAttributesToken handle=0x{:x} num_attrs={} buf=0x{:x} len={}",
        WIN64_NTQUERYSECURITYATTRIBUTESTOKEN,
        token_handle,
        num_attrs,
        buffer,
        length
    );

    // TOKEN_SECURITY_ATTRIBUTES_INFORMATION (x64):
    //   USHORT Version         = 1  (offset 0)
    //   USHORT Reserved        = 0  (offset 2)
    //   ULONG  AttributeCount  = 0  (offset 4)
    //   PVOID  pAttributeV1    = 0  (offset 8)
    // Total: 16 bytes
    const STRUCT_SIZE: u64 = 16;

    if return_length_ptr != 0 && emu.maps.is_mapped(return_length_ptr) {
        let _ = emu.maps.write_dword(return_length_ptr, STRUCT_SIZE as u32);
    }

    if buffer == 0 || length < STRUCT_SIZE {
        emu.regs_mut().rax = STATUS_BUFFER_TOO_SMALL;
        return;
    }

    if !emu.maps.is_mapped(buffer) {
        emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
        return;
    }

    // Version = 1, Reserved = 0, AttributeCount = 0, pAttributeV1 = NULL
    let _ = emu.maps.write_word(buffer, 1); // Version
    let _ = emu.maps.write_word(buffer + 2, 0); // Reserved
    let _ = emu.maps.write_dword(buffer + 4, 0); // AttributeCount
    let _ = emu.maps.write_qword(buffer + 8, 0); // pAttributeV1

    emu.regs_mut().rax = STATUS_SUCCESS;
}

/// `NtContinue` — syscall 0x43.
/// RCX = PCONTEXT, RDX = TestAlert (BOOLEAN)
///
/// Restores CPU state from the CONTEXT structure and resumes execution at CONTEXT.Rip.
/// In our emulator this is used by `LdrInitializeThunk` to transfer control to the
/// process entry point after loader initialisation is complete.
///
/// x64 CONTEXT field offsets (relative to context base):
///   +0x30  ContextFlags   (DWORD)
///   +0x78  Rax            (QWORD)
///   +0x80  Rcx            (QWORD)
///   +0x88  Rdx            (QWORD)
///   +0x90  Rbx            (QWORD)
///   +0x98  Rsp            (QWORD)
///   +0xA0  Rbp            (QWORD)
///   +0xA8  Rsi            (QWORD)
///   +0xB0  Rdi            (QWORD)
///   +0xF8  Rip            (QWORD)
pub fn nt_continue(emu: &mut Emu) {
    let context_ptr = emu.regs().rcx;
    let _test_alert = emu.regs().rdx;

    log_orange!(
        emu,
        "syscall 0x{:x}: NtContinue context_ptr=0x{:x}",
        WIN64_NTCONTINUE,
        context_ptr
    );

    if context_ptr == 0 || !emu.maps.is_mapped(context_ptr) {
        emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
        return;
    }

    // Read register fields from the CONTEXT structure.
    let ctx_rip = emu.maps.read_qword(context_ptr + 0xF8).unwrap_or(0);
    let ctx_rsp = emu.maps.read_qword(context_ptr + 0x98).unwrap_or(0);
    let ctx_rax = emu.maps.read_qword(context_ptr + 0x78).unwrap_or(0);
    let ctx_rcx = emu.maps.read_qword(context_ptr + 0x80).unwrap_or(0);
    let ctx_rdx = emu.maps.read_qword(context_ptr + 0x88).unwrap_or(0);
    let ctx_rbx = emu.maps.read_qword(context_ptr + 0x90).unwrap_or(0);
    let ctx_rbp = emu.maps.read_qword(context_ptr + 0xA0).unwrap_or(0);
    let ctx_rsi = emu.maps.read_qword(context_ptr + 0xA8).unwrap_or(0);
    let ctx_rdi = emu.maps.read_qword(context_ptr + 0xB0).unwrap_or(0);
    let ctx_r8 = emu.maps.read_qword(context_ptr + 0xB8).unwrap_or(0);
    let ctx_r9 = emu.maps.read_qword(context_ptr + 0xC0).unwrap_or(0);
    let ctx_r10 = emu.maps.read_qword(context_ptr + 0xC8).unwrap_or(0);
    let ctx_r11 = emu.maps.read_qword(context_ptr + 0xD0).unwrap_or(0);
    let ctx_r12 = emu.maps.read_qword(context_ptr + 0xD8).unwrap_or(0);
    let ctx_r13 = emu.maps.read_qword(context_ptr + 0xE0).unwrap_or(0);
    let ctx_r14 = emu.maps.read_qword(context_ptr + 0xE8).unwrap_or(0);
    let ctx_r15 = emu.maps.read_qword(context_ptr + 0xF0).unwrap_or(0);

    // Restore integer registers.
    emu.regs_mut().rax = ctx_rax;
    emu.regs_mut().rcx = ctx_rcx;
    emu.regs_mut().rdx = ctx_rdx;
    emu.regs_mut().rbx = ctx_rbx;
    if ctx_rsp != 0 {
        emu.regs_mut().rsp = ctx_rsp;
    }
    emu.regs_mut().rbp = ctx_rbp;
    emu.regs_mut().rsi = ctx_rsi;
    emu.regs_mut().rdi = ctx_rdi;
    emu.regs_mut().r8 = ctx_r8;
    emu.regs_mut().r9 = ctx_r9;
    emu.regs_mut().r10 = ctx_r10;
    emu.regs_mut().r11 = ctx_r11;
    emu.regs_mut().r12 = ctx_r12;
    emu.regs_mut().r13 = ctx_r13;
    emu.regs_mut().r14 = ctx_r14;
    emu.regs_mut().r15 = ctx_r15;

    // Redirect execution to the address stored in CONTEXT.Rip.
    // Set force_reload so the emulator skips the automatic `rip += instruction_sz`
    // advancement that would otherwise overshoot the target by 2 bytes.
    if ctx_rip != 0 {
        emu.regs_mut().rip = ctx_rip;
        emu.force_reload = true;
    }

    emu.regs_mut().rax = STATUS_SUCCESS;
}

/// `NtCreateProfileEx` — x64 syscall 0xc3.
///
/// Creates a profiling object. Stub: returns STATUS_NOT_SUPPORTED.
/// Callers check the return but don't fatal-fail on this error.
pub fn nt_create_profile_ex(emu: &mut Emu) {
    let profile_handle = emu.regs().rcx;

    log_orange!(
        emu,
        "syscall 0x{:x}: NtCreateProfileEx out=0x{:x}",
        WIN64_NTCREATEPROFILEEX,
        profile_handle,
    );

    emu.regs_mut().rax = STATUS_NOT_SUPPORTED;
}

pub fn nt_query_information_process(emu: &mut Emu) {
    process::query_information_process(emu, WinSyscall);
}

pub(super) fn set_information_process(emu: &mut Emu) {
    let call = WinSyscall;
    let [process, class, buffer, size] = api_try!(emu, call.args(emu));
    if class as u32 == PROCESS_CONSOLE_HOST_PROCESS as u32 {
        return process::set_console_host_process(emu, call);
    }
    if class as u32 == PROCESS_RAISE_UM_EXCEPTION_ON_INVALID_HANDLE_CLOSE as u32 {
        return process::set_invalid_close_policy(emu, call);
    }
    if class as u32 == PROCESS_TLS_INFORMATION
        && process == NT_CURRENT_PROCESS
        && let Some(status) = replace_process_tls(emu, buffer, size as u32)
    {
        log_orange!(
            emu,
            "NtSetInformationProcess ProcessTlsInformation => {status:#x}"
        );
        return call.set_return(emu, status);
    }
    if class as u32 != PROCESS_SCHEDULER_SHARED_DATA || process != NT_CURRENT_PROCESS {
        return policy::unsupported_with_abi(
            emu,
            "ntdll",
            "NtSetInformationProcess",
            emu.pc(),
            call,
        );
    }
    // The loader can run without the optional shared scheduler page.
    let status = if size as u32 != size_of::<u64>() as u32 {
        STATUS_INFO_LENGTH_MISMATCH
    } else if emu.maps.read_qword(buffer).is_none() {
        STATUS_ACCESS_VIOLATION
    } else {
        STATUS_NOT_SUPPORTED
    };
    log_orange!(
        emu,
        "NtSetInformationProcess ProcessSchedulerSharedData => {status:#x}"
    );
    call.set_return(emu, status);
}

// Win11 x64 passes a 16-byte header followed by 24-byte in/out thread records.
// The DWORD at +12 is the index for ReplaceIndex and the old slot count for ReplaceVector.
fn replace_process_tls(emu: &mut Emu, buffer: u64, size: u32) -> Option<u64> {
    const HEADER: u32 = 16;
    const THREAD_RECORD: u32 = 24;
    const REPLACE_INDEX: u32 = 0;
    const REPLACE_VECTOR: u32 = 1;
    const THREAD_UPDATED: u32 = 2;

    if size < HEADER {
        return Some(STATUS_INFO_LENGTH_MISMATCH);
    }
    if !emu
        .maps
        .can_access_range(buffer, HEADER as usize, Permission::READ)
    {
        return Some(STATUS_ACCESS_VIOLATION);
    }
    let flags = emu.maps.read_dword(buffer)?;
    let operation = emu.maps.read_dword(buffer + 4)?;
    let count = emu.maps.read_dword(buffer + 8)?;
    let index_or_count = emu.maps.read_dword(buffer + 12)? as u64;
    if count
        .checked_mul(THREAD_RECORD)
        .and_then(|bytes| bytes.checked_add(HEADER))
        != Some(size)
    {
        return Some(STATUS_INFO_LENGTH_MISMATCH);
    }
    // Startup operates on one live thread. Other thread enumeration contracts follow API policy.
    if flags != 0
        || !matches!(operation, REPLACE_INDEX | REPLACE_VECTOR)
        || count != 1
        || emu
            .threads
            .iter()
            .filter(|thread| thread.exit_value.is_none())
            .count()
            != 1
    {
        return None;
    }
    if !emu
        .maps
        .can_access_range(buffer, size as usize, Permission::READ_WRITE)
    {
        return Some(STATUS_ACCESS_VIOLATION);
    }
    let entry = buffer + HEADER as u64;
    let data = entry + 8;
    let thread_id = entry + 16;
    if emu.maps.read_dword(entry)? != 0 {
        return None;
    }
    let Some(teb_slot) = emu
        .windows_teb()
        .and_then(|teb| teb.checked_add(TEB64::THREAD_LOCAL_STORAGE_POINTER_OFFSET))
    else {
        return Some(STATUS_ACCESS_VIOLATION);
    };
    let Some(vector) = emu.maps.read_qword(teb_slot) else {
        return Some(STATUS_ACCESS_VIOLATION);
    };
    let replacement = emu.maps.read_qword(data)?;
    let bytes = index_or_count * size_of::<u64>() as u64;
    let slot = if operation == REPLACE_VECTOR {
        teb_slot
    } else if vector != 0
        && let Some(slot) = vector.checked_add(bytes)
    {
        slot
    } else {
        return Some(STATUS_ACCESS_VIOLATION);
    };
    let Some(previous) = emu.maps.read_qword(slot) else {
        return Some(STATUS_ACCESS_VIOLATION);
    };
    if !emu.maps.validate_write_range(slot, size_of::<u64>() as u64) {
        return Some(STATUS_ACCESS_VIOLATION);
    }
    if operation == REPLACE_VECTOR
        && (replacement == 0
            || !emu
                .maps
                .validate_write_range(replacement, bytes.max(size_of::<u64>() as u64))
            || (vector != 0
                && bytes != 0
                && !emu.maps.copy_range(replacement, vector, bytes as usize)))
    {
        return Some(STATUS_ACCESS_VIOLATION);
    }
    emu.maps.write_qword(slot, replacement);
    emu.maps.write_qword(data, previous);
    emu.maps.write_qword(thread_id, emu.current_thread().id);
    emu.maps.write_dword(entry, THREAD_UPDATED);
    Some(STATUS_SUCCESS)
}
