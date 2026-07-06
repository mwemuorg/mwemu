use crate::emu::Emu;
use crate::windows::constants::*;

/// `SYSTEM_TIMEOFDAY_INFORMATION` size (56 bytes).
const SYSTEM_TIMEOFDAY_INFORMATION_SIZE: u32 = 56;
/// `SYSTEM_PROCESS_INFORMATION` header size (224 bytes).
const SYSTEM_PROCESS_INFORMATION_SIZE: u32 = 224;

fn write_return_length(emu: &mut Emu, ret_len_ptr: u64, n: u32) {
    if ret_len_ptr == 0 {
        return;
    }
    let _ = emu.maps.write_dword(ret_len_ptr, n);
}

/// Fill a 0x40-byte x64 `SYSTEM_BASIC_INFORMATION`. Field offsets follow the
/// native 64-bit layout: the `ULONG_PTR` members are 8-byte aligned, so
/// `MinimumUserModeAddress` is at +0x20 and `MaximumUserModeAddress` at +0x28
/// (not +0x1c/+0x24 as in the 32-bit struct). ntdll reads `MaximumUserModeAddress`
/// to size loader bitmaps, so it must be the real top-of-user-VA value.
fn fill_system_basic_information(emu: &mut Emu, info: u64, len: u32) {
    for off in 0..len.min(0x40) {
        let _ = emu.maps.write_byte(info + u64::from(off), 0);
    }
    let _ = emu.maps.write_dword(info + 0x08, 0x1000); // PageSize
    let _ = emu.maps.write_dword(info + 0x0c, 0x0010_0000); // NumberOfPhysicalPages (~4GB)
    let _ = emu.maps.write_dword(info + 0x18, 0x0001_0000); // AllocationGranularity (64KB)
    let _ = emu.maps.write_qword(info + 0x20, 0x0000_0000_0001_0000); // MinimumUserModeAddress
    let _ = emu.maps.write_qword(info + 0x28, 0x0000_7fff_fffe_ffff); // MaximumUserModeAddress
    let _ = emu.maps.write_qword(info + 0x30, 1); // ActiveProcessorsAffinityMask
    let _ = emu.maps.write_byte(info + 0x38, 1); // NumberOfProcessors
}

/// `SYSTEM_INFORMATION_CLASS` values commonly hit during ntdll bootstrap (subset).
const SYSTEM_BASIC_INFORMATION: u64 = 0;
const SYSTEM_PROCESSOR_INFORMATION: u64 = 1;
const SYSTEM_PERFORMANCE_INFORMATION: u64 = 2;
const SYSTEM_TIMEOFDAY_INFORMATION: u64 = 3;
const SYSTEM_PROCESS_INFORMATION: u64 = 5;

/// `SystemEmulationBasicInformation` (class 0x3e = 62). Returns a
/// `SYSTEM_BASIC_INFORMATION` for the emulated environment (identical layout to
/// class 0). Win11's `LdrInitializeThunk` queries this very early and reads
/// `MaximumUserModeAddress` (offset 0x28) to size a loader bitmap (2 bits per MB
/// of the user address space). Returning the wrong/short structure makes that
/// size 0 and the following `NtAllocateVirtualMemoryEx` fails with
/// STATUS_INVALID_PARAMETER, aborting process init.
const SYSTEM_EMULATION_BASIC_INFORMATION: u64 = 0x3e;
/// `SYSTEM_KERNEL_DEBUGGER_INFORMATION` (class 0x23 = 35).
const SYSTEM_KERNEL_DEBUGGER_INFORMATION: u64 = 0x23;
/// `SystemKernelDebuggerInformationEx` (class 0x95 = 149).
const SYSTEM_KERNEL_DEBUGGER_INFORMATION_EX: u64 = 0x95;
/// `SYSTEM_CODE_INTEGRITY_INFORMATION` (Windows 8+).
const SYSTEM_CODE_INTEGRITY_INFORMATION: u64 = 103;
/// `SystemCodeIntegrityPolicyInformation` (0xC0) — ntdll `LdrInitializeThunk` queries this on
/// modern builds; real kernel returns `STATUS_SUCCESS` with a zeroed policy buffer (see
/// `trace_LdrInitializeThunk.txt` around `NtQuerySystemInformation`, `rcx == 0xC0`).
const SYSTEM_CODE_INTEGRITY_POLICY_INFORMATION: u64 = 0xC0;
/// `SystemExtendedHandleInformation` (0x37) — returns the handle table; we return 0 handles.
const SYSTEM_EXTENDED_HANDLE_INFORMATION: u64 = 0x37;
/// `SystemSupportedProcessorArchitectures2` (0xC5) — returns supported CPU architectures.
const SYSTEM_SUPPORTED_PROCESSOR_ARCHITECTURES2: u64 = 0xC5;

/// `NtQuerySystemInformation` — x64: RCX `Class`, RDX `Buffer`, R8 `Length`, R9 `ReturnLength`.
pub fn nt_query_system_information(emu: &mut Emu) {
    let class = emu.regs().rcx;
    let info = emu.regs().rdx;
    let len = emu.regs().r8 as u32;
    let ret_len_ptr = emu.regs().r9;

    log_orange!(
        emu,
        "syscall 0x{:x}: NtQuerySystemInformation class: 0x{:x}, buf: 0x{:x}, len: 0x{:x}, ret_len: 0x{:x}",
        WIN64_NTQUERYSYSTEMINFORMATION,
        class,
        info,
        len,
        ret_len_ptr
    );

    if info == 0 && len > 0 {
        emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
        return;
    }

    if len > 0 && info != 0
        && (!emu.maps.is_mapped(info) || !emu.maps.is_mapped(info + u64::from(len).saturating_sub(1)))
        {
            emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
            return;
        }

    match class {
        // Both return the same x64 `SYSTEM_BASIC_INFORMATION`; the emulation
        // variant just describes the (here identical) emulated environment.
        SYSTEM_BASIC_INFORMATION | SYSTEM_EMULATION_BASIC_INFORMATION => {
            const NEED: u32 = 0x40;
            if len < NEED {
                write_return_length(emu, ret_len_ptr, NEED);
                emu.regs_mut().rax = STATUS_INFO_LENGTH_MISMATCH;
                return;
            }
            fill_system_basic_information(emu, info, len);
            write_return_length(emu, ret_len_ptr, NEED);
            emu.regs_mut().rax = STATUS_SUCCESS;
        }

        SYSTEM_PROCESSOR_INFORMATION => {
            const NEED: u32 = 0x18;
            if len < NEED {
                write_return_length(emu, ret_len_ptr, NEED);
                emu.regs_mut().rax = STATUS_INFO_LENGTH_MISMATCH;
                return;
            }
            for off in 0..len {
                let _ = emu.maps.write_byte(info + u64::from(off), 0);
            }
            write_return_length(emu, ret_len_ptr, NEED);
            emu.regs_mut().rax = STATUS_SUCCESS;
        }

        SYSTEM_PERFORMANCE_INFORMATION => {
            if len < 8 {
                write_return_length(emu, ret_len_ptr, 0x100);
                emu.regs_mut().rax = STATUS_INFO_LENGTH_MISMATCH;
                return;
            }
            for off in 0..len {
                let _ = emu.maps.write_byte(info + u64::from(off), 0);
            }
            write_return_length(emu, ret_len_ptr, len);
            emu.regs_mut().rax = STATUS_SUCCESS;
        }

        SYSTEM_TIMEOFDAY_INFORMATION => {
            let need = SYSTEM_TIMEOFDAY_INFORMATION_SIZE;
            if len < need {
                write_return_length(emu, ret_len_ptr, need);
                emu.regs_mut().rax = STATUS_INFO_LENGTH_MISMATCH;
                return;
            }
            for off in 0..len {
                let _ = emu.maps.write_byte(info + u64::from(off), 0);
            }
            let _ = emu.maps.write_qword(info + 8, 1); // CurrentTime (stub)
            write_return_length(emu, ret_len_ptr, need);
            emu.regs_mut().rax = STATUS_SUCCESS;
        }

        SYSTEM_PROCESS_INFORMATION => {
            let need = SYSTEM_PROCESS_INFORMATION_SIZE;
            if len < need {
                write_return_length(emu, ret_len_ptr, need);
                emu.regs_mut().rax = STATUS_INFO_LENGTH_MISMATCH;
                return;
            }
            for off in 0..len {
                let _ = emu.maps.write_byte(info + u64::from(off), 0);
            }
            let _ = emu.maps.write_dword(info, 0);
            let _ = emu.maps.write_dword(info + 4, 0);
            write_return_length(emu, ret_len_ptr, need);
            emu.regs_mut().rax = STATUS_SUCCESS;
        }

        SYSTEM_KERNEL_DEBUGGER_INFORMATION => {
            // SYSTEM_KERNEL_DEBUGGER_INFORMATION: { DebuggerEnabled: BOOLEAN, DebuggerNotPresent: BOOLEAN }
            const NEED: u32 = 2;
            if len < NEED {
                write_return_length(emu, ret_len_ptr, NEED);
                emu.regs_mut().rax = STATUS_INFO_LENGTH_MISMATCH;
                return;
            }
            let _ = emu.maps.write_byte(info, 0); // DebuggerEnabled = FALSE
            let _ = emu.maps.write_byte(info + 1, 1); // DebuggerNotPresent = TRUE
            write_return_length(emu, ret_len_ptr, NEED);
            emu.regs_mut().rax = STATUS_SUCCESS;
        }

        SYSTEM_CODE_INTEGRITY_INFORMATION => {
            const NEED: u32 = 8;
            if len < NEED {
                write_return_length(emu, ret_len_ptr, NEED);
                emu.regs_mut().rax = STATUS_INFO_LENGTH_MISMATCH;
                return;
            }
            let _ = emu.maps.write_dword(info, NEED);
            let _ = emu.maps.write_dword(info + 4, 0);
            write_return_length(emu, ret_len_ptr, NEED);
            emu.regs_mut().rax = STATUS_SUCCESS;
        }

        SYSTEM_CODE_INTEGRITY_POLICY_INFORMATION => {
            // `SYSTEM_CODEINTEGRITYPOLICY_INFORMATION` is variable-length; ntdll passes ~0x20-byte
            // buffers during loader init. Zero-fill and report success like the reference trace.
            const MIN: u32 = 0x10;
            if len < MIN {
                write_return_length(emu, ret_len_ptr, MIN);
                emu.regs_mut().rax = STATUS_INFO_LENGTH_MISMATCH;
                return;
            }
            for off in 0..len {
                let _ = emu.maps.write_byte(info + u64::from(off), 0);
            }
            write_return_length(emu, ret_len_ptr, len);
            emu.regs_mut().rax = STATUS_SUCCESS;
        }

        SYSTEM_EXTENDED_HANDLE_INFORMATION => {
            // Returns SYSTEM_HANDLE_INFORMATION_EX: { NumberOfHandles: ULONG_PTR, Reserved: ULONG_PTR, ... }
            // Return 0 handles — minimal valid response.
            const HEADER: u32 = 16; // NumberOfHandles (8) + Reserved (8)
            if len < HEADER {
                write_return_length(emu, ret_len_ptr, HEADER);
                emu.regs_mut().rax = STATUS_INFO_LENGTH_MISMATCH;
                return;
            }
            for off in 0..HEADER {
                let _ = emu.maps.write_byte(info + u64::from(off), 0);
            }
            write_return_length(emu, ret_len_ptr, HEADER);
            emu.regs_mut().rax = STATUS_SUCCESS;
        }

        SYSTEM_SUPPORTED_PROCESSOR_ARCHITECTURES2 => {
            // Returns supported CPU architecture list; zero-fill for x64-only stub.
            const NEED: u32 = 8;
            if len < NEED {
                write_return_length(emu, ret_len_ptr, NEED);
                emu.regs_mut().rax = STATUS_INFO_LENGTH_MISMATCH;
                return;
            }
            for off in 0..NEED {
                let _ = emu.maps.write_byte(info + u64::from(off), 0);
            }
            write_return_length(emu, ret_len_ptr, NEED);
            emu.regs_mut().rax = STATUS_SUCCESS;
        }

        // SystemKernelDebuggerInformationEx (0x95) and the historically-probed
        // 0x73: SYSTEM_KERNEL_DEBUGGER_INFORMATION_EX { DebuggerAllowed,
        // DebuggerEnabled, DebuggerPresent }. Return all-zero (no kernel
        // debugger) so the js/jl on negative return is not taken.
        SYSTEM_KERNEL_DEBUGGER_INFORMATION_EX | 0x73 => {
            const NEED: u32 = 3;
            if len < NEED {
                write_return_length(emu, ret_len_ptr, NEED);
                emu.regs_mut().rax = STATUS_INFO_LENGTH_MISMATCH;
                return;
            }
            for off in 0..NEED {
                let _ = emu.maps.write_byte(info + u64::from(off), 0);
            }
            write_return_length(emu, ret_len_ptr, NEED);
            emu.regs_mut().rax = STATUS_SUCCESS;
        }

        _ => {
            log_orange!(
                emu,
                "NtQuerySystemInformation: unhandled class 0x{:x}, returning STATUS_INVALID_INFO_CLASS",
                class
            );
            write_return_length(emu, ret_len_ptr, 0);
            emu.regs_mut().rax = STATUS_INVALID_INFO_CLASS;
        }
    }
}

/// `NtManageHotPatch` — RCX `HotPatchInfo` (pointer to struct).
/// Stub: hot-patching is not supported in the emulator.
pub fn nt_manage_hot_patch(emu: &mut Emu) {
    let info = emu.regs().rcx;

    log_orange!(
        emu,
        "syscall 0x{:x}: NtManageHotPatch info: 0x{:x}",
        WIN64_NTMANAGEHOTPATCH,
        info
    );

    emu.regs_mut().rax = STATUS_NOT_SUPPORTED;
}

/// `NtQueryDebugFilterState(ComponentId, Level)` — returns FALSE (0) to indicate
/// that debug output is suppressed for this component/level (no debugger attached).
pub fn nt_query_debug_filter_state(emu: &mut Emu) {
    let component = emu.regs().rcx;
    let level = emu.regs().rdx;
    log_orange!(
        emu,
        "syscall 0x{:x}: NtQueryDebugFilterState component: 0x{:x}, level: 0x{:x}",
        WIN64_NTQUERYDEBUGFILTERSTATE,
        component,
        level
    );
    emu.regs_mut().rax = 0; // FALSE — debug output suppressed
}

/// `NtTraceEvent` — stub; ETW tracing is not emulated.
pub fn nt_trace_event(emu: &mut Emu) {
    log_orange!(
        emu,
        "syscall 0x{:x}: NtTraceEvent (stub)",
        WIN64_NTTRACEEVENT
    );
    emu.regs_mut().rax = STATUS_SUCCESS;
}

/// `NtQueryInformationTransactionManager` — syscall 0x15a.
/// x64: RCX=`TransactionManagerHandle`, RDX=`InformationClass`,
///      R8=`Buffer`, R9=`BufferLength`, `[rsp+0x28]`=`ReturnLength` (PULONG).
///
/// Kernel Transaction Manager (KTM) query. Called by ntdll during loader init
/// to probe transaction support. We handle the two most common classes and
/// return STATUS_INVALID_INFO_CLASS for everything else.
///
/// Information classes:
///   0 = TransactionManagerBasicInformation  — GUID(16) + VirtualClock(8) = 24 bytes
///   1 = TransactionManagerLogInformation    — GUID(16) = 16 bytes
pub fn nt_query_information_transaction_manager(emu: &mut Emu) {
    let _handle = emu.regs().rcx;
    let info_class = emu.regs().rdx;
    let buffer = emu.regs().r8;
    let buffer_len = emu.regs().r9;
    let rsp = emu.regs().rsp;
    let return_length_ptr = emu.maps.read_qword(rsp + 0x28).unwrap_or(0);

    log_orange!(
        emu,
        "syscall 0x{:x}: NtQueryInformationTransactionManager class: {} buf: 0x{:x} len: {}",
        WIN64_NTQUERYINFORMATIONTRANSACTIONMANAGER,
        info_class,
        buffer,
        buffer_len
    );

    let (needed, _desc): (u64, &str) = match info_class {
        0 => (24, "BasicInformation"), // GUID(16) + LARGE_INTEGER(8)
        1 => (16, "LogInformation"),   // GUID(16)
        _ => {
            emu.regs_mut().rax = STATUS_INVALID_INFO_CLASS;
            return;
        }
    };

    write_return_length(emu, return_length_ptr, needed as u32);

    if buffer == 0 || buffer_len < needed {
        emu.regs_mut().rax = STATUS_BUFFER_TOO_SMALL;
        return;
    }

    if !emu.maps.is_mapped(buffer) {
        emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
        return;
    }

    // Zero-fill the output — no real KTM state to return.
    emu.maps.memset(buffer, 0, needed as usize);

    emu.regs_mut().rax = STATUS_SUCCESS;
}

/// `NtQueryIoCompletion` — syscall 0x15e.
/// RCX=IoCompletionHandle, RDX=IoCompletionInformationClass,
/// R8=IoCompletionInformation (out), R9=IoCompletionInformationLength,
/// [rsp+0x28]=ReturnLength (out PULONG).
///
/// IoCompletionBasicInformation (class 0) returns a single ULONG Depth.
/// Since we do not track real I/O completion ports, return STATUS_INVALID_HANDLE
/// so callers fall back gracefully rather than receiving STATUS_NOT_IMPLEMENTED.
pub fn nt_query_io_completion(emu: &mut Emu) {
    let handle = emu.regs().rcx;
    let info_class = emu.regs().rdx;
    let buffer = emu.regs().r8;
    let buffer_len = emu.regs().r9;
    let rsp = emu.regs().rsp;
    let return_length_ptr = emu.maps.read_qword(rsp + 0x28).unwrap_or(0);

    log_orange!(
        emu,
        "syscall 0x{:x}: NtQueryIoCompletion handle: 0x{:x}, class: {}, buf: 0x{:x}, len: {}",
        WIN64_NTQUERYIOCOMPLETION,
        handle,
        info_class,
        buffer,
        buffer_len,
    );

    // IoCompletionBasicInformation (0): single ULONG Depth.
    // Accept any class and return zeroed output — callers interpret 0 as "no queued items".
    const NEEDED: u64 = 4; // sizeof(ULONG)
    write_return_length(emu, return_length_ptr, NEEDED as u32);

    if buffer == 0 || buffer_len < NEEDED {
        emu.regs_mut().rax = STATUS_BUFFER_TOO_SMALL;
        return;
    }

    if !emu.maps.is_mapped(buffer) {
        emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
        return;
    }

    // Depth = 0: completion port exists but has no queued items.
    let _ = emu.maps.write_dword(buffer, 0);
    emu.regs_mut().rax = STATUS_SUCCESS;
}
