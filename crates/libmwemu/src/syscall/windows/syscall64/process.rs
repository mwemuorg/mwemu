use std::sync::atomic::Ordering;

use crate::emu::Emu;
use crate::windows::constants::*;
use crate::windows::structures::ProcessBasicInformation64;

fn is_current_process_handle(h: u64) -> bool {
    h == !0 || h == 0xffff_ffff_ffff_fffe
}

/// `NtAccessCheck` — syscall 0x0.  x64: RCX `SecurityDescriptor`, RDX `ClientToken`,
/// R8 `DesiredAccess`, R9 `GenericMapping`, then `[rsp+0x28]` `PrivilegeSet`,
/// `[rsp+0x30]` `PrivilegeSetLength`, `[rsp+0x38]` `GrantedAccess`, `[rsp+0x40]` `AccessStatus`.
///
/// Stub: grant the requested access; ntdll uses this during loader security checks.
pub fn nt_access_check(emu: &mut Emu) {
    let _security_descriptor = emu.regs().rcx;
    let _client_token = emu.regs().rdx;
    let desired_access = emu.regs().r8 as u32;
    let _generic_mapping = emu.regs().r9;
    let rsp = emu.regs().rsp;
    let privilege_set = emu.maps.read_qword(rsp + 0x28).unwrap_or(0);
    let privilege_set_length_ptr = emu.maps.read_qword(rsp + 0x30).unwrap_or(0);
    let granted_access_ptr = emu.maps.read_qword(rsp + 0x38).unwrap_or(0);
    let access_status_ptr = emu.maps.read_qword(rsp + 0x40).unwrap_or(0);

    log_orange!(
        emu,
        "syscall 0x{:x}: NtAccessCheck desired_access: 0x{:x}, granted_out: 0x{:x}, status_out: 0x{:x}",
        WIN64_NTACCESSCHECK,
        desired_access,
        granted_access_ptr,
        access_status_ptr
    );

    if granted_access_ptr == 0 || access_status_ptr == 0 {
        emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
        return;
    }
    if !emu.maps.is_mapped(granted_access_ptr) || !emu.maps.is_mapped(access_status_ptr) {
        emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
        return;
    }
    let _ = emu.maps.write_dword(granted_access_ptr, desired_access);
    let _ = emu
        .maps
        .write_dword(access_status_ptr, STATUS_SUCCESS as u32);
    if privilege_set_length_ptr != 0 && emu.maps.is_mapped(privilege_set_length_ptr) {
        let len = emu.maps.read_dword(privilege_set_length_ptr).unwrap_or(0) as usize;
        if len > 0 && privilege_set != 0 && emu.maps.is_mapped(privilege_set) {
            let cap = len.min(0x1000);
            emu.maps.memset(privilege_set, 0, cap);
        }
        let _ = emu.maps.write_dword(privilege_set_length_ptr, 0);
    }

    emu.regs_mut().rax = STATUS_SUCCESS;
}

/// `NtQueryInformationProcess` — x64: RCX..R9 + 5th arg (`ReturnLength`) at `[rsp+0x28]`.
pub fn nt_query_information_process(emu: &mut Emu) {
    let process_handle = emu.regs().rcx;
    let process_information_class = emu.regs().rdx;
    let process_information = emu.regs().r8;
    let process_information_length = emu.regs().r9;
    let rsp = emu.regs().rsp;
    let return_length_ptr = emu.maps.read_qword(rsp + 0x28).unwrap_or(0);

    log_orange!(
        emu,
        "syscall 0x{:x}: NtQueryInformationProcess process_handle: 0x{:x}, process_information_class: 0x{:x}, process_information: 0x{:x}, process_information_length: 0x{:x}, return_length_ptr: 0x{:x}",
        WIN64_NTQUERYINFORMATIONPROCESS,
        process_handle,
        process_information_class,
        process_information,
        process_information_length,
        return_length_ptr
    );

    if process_information_class == PROCESS_INFORMATION_CLASS_PROCESS_BASIC_INFORMATION {
        if process_information == 0 {
            emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
            return;
        }

        if !emu.maps.is_mapped(process_information) {
            emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
            return;
        }

        if process_information_length < ProcessBasicInformation64::size() {
            emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
            return;
        }

        let peb_base = emu.maps.get_mem("peb").get_base();
        let process_info = ProcessBasicInformation64 {
            Reserved1: 0,
            PebBaseAddress: peb_base,
            Reserved2: [0, 0],
            UniqueProcessId: 4,
            Reserved3: 0,
        };
        process_info.save(process_information, &mut emu.maps);

        if return_length_ptr != 0
            && !emu
                .maps
                .write_qword(return_length_ptr, ProcessBasicInformation64::size())
            {
                emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
                return;
            }

        emu.regs_mut().rax = STATUS_SUCCESS;
        return;
    }

    if process_information_class == PROCESS_INFORMATION_CLASS_PROCESS_COOKIE {
        // Used by ntdll init paths; output is a 32-bit cookie (see SDK / `ProcessCookie`).
        if process_information == 0 {
            emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
            return;
        }
        if process_information_length < 4 {
            emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
            return;
        }
        if !emu.maps.is_mapped(process_information) {
            emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
            return;
        }
        if !emu.maps.write_dword(process_information, 0x01234567) {
            emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
            return;
        }
        if return_length_ptr != 0
            && !emu.maps.write_dword(return_length_ptr, 4) {
                emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
                return;
            }
        emu.regs_mut().rax = STATUS_SUCCESS;
        return;
    }

    if process_information_class == PROCESS_INFORMATION_CLASS_PROCESS_DEBUG_PORT {
        if process_information_length < 8 {
            emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
            return;
        }
        if !emu.maps.is_mapped(process_information) {
            emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
            return;
        }
        if !emu.maps.write_qword(process_information, 0) {
            emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
            return;
        }
        if return_length_ptr != 0
            && !emu.maps.write_qword(return_length_ptr, 8) {
                emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
                return;
            }
        emu.regs_mut().rax = STATUS_SUCCESS;
        return;
    }

    // ProcessWow64Information (0x25): returns ULONG_PTR = 0 for native 64-bit processes.
    if process_information_class == 0x25 {
        if process_information_length < 8
            || process_information == 0
            || !emu.maps.is_mapped(process_information)
        {
            emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
            return;
        }
        let _ = emu.maps.write_qword(process_information, 0);
        if return_length_ptr != 0 {
            let _ = emu.maps.write_dword(return_length_ptr, 8);
        }
        emu.regs_mut().rax = STATUS_SUCCESS;
        return;
    }

    // ProcessImageFileName (0x1b): return a UNICODE_STRING with a fake path.
    if process_information_class == 0x1b {
        // Minimum buffer: UNICODE_STRING header (8 bytes on x64: Length+MaxLength+Buffer ptr = actually
        // it is 4+4+8=16 bytes on x64) + the string data. Return STATUS_INFO_LENGTH_MISMATCH if too small.
        const HDR: u32 = 16; // sizeof(UNICODE_STRING) on x64
        if process_information_length < HDR as u64 {
            if return_length_ptr != 0 && emu.maps.is_mapped(return_length_ptr) {
                let _ = emu.maps.write_dword(return_length_ptr, HDR);
            }
            emu.regs_mut().rax = STATUS_INFO_LENGTH_MISMATCH;
            return;
        }
        if process_information == 0 || !emu.maps.is_mapped(process_information) {
            emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
            return;
        }
        // Write an empty UNICODE_STRING (Length=0, MaxLength=0, Buffer=0).
        let _ = emu.maps.write_word(process_information, 0);
        let _ = emu.maps.write_word(process_information + 2, 0);
        let _ = emu.maps.write_qword(process_information + 8, 0);
        if return_length_ptr != 0 && emu.maps.is_mapped(return_length_ptr) {
            let _ = emu.maps.write_dword(return_length_ptr, HDR);
        }
        emu.regs_mut().rax = STATUS_SUCCESS;
        return;
    }

    emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
}

/// `NtQueryPerformanceCounter` — RCX = counter (PLARGE_INTEGER), RDX = frequency (PLARGE_INTEGER, optional).
/// Matches the usual x64 syscall stub used from `kernelbase!QueryPerformanceCounter` → ntdll.
pub fn nt_query_performance_counter(emu: &mut Emu) {
    let counter_ptr = emu.regs().rcx;
    let freq_ptr = emu.regs().rdx;

    log_orange!(
        emu,
        "syscall 0x{:x}: NtQueryPerformanceCounter counter: 0x{:x} freq: 0x{:x}",
        WIN64_NTQUERYPERFORMANCECOUNTER,
        counter_ptr,
        freq_ptr
    );

    if counter_ptr == 0 {
        emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
        return;
    }
    if !emu.maps.is_mapped(counter_ptr) {
        emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
        return;
    }

    let counter_value = (emu.tick as u64).saturating_mul(1000);
    if !emu.maps.write_qword(counter_ptr, counter_value) {
        emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
        return;
    }

    if freq_ptr != 0 {
        if !emu.maps.is_mapped(freq_ptr) {
            emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
            return;
        }
        // Fake ~10 MHz; order of magnitude typical for QPC frequency on Windows.
        if !emu.maps.write_qword(freq_ptr, 10_000_000) {
            emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
            return;
        }
    }

    emu.regs_mut().rax = STATUS_SUCCESS;
}

/// `NtQueryInformationThread` — x64: 5th arg at `[rsp+0x28]`.
pub fn nt_query_information_thread(emu: &mut Emu) {
    let _thread_handle = emu.regs().rcx;
    let thread_class = emu.regs().rdx;
    let thread_info = emu.regs().r8;
    let thread_info_len = emu.regs().r9;
    let rsp = emu.regs().rsp;
    let return_length_ptr = emu.maps.read_qword(rsp + 0x28).unwrap_or(0);

    log_orange!(
        emu,
        "syscall 0x{:x}: NtQueryInformationThread class: 0x{:x}, out: 0x{:x}, len: 0x{:x}",
        WIN64_NTQUERYINFORMATIONTHREAD,
        thread_class,
        thread_info,
        thread_info_len
    );

    if thread_info == 0 && thread_info_len > 0 {
        emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
        return;
    }

    if thread_class == THREAD_INFORMATION_CLASS_THREAD_BASIC_INFORMATION {
        const TBI_SIZE: u64 = 48;
        if thread_info_len < TBI_SIZE {
            emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
            return;
        }
        if !emu.maps.is_mapped(thread_info) {
            emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
            return;
        }
        for i in 0..TBI_SIZE {
            let _ = emu.maps.write_byte(thread_info + i, 0);
        }
        let teb_base = emu.maps.get_mem("teb").get_base();
        let _ = emu.maps.write_qword(thread_info + 8, teb_base);
        // ClientId.UniqueProcess and .UniqueThread must be non-zero;
        // ntdll may dereference them when resolving thread/process context.
        let _ = emu.maps.write_qword(thread_info + 0x10, 0x1000); // fake PID
        let _ = emu.maps.write_qword(thread_info + 0x18, 0x1004); // fake TID
        if return_length_ptr != 0 {
            let _ = emu.maps.write_qword(return_length_ptr, TBI_SIZE);
        }
        emu.regs_mut().rax = STATUS_SUCCESS;
        return;
    }

    if thread_class == THREAD_INFORMATION_CLASS_THREAD_QUERY_SET_WIN32_START_ADDRESS {
        if thread_info_len < 8 {
            emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
            return;
        }
        if !emu.maps.is_mapped(thread_info) {
            emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
            return;
        }
        let start = emu.regs().rip;
        let _ = emu.maps.write_qword(thread_info, start);
        if return_length_ptr != 0 {
            let _ = emu.maps.write_qword(return_length_ptr, 8);
        }
        emu.regs_mut().rax = STATUS_SUCCESS;
        return;
    }

    emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
}

/// `NtSetInformationProcess` — x64: four arguments in RCX..R9.
pub fn nt_set_information_process(emu: &mut Emu) {
    let process_handle = emu.regs().rcx;
    let class = emu.regs().rdx;

    log_orange!(
        emu,
        "syscall 0x{:x}: NtSetInformationProcess h: 0x{:x} class: 0x{:x}",
        WIN64_NTSETINFORMATIONPROCESS,
        process_handle,
        class
    );

    if !is_current_process_handle(process_handle) {
        emu.regs_mut().rax = STATUS_ACCESS_DENIED;
        return;
    }

    emu.regs_mut().rax = STATUS_SUCCESS;
}

/// `NtSetInformationThread` — x64: four arguments in RCX..R9.
pub fn nt_set_information_thread(emu: &mut Emu) {
    let thread_handle = emu.regs().rcx;
    let class = emu.regs().rdx;

    log_orange!(
        emu,
        "syscall 0x{:x}: NtSetInformationThread h: 0x{:x} class: 0x{:x}",
        WIN64_NTSETINFORMATIONTHREAD,
        thread_handle,
        class
    );

    emu.regs_mut().rax = STATUS_SUCCESS;
}

/// `NtOpenProcess` — x64: PHANDLE, ACCESS_MASK, OBJECT_ATTRIBUTES, CLIENT_ID in RCX..R9.
pub fn nt_open_process(emu: &mut Emu) {
    let handle_out = emu.regs().rcx;
    let _desired_access = emu.regs().rdx;
    let _obj_attr = emu.regs().r8;
    let _client_id = emu.regs().r9;

    log_orange!(
        emu,
        "syscall 0x{:x}: NtOpenProcess out: 0x{:x}",
        WIN64_NTOPENPROCESS,
        handle_out
    );

    if handle_out == 0 {
        emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
        return;
    }

    let fake = 0x4u64;
    if emu.maps.write_qword(handle_out, fake) {
        emu.regs_mut().rax = STATUS_SUCCESS;
    } else {
        emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
    }
}

/// `NtRaiseHardError` — CSR / hard-error dialog path (no real session in emulation).
/// x64: RCX=`ErrorStatus`, RDX=`NumberOfParameters`, R8=`UnicodeStringParameterMask`,
/// R9=`Parameters`, `[rsp+0x28]`=`ValidResponseOptions`, `[rsp+0x30]`=`Response` (PULONG).
///
/// Writes **ResponseOk (6)** to `*Response` when the pointer is mapped; returns success so ntdll
/// can continue instead of wedging on an unimplemented syscall.
pub fn nt_raise_hard_error(emu: &mut Emu) {
    let error_status = emu.regs().rcx as u32;
    let num_params = emu.regs().rdx as u32;
    let unicode_mask = emu.regs().r8 as u32;
    let parameters = emu.regs().r9;
    let rsp = emu.regs().rsp;
    let valid_options = emu.maps.read_dword(rsp + 0x28).unwrap_or(0);
    let response_ptr = emu.maps.read_qword(rsp + 0x30).unwrap_or(0);

    let status_tag = if error_status == STATUS_APP_INIT_FAILURE as u32 {
        " STATUS_APP_INIT_FAILURE"
    } else {
        ""
    };
    log_orange!(
        emu,
        "syscall 0x{:x}: NtRaiseHardError status: 0x{:x}{}, nparams: {}, umask: 0x{:x}, params: 0x{:x}, options: 0x{:x}, response: 0x{:x} (stub OK)",
        WIN64_NTRAISEHARDERROR,
        error_status,
        status_tag,
        num_params,
        unicode_mask,
        parameters,
        valid_options,
        response_ptr
    );

    // For diagnostic purposes, dump each Parameter. If `UnicodeStringParameterMask`
    // has bit N set, parameter N is a `PUNICODE_STRING`; otherwise it is a ULONG_PTR.
    if num_params > 0 && parameters != 0 && emu.maps.is_mapped(parameters) {
        for i in 0..num_params.min(4) {
            let param_addr = parameters + (i as u64) * 8;
            let pv = emu.maps.read_qword(param_addr).unwrap_or(0);
            if (unicode_mask >> i) & 1 == 1 {
                // PUNICODE_STRING — read Buffer + Length and dump the wide string.
                if pv != 0 && emu.maps.is_mapped(pv) {
                    let len = emu.maps.read_word(pv).unwrap_or(0) as u64;
                    let bufp = emu.maps.read_qword(pv + 8).unwrap_or(0);
                    let mut s = String::new();
                    let mut j = 0u64;
                    while j < len.min(256) && bufp != 0 {
                        let w = emu.maps.read_word(bufp + j).unwrap_or(0);
                        if w == 0 {
                            break;
                        }
                        s.push(char::from_u32(w as u32).unwrap_or('?'));
                        j += 2;
                    }
                    log_orange!(
                        emu,
                        "    NtRaiseHardError param[{}] = UNICODE_STRING@0x{:x} (\"{}\")",
                        i,
                        pv,
                        s,
                    );
                } else {
                    log_orange!(
                        emu,
                        "    NtRaiseHardError param[{}] = UNICODE_STRING@0x{:x} (unmapped)",
                        i,
                        pv,
                    );
                }
            } else {
                log_orange!(emu, "    NtRaiseHardError param[{}] = 0x{:x}", i, pv,);
            }
        }
    }

    const HARDERROR_RESPONSE_OK: u32 = 6;
    if response_ptr != 0 && emu.maps.is_mapped(response_ptr) {
        let _ = emu.maps.write_dword(response_ptr, HARDERROR_RESPONSE_OK);
    }
    emu.regs_mut().rax = STATUS_SUCCESS;
}

/// `NtTerminateProcess` — x64: ProcessHandle, ExitStatus.
pub fn nt_terminate_process(emu: &mut Emu) {
    let process_handle = emu.regs().rcx;
    let exit_status = emu.regs().rdx;

    log_orange!(
        emu,
        "syscall 0x{:x}: NtTerminateProcess h: 0x{:x} status: 0x{:x}",
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
    emu.process_terminated = true;
    emu.is_running.store(0, Ordering::Relaxed);
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
        "syscall 0x{:x}: NtRaiseException record: 0x{:x}, ctx: 0x{:x}, first: {}, code: 0x{:x}",
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
        "syscall 0x{:x}: NtQuerySecurityAttributesToken h: 0x{:x} num_attrs: {} buf: 0x{:x} len: {}",
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

/// `NtCreateThreadEx` — syscall 0xc9.
/// x64: RCX=`ThreadHandle` (PHANDLE out), RDX=`DesiredAccess`, R8=`ObjectAttributes`,
///      R9=`ProcessHandle`, `[rsp+0x28]`=`StartRoutine`, `[rsp+0x30]`=`Argument`,
///      `[rsp+0x38]`=`CreateFlags`, `[rsp+0x40]`=`ZeroBits`, `[rsp+0x48]`=`StackSize`,
///      `[rsp+0x50]`=`MaxStackSize`, `[rsp+0x58]`=`AttributeList`.
///
/// Stub: writes a fake thread handle to *ThreadHandle and returns STATUS_SUCCESS.
/// The emulator runs single-threaded so no real thread is created.
pub fn nt_create_thread_ex(emu: &mut Emu) {
    let thread_handle_ptr = emu.regs().rcx;
    let _desired_access = emu.regs().rdx;
    let _process_handle = emu.regs().r9;

    log_orange!(
        emu,
        "syscall 0x{:x}: NtCreateThreadEx handle_out: 0x{:x} process: 0x{:x}",
        WIN64_NTCREATETHREADEX,
        thread_handle_ptr,
        _process_handle,
    );

    if thread_handle_ptr == 0 || !emu.maps.is_mapped(thread_handle_ptr) {
        emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
        return;
    }

    // Write a synthetic handle value (same scheme used by NtCreateEvent etc.).
    let h = crate::syscall::windows::syscall64::sync::next_handle();
    let _ = emu.maps.write_qword(thread_handle_ptr, h);

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

/// `NtDuplicateObject` — syscall 0x3c.
/// RCX=SourceProcessHandle, RDX=SourceHandle, R8=TargetProcessHandle,
/// R9=TargetHandle (PHANDLE, optional out), [rsp+0x28]=DesiredAccess,
/// [rsp+0x30]=HandleAttributes, [rsp+0x38]=Options.
///
/// If TargetHandle is non-null, write a fresh fake handle to it.
pub fn nt_duplicate_object(emu: &mut Emu) {
    let source_process = emu.regs().rcx;
    let source_handle = emu.regs().rdx;
    let _target_process = emu.regs().r8;
    let target_handle_out = emu.regs().r9;
    let rsp = emu.regs().rsp;
    let desired_access = emu.maps.read_qword(rsp + 0x28).unwrap_or(0);
    let options = emu.maps.read_qword(rsp + 0x38).unwrap_or(0);

    log_orange!(
        emu,
        "syscall 0x{:x}: NtDuplicateObject src_proc: 0x{:x}, src_handle: 0x{:x}, target_out: 0x{:x}, access: 0x{:x}, opts: 0x{:x}",
        WIN64_NTDUPLICATEOBJECT,
        source_process,
        source_handle,
        target_handle_out,
        desired_access,
        options,
    );

    if target_handle_out != 0 && emu.maps.is_mapped(target_handle_out) {
        let h = super::sync::next_handle();
        let _ = emu.maps.write_qword(target_handle_out, h);
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
        "syscall 0x{:x}: NtCreateProfileEx out: 0x{:x}",
        WIN64_NTCREATEPROFILEEX,
        profile_handle,
    );

    emu.regs_mut().rax = STATUS_NOT_SUPPORTED;
}
