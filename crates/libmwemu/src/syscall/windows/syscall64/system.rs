use crate::emu::Emu;
use crate::windows::constants::*;

/// Synthetic kernel image base/size/full path used by `SystemModuleInformation`
/// and `SystemModuleInformationEx`. The values are stable across runs.
const FAKE_KERNEL_BASE: u64 = 0xFFFF_F800_0000_0000;
const FAKE_KERNEL_SIZE: u32 = 0x00A0_0000;
const FAKE_KERNEL_DIR: &[u8] = b"\\SystemRoot\\system32\\";
const FAKE_KERNEL_FULL_PATH: &[u8] = b"\\SystemRoot\\system32\\ntoskrnl.exe";

/// Synthetic `SYSTEM_PROCESS_INFORMATION` PID. Mirrors Sogen's behaviour where
/// the emulated process reports PID 1 and uses `ThreadContext.id` for thread
/// IDs (since `ThreadContext` does not own per-thread TEBs).
const SYNTHETIC_PROCESS_ID: u64 = 1;

/// `SYSTEM_INFORMATION_CLASS` values commonly hit during ntdll bootstrap and
/// the practical Sogen-compatible classes. Local to this module; do not
/// promote into global Windows constants because their numeric assignments
/// reflect the implementation contract.
const SYSTEM_BASIC_INFORMATION: u64 = 0x00;
const SYSTEM_PROCESSOR_INFORMATION: u64 = 0x01;
const SYSTEM_PERFORMANCE_INFORMATION: u64 = 0x02;
const SYSTEM_TIME_OF_DAY_INFORMATION: u64 = 0x03;
const SYSTEM_PROCESS_INFORMATION: u64 = 0x05;
const SYSTEM_DEVICE_INFORMATION: u64 = 0x07;
const SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION: u64 = 0x08;
const SYSTEM_FILE_CACHE_INFORMATION: u64 = 0x15;
const SYSTEM_EXCEPTION_INFORMATION: u64 = 0x21;
const SYSTEM_FULL_MEMORY_INFORMATION: u64 = 0x19;
const SYSTEM_SUMMARY_MEMORY_INFORMATION: u64 = 0x1D;
const SYSTEM_CURRENT_TIME_ZONE_INFORMATION: u64 = 0x2C;
const SYSTEM_RANGE_START_INFORMATION: u64 = 0x32;
const SYSTEM_NUMA_PROCESSOR_MAP: u64 = 0x37;
const SYSTEM_KERNEL_DEBUGGER_INFORMATION: u64 = 0x23;
const SYSTEM_NUMA_AVAILABLE_MEMORY: u64 = 0x3C;
const SYSTEM_MODULE_INFORMATION: u64 = 0x0B;
const SYSTEM_RECOMMENDED_SHARED_DATA_ALIGNMENT: u64 = 0x3A;
const SYSTEM_EXTENDED_HANDLE_INFORMATION: u64 = 0x40;
const SYSTEM_MODULE_INFORMATION_EX: u64 = 0x4D;
const SYSTEM_MEMORY_LIST_INFORMATION: u64 = 0x50;
const SYSTEM_FILE_CACHE_INFORMATION_EX: u64 = 0x51;
const SYSTEM_LOGICAL_PROCESSOR_INFORMATION: u64 = 0x49;
const SYSTEM_TIME_ZONE_INFORMATION: u64 = 0x5C;
const SYSTEM_DYNAMIC_TIME_ZONE_INFORMATION: u64 = 0x66;
const SYSTEM_BOOT_ENVIRONMENT_INFORMATION: u64 = 0x5A;
const SYSTEM_CODE_INTEGRITY_INFORMATION: u64 = 0x67;
const SYSTEM_KERNEL_DEBUGGER_INFORMATION_EX_LEGACY: u64 = 0x73;
const SYSTEM_ERROR_PORT_TIMEOUTS: u64 = 0x73;
const SYSTEM_KERNEL_DEBUGGER_INFORMATION_EX: u64 = 0x95;
const SYSTEM_EMULATION_BASIC_INFORMATION: u64 = 0x3E;
const SYSTEM_CODE_INTEGRITY_POLICY_INFORMATION: u64 = 0xC0;
const SYSTEM_HYPERVISOR_SHARED_PAGE_INFORMATION: u64 = 0xC5;
const SYSTEM_MEMORY_USAGE_INFORMATION: u64 = 0xB5;
const SYSTEM_FLUSH_INFORMATION: u64 = 0xDD;
const SYSTEM_SUPPORTED_PROCESSOR_ARCHITECTURES2: u64 = 0xE6;
const SYSTEM_SUPPORTED_PROCESSOR_ARCHITECTURES: u64 = 0xB4;

/// Fixed payload sizes for classes we model directly. Sourced from Sogen
/// `process.hpp` (`Emu64` traits) and PHNT master ntexapi.h. The values are
/// the authoritative sizes callers must accept.
const SYSTEM_BASIC_INFO_SIZE: u32 = 0x40;
const SYSTEM_PROCESSOR_INFO_SIZE: u32 = 0x18;
const SYSTEM_PERFORMANCE_INFO_SIZE: u32 = 0x138;
const SYSTEM_TIME_OF_DAY_INFO_SIZE: u32 = 0x30;
const SYSTEM_PROCESSOR_PERFORMANCE_INFO_SIZE: u32 = 0x30;
const SYSTEM_DEVICE_INFO_SIZE: u32 = 0x18;
const SYSTEM_FILE_CACHE_INFO_SIZE: u32 = 0x40;
const SYSTEM_EXCEPTION_INFO_SIZE: u32 = 0x10;
const SYSTEM_MODULE_INFO_REQUIRED: u32 = 0x130;
const SYSTEM_MODULE_INFO_EX_REQUIRED: u32 = 0x148;
const SYSTEM_PROCESS_INFO_PREFIX_SIZE: u32 = 0x100;
const SYSTEM_THREAD_INFO_SIZE: u32 = 0x50;
const SYSTEM_CODE_INTEGRITY_INFO_SIZE: u32 = 0x08;
const SYSTEM_KERNEL_DEBUGGER_INFO_SIZE: u32 = 0x02;
const SYSTEM_KERNEL_DEBUGGER_INFO_EX_SIZE: u32 = 0x03;
const SYSTEM_EXTENDED_HANDLE_HEADER_SIZE: u32 = 0x10;
const SYSTEM_MEMORY_LIST_INFO_SIZE: u32 = 0xB0;
const SYSTEM_ERROR_PORT_TIMEOUTS_SIZE: u32 = 0x08;
const SYSTEM_RECOMMENDED_SHARED_DATA_ALIGNMENT_SIZE: u32 = 0x04;
const SYSTEM_CODE_INTEGRITY_POLICY_MIN_SIZE: u32 = 0x10;

/// Win32 `KWAIT_REASON` values used to populate `SYSTEM_THREAD_INFORMATION.WaitReason`.
const KWAIT_REASON_DELAY_EXECUTION: u32 = 4;
const KWAIT_REASON_SUSPENDED: u32 = 5;
const KWAIT_REASON_WR_EXECUTIVE: u32 = 7;

/// Win32 `KTHREAD_STATE` values used to populate `SYSTEM_THREAD_INFORMATION.ThreadState`.
const KTHREAD_STATE_READY: u32 = 1;
const KTHREAD_STATE_RUNNING: u32 = 2;
const KTHREAD_STATE_WAITING: u32 = 5;

/// `SYSTEM_CODEINTEGRITY_INFORMATION.CodeIntegrityOptions` reported to callers.
/// Matches a normal retail configuration (driver signature enforcement on).
const CODE_INTEGRITY_OPTION_ENABLED: u32 = 0x1;

/// Default thread scheduling values used for `SYSTEM_PROCESS_INFORMATION`.
const THREAD_BASE_PRIORITY: u32 = 8;

fn write_return_length(emu: &mut Emu, ret_len_ptr: u64, n: u32) {
    if ret_len_ptr == 0 {
        return;
    }
    let _ = emu.maps.write_dword(ret_len_ptr, n);
}

/// Write `n` zero bytes starting at `addr`. Bounds-checked via `is_mapped` at
/// the dispatcher entry; failures here are silently ignored because callers
/// already passed the validation gate.
fn zero_span(emu: &mut Emu, addr: u64, n: u32) {
    for off in 0..n {
        let _ = emu.maps.write_byte(addr + u64::from(off), 0);
    }
}

/// Reject short buffers and report the required size in `ReturnLength`.
/// Returns `true` when the caller's buffer is too small and the dispatcher
/// must return immediately.
fn short_buffer(emu: &mut Emu, ret_len_ptr: u64, required: u32, len: u32) -> bool {
    if len < required {
        write_return_length(emu, ret_len_ptr, required);
        emu.regs_mut().rax = STATUS_INFO_LENGTH_MISMATCH;
        return true;
    }
    false
}

/// Validate the caller-provided output pointer and length. On failure writes
/// the appropriate NTSTATUS into `rax` and returns `false`; the dispatcher
/// must then early-return.
fn validate_output_buffer(emu: &mut Emu, info: u64, len: u32) -> bool {
    if info == 0 && len > 0 {
        emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
        return false;
    }
    if len > 0
        && info != 0
        && (!emu.maps.is_mapped(info)
            || !emu.maps.is_mapped(info + u64::from(len).saturating_sub(1)))
    {
        emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
        return false;
    }
    true
}

/// Fill a 0x40-byte x64 `SYSTEM_BASIC_INFORMATION`. Field offsets follow the
/// native 64-bit layout: the `ULONG_PTR` members are 8-byte aligned, so
/// `MinimumUserModeAddress` is at +0x20 and `MaximumUserModeAddress` at +0x28
/// (not +0x1c/+0x24 as in the 32-bit struct). ntdll reads
/// `MaximumUserModeAddress` to size loader bitmaps, so it must be the real
/// top-of-user-VA value.
fn fill_system_basic_information(emu: &mut Emu, info: u64, len: u32) {
    zero_span(emu, info, len.min(SYSTEM_BASIC_INFO_SIZE));
    let _ = emu.maps.write_dword(info + 0x08, 0x1000); // PageSize
    let _ = emu.maps.write_dword(info + 0x0C, 0x0010_0000); // NumberOfPhysicalPages (~4GB)
    let _ = emu.maps.write_dword(info + 0x18, 0x0001_0000); // AllocationGranularity (64KB)
    let _ = emu.maps.write_qword(info + 0x20, 0x0000_0000_0001_0000); // MinimumUserModeAddress
    let _ = emu.maps.write_qword(info + 0x28, 0x0000_7fff_fffe_ffff); // MaximumUserModeAddress
    let _ = emu.maps.write_qword(info + 0x30, 1); // ActiveProcessorsAffinityMask
    let _ = emu.maps.write_byte(info + 0x38, 1); // NumberOfProcessors
}

/// Fill a 0x18-byte x64 `SYSTEM_PROCESSOR_INFORMATION`. Writes only the
/// declared payload bytes (not the entire caller buffer).
fn fill_system_processor_information(emu: &mut Emu, info: u64, len: u32) {
    zero_span(emu, info, len.min(SYSTEM_PROCESSOR_INFO_SIZE));
    // ProcessorArchitecture = 0x0009 = PROCESSOR_ARCHITECTURE_AMD64
    let _ = emu.maps.write_word(info, 0x0009);
    // ProcessorLevel / ProcessorRevision left zero (typical AMD64 family).
    // MaximumProcessors = 1
    let _ = emu.maps.write_word(info + 6, 1);
}

/// Compute the per-thread state for `SystemProcessInformation`. Precedence:
/// suspended > blocked_on_cs > sleeping > runnable. The current runnable
/// thread is reported as `Running`; all other runnable threads are `Ready`.
fn thread_state_reason(emu: &Emu, thread_idx: usize) -> (u32, u32) {
    let thread = &emu.threads[thread_idx];
    if thread.suspended {
        return (KTHREAD_STATE_WAITING, KWAIT_REASON_SUSPENDED);
    }
    if thread.blocked_on_cs.is_some() {
        return (KTHREAD_STATE_WAITING, KWAIT_REASON_WR_EXECUTIVE);
    }
    if thread.wake_tick > emu.tick {
        return (KTHREAD_STATE_WAITING, KWAIT_REASON_DELAY_EXECUTION);
    }
    if thread_idx == emu.current_thread_id {
        (KTHREAD_STATE_RUNNING, 0)
    } else {
        (KTHREAD_STATE_READY, 0)
    }
}

/// Write one x64 `SYSTEM_THREAD_INFORMATION` (0x50 bytes) at `addr`.
fn write_system_thread_information(
    emu: &mut Emu,
    addr: u64,
    tid: u64,
    state: u32,
    wait_reason: u32,
) {
    zero_span(emu, addr, SYSTEM_THREAD_INFO_SIZE);
    // ClientId at +0x28 (UniqueProcess at +0x28, UniqueThread at +0x30)
    let _ = emu.maps.write_qword(addr + 0x28, SYNTHETIC_PROCESS_ID);
    let _ = emu.maps.write_qword(addr + 0x30, tid);
    // Priority / BasePriority at +0x38 / +0x3C
    let _ = emu.maps.write_dword(addr + 0x38, THREAD_BASE_PRIORITY);
    let _ = emu.maps.write_dword(addr + 0x3C, THREAD_BASE_PRIORITY);
    // ContextSwitches at +0x40
    let _ = emu.maps.write_dword(addr + 0x40, 0);
    // ThreadState at +0x44
    let _ = emu.maps.write_dword(addr + 0x44, state);
    // WaitReason at +0x48
    let _ = emu.maps.write_dword(addr + 0x48, wait_reason);
}

/// Write a single x64 `RTL_PROCESS_MODULE_INFORMATION` (0x128 bytes) at
/// `addr` for the synthetic `ntoskrnl.exe`.
fn write_rtl_process_module_information(emu: &mut Emu, addr: u64) {
    const SIZE: u32 = 0x128;
    zero_span(emu, addr, SIZE);
    // MappedBase +0x08, ImageBase +0x10
    let _ = emu.maps.write_qword(addr + 0x08, FAKE_KERNEL_BASE);
    let _ = emu.maps.write_qword(addr + 0x10, FAKE_KERNEL_BASE);
    // ImageSize +0x18
    let _ = emu.maps.write_dword(addr + 0x18, FAKE_KERNEL_SIZE);
    // LoadCount +0x24
    let _ = emu.maps.write_word(addr + 0x24, 1);
    // OffsetToFileName +0x26 = byte index of the basename within FullPathName.
    let _ = emu
        .maps
        .write_word(addr + 0x26, FAKE_KERNEL_DIR.len() as u16);
    // FullPathName +0x28 (NUL-terminated ASCII).
    let _ = emu.maps.write_bytes(addr + 0x28, FAKE_KERNEL_FULL_PATH);
}

/// `NtQuerySystemInformation` — x64: RCX `Class`, RDX `Buffer`, R8 `Length`, R9 `ReturnLength`.
///
/// Returns `STATUS_NOT_SUPPORTED` for classes the dispatcher recognises but
/// does not implement; returns `STATUS_INVALID_INFO_CLASS` for unknown
/// classes. The NtQuerySystemInformationEx path is intentionally untouched.
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

    if !validate_output_buffer(emu, info, len) {
        return;
    }

    match class {
        SYSTEM_BASIC_INFORMATION | SYSTEM_EMULATION_BASIC_INFORMATION => {
            if short_buffer(emu, ret_len_ptr, SYSTEM_BASIC_INFO_SIZE, len) {
                return;
            }
            fill_system_basic_information(emu, info, len);
            write_return_length(emu, ret_len_ptr, SYSTEM_BASIC_INFO_SIZE);
            emu.regs_mut().rax = STATUS_SUCCESS;
        }

        SYSTEM_PROCESSOR_INFORMATION => {
            if short_buffer(emu, ret_len_ptr, SYSTEM_PROCESSOR_INFO_SIZE, len) {
                return;
            }
            fill_system_processor_information(emu, info, len);
            write_return_length(emu, ret_len_ptr, SYSTEM_PROCESSOR_INFO_SIZE);
            emu.regs_mut().rax = STATUS_SUCCESS;
        }

        SYSTEM_PERFORMANCE_INFORMATION => {
            // OS-version-dependent counter block: zero only the caller's
            // reported size and report success. We don't model it.
            if len < 8 {
                write_return_length(emu, ret_len_ptr, 0x100);
                emu.regs_mut().rax = STATUS_INFO_LENGTH_MISMATCH;
                return;
            }
            zero_span(emu, info, len);
            write_return_length(emu, ret_len_ptr, len);
            emu.regs_mut().rax = STATUS_SUCCESS;
        }

        SYSTEM_TIME_OF_DAY_INFORMATION => {
            if short_buffer(emu, ret_len_ptr, SYSTEM_TIME_OF_DAY_INFO_SIZE, len) {
                return;
            }
            zero_span(emu, info, SYSTEM_TIME_OF_DAY_INFO_SIZE);
            // CurrentTime at +0x08, TimeZoneId at +0x18.
            let _ = emu.maps.write_qword(info + 0x08, 1);
            let _ = emu.maps.write_dword(info + 0x18, 0x2);
            write_return_length(emu, ret_len_ptr, SYSTEM_TIME_OF_DAY_INFO_SIZE);
            emu.regs_mut().rax = STATUS_SUCCESS;
        }

        SYSTEM_PROCESS_INFORMATION => {
            let thread_count = emu.threads.len() as u32;
            let total = match SYSTEM_PROCESS_INFO_PREFIX_SIZE.checked_add(
                thread_count
                    .checked_mul(SYSTEM_THREAD_INFO_SIZE)
                    .unwrap_or(u32::MAX),
            ) {
                Some(v) => v,
                None => {
                    emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
                    return;
                }
            };
            if short_buffer(emu, ret_len_ptr, total, len) {
                return;
            }
            zero_span(emu, info, total);
            // NextEntryOffset at +0x00, NumberOfThreads at +0x04.
            let _ = emu.maps.write_dword(info, 0);
            let _ = emu.maps.write_dword(info + 0x04, thread_count);
            // BasePriority at +0x048, UniqueProcessId at +0x050, HandleCount
            // at +0x05C, SessionId at +0x060. (4 B pad sits at +0x04C between
            // BasePriority (LONG) and UniqueProcessId (HANDLE) so the qword
            // is 8-byte aligned; we already zeroed the whole prefix above.)
            let _ = emu.maps.write_dword(info + 0x048, THREAD_BASE_PRIORITY);
            let _ = emu.maps.write_qword(info + 0x050, SYNTHETIC_PROCESS_ID);
            let _ = emu.maps.write_dword(info + 0x05C, 0);
            let _ = emu.maps.write_dword(info + 0x060, 0);

            let entries: Vec<(u64, u32, u32)> = emu
                .threads
                .iter()
                .enumerate()
                .map(|(i, thread)| {
                    let (state, reason) = thread_state_reason(emu, i);
                    (thread.id, state, reason)
                })
                .collect();
            for (i, (tid, state, reason)) in entries.into_iter().enumerate() {
                let thread_addr = info
                    + u64::from(SYSTEM_PROCESS_INFO_PREFIX_SIZE)
                    + u64::from(i as u32) * u64::from(SYSTEM_THREAD_INFO_SIZE);
                write_system_thread_information(emu, thread_addr, tid, state, reason);
            }
            write_return_length(emu, ret_len_ptr, total);
            emu.regs_mut().rax = STATUS_SUCCESS;
        }

        SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION => {
            if short_buffer(
                emu,
                ret_len_ptr,
                SYSTEM_PROCESSOR_PERFORMANCE_INFO_SIZE,
                len,
            ) {
                return;
            }
            zero_span(emu, info, SYSTEM_PROCESSOR_PERFORMANCE_INFO_SIZE);
            // IdleTime +0x00, KernelTime +0x08; user/dpc/interrupt remain 0.
            let tick = emu.pos as u64;
            let _ = emu.maps.write_qword(info, tick);
            let _ = emu.maps.write_qword(info + 0x08, tick);
            write_return_length(emu, ret_len_ptr, SYSTEM_PROCESSOR_PERFORMANCE_INFO_SIZE);
            emu.regs_mut().rax = STATUS_SUCCESS;
        }

        SYSTEM_DEVICE_INFORMATION => {
            if short_buffer(emu, ret_len_ptr, SYSTEM_DEVICE_INFO_SIZE, len) {
                return;
            }
            zero_span(emu, info, SYSTEM_DEVICE_INFO_SIZE);
            // NumberOfDisks at +0x00.
            let _ = emu.maps.write_dword(info, 1);
            write_return_length(emu, ret_len_ptr, SYSTEM_DEVICE_INFO_SIZE);
            emu.regs_mut().rax = STATUS_SUCCESS;
        }

        SYSTEM_EXCEPTION_INFORMATION => {
            if short_buffer(emu, ret_len_ptr, SYSTEM_EXCEPTION_INFO_SIZE, len) {
                return;
            }
            zero_span(emu, info, SYSTEM_EXCEPTION_INFO_SIZE);
            write_return_length(emu, ret_len_ptr, SYSTEM_EXCEPTION_INFO_SIZE);
            emu.regs_mut().rax = STATUS_SUCCESS;
        }

        SYSTEM_FILE_CACHE_INFORMATION | SYSTEM_FILE_CACHE_INFORMATION_EX => {
            if short_buffer(emu, ret_len_ptr, SYSTEM_FILE_CACHE_INFO_SIZE, len) {
                return;
            }
            zero_span(emu, info, SYSTEM_FILE_CACHE_INFO_SIZE);
            write_return_length(emu, ret_len_ptr, SYSTEM_FILE_CACHE_INFO_SIZE);
            emu.regs_mut().rax = STATUS_SUCCESS;
        }

        SYSTEM_MEMORY_LIST_INFORMATION => {
            if short_buffer(emu, ret_len_ptr, SYSTEM_MEMORY_LIST_INFO_SIZE, len) {
                return;
            }
            zero_span(emu, info, SYSTEM_MEMORY_LIST_INFO_SIZE);
            write_return_length(emu, ret_len_ptr, SYSTEM_MEMORY_LIST_INFO_SIZE);
            emu.regs_mut().rax = STATUS_SUCCESS;
        }

        SYSTEM_MODULE_INFORMATION => {
            if short_buffer(emu, ret_len_ptr, SYSTEM_MODULE_INFO_REQUIRED, len) {
                return;
            }
            zero_span(emu, info, SYSTEM_MODULE_INFO_REQUIRED);
            // NumberOfModules at +0x00; the module array starts at +0x08.
            let _ = emu.maps.write_dword(info, 1);
            write_rtl_process_module_information(emu, info + 0x08);
            write_return_length(emu, ret_len_ptr, SYSTEM_MODULE_INFO_REQUIRED);
            emu.regs_mut().rax = STATUS_SUCCESS;
        }

        SYSTEM_MODULE_INFORMATION_EX => {
            if short_buffer(emu, ret_len_ptr, SYSTEM_MODULE_INFO_EX_REQUIRED, len) {
                return;
            }
            zero_span(emu, info, SYSTEM_MODULE_INFO_EX_REQUIRED);
            // NextOffset at +0x00 (terminator). BaseInfo at +0x08; DefaultBase at +0x140.
            write_rtl_process_module_information(emu, info + 0x08);
            let _ = emu.maps.write_qword(info + 0x140, FAKE_KERNEL_BASE);
            write_return_length(emu, ret_len_ptr, SYSTEM_MODULE_INFO_EX_REQUIRED);
            emu.regs_mut().rax = STATUS_SUCCESS;
        }

        SYSTEM_EXTENDED_HANDLE_INFORMATION => {
            if short_buffer(emu, ret_len_ptr, SYSTEM_EXTENDED_HANDLE_HEADER_SIZE, len) {
                return;
            }
            // 16-byte header: NumberOfHandles (8) + Reserved (8) = 0 handles.
            zero_span(emu, info, SYSTEM_EXTENDED_HANDLE_HEADER_SIZE);
            write_return_length(emu, ret_len_ptr, SYSTEM_EXTENDED_HANDLE_HEADER_SIZE);
            emu.regs_mut().rax = STATUS_SUCCESS;
        }

        SYSTEM_ERROR_PORT_TIMEOUTS => {
            if short_buffer(emu, ret_len_ptr, SYSTEM_ERROR_PORT_TIMEOUTS_SIZE, len) {
                return;
            }
            zero_span(emu, info, SYSTEM_ERROR_PORT_TIMEOUTS_SIZE);
            write_return_length(emu, ret_len_ptr, SYSTEM_ERROR_PORT_TIMEOUTS_SIZE);
            emu.regs_mut().rax = STATUS_SUCCESS;
        }

        SYSTEM_RECOMMENDED_SHARED_DATA_ALIGNMENT => {
            if short_buffer(
                emu,
                ret_len_ptr,
                SYSTEM_RECOMMENDED_SHARED_DATA_ALIGNMENT_SIZE,
                len,
            ) {
                return;
            }
            zero_span(emu, info, SYSTEM_RECOMMENDED_SHARED_DATA_ALIGNMENT_SIZE);
            let _ = emu.maps.write_dword(info, 64);
            write_return_length(
                emu,
                ret_len_ptr,
                SYSTEM_RECOMMENDED_SHARED_DATA_ALIGNMENT_SIZE,
            );
            emu.regs_mut().rax = STATUS_SUCCESS;
        }

        SYSTEM_KERNEL_DEBUGGER_INFORMATION => {
            // SYSTEM_KERNEL_DEBUGGER_INFORMATION: { DebuggerEnabled: BOOLEAN, DebuggerNotPresent: BOOLEAN }
            if short_buffer(emu, ret_len_ptr, SYSTEM_KERNEL_DEBUGGER_INFO_SIZE, len) {
                return;
            }
            let _ = emu.maps.write_byte(info, 0); // DebuggerEnabled = FALSE
            let _ = emu.maps.write_byte(info + 1, 1); // DebuggerNotPresent = TRUE
            write_return_length(emu, ret_len_ptr, SYSTEM_KERNEL_DEBUGGER_INFO_SIZE);
            emu.regs_mut().rax = STATUS_SUCCESS;
        }

        SYSTEM_CODE_INTEGRITY_INFORMATION => {
            if short_buffer(emu, ret_len_ptr, SYSTEM_CODE_INTEGRITY_INFO_SIZE, len) {
                return;
            }
            // Length at +0x00, CodeIntegrityOptions at +0x04.
            let _ = emu.maps.write_dword(info, SYSTEM_CODE_INTEGRITY_INFO_SIZE);
            let _ = emu
                .maps
                .write_dword(info + 0x04, CODE_INTEGRITY_OPTION_ENABLED);
            write_return_length(emu, ret_len_ptr, SYSTEM_CODE_INTEGRITY_INFO_SIZE);
            emu.regs_mut().rax = STATUS_SUCCESS;
        }

        SYSTEM_CODE_INTEGRITY_POLICY_INFORMATION => {
            // `SYSTEM_CODEINTEGRITYPOLICY_INFORMATION` is variable-length; ntdll passes ~0x20-byte
            // buffers during loader init. Zero-fill and report success like the reference trace.
            if len < SYSTEM_CODE_INTEGRITY_POLICY_MIN_SIZE {
                write_return_length(emu, ret_len_ptr, SYSTEM_CODE_INTEGRITY_POLICY_MIN_SIZE);
                emu.regs_mut().rax = STATUS_INFO_LENGTH_MISMATCH;
                return;
            }
            zero_span(emu, info, len);
            write_return_length(emu, ret_len_ptr, len);
            emu.regs_mut().rax = STATUS_SUCCESS;
        }

        SYSTEM_KERNEL_DEBUGGER_INFORMATION_EX => {
            // 3-byte response: { DebuggerAllowed, DebuggerEnabled, DebuggerPresent }.
            if short_buffer(emu, ret_len_ptr, SYSTEM_KERNEL_DEBUGGER_INFO_EX_SIZE, len) {
                return;
            }
            zero_span(emu, info, SYSTEM_KERNEL_DEBUGGER_INFO_EX_SIZE);
            write_return_length(emu, ret_len_ptr, SYSTEM_KERNEL_DEBUGGER_INFO_EX_SIZE);
            emu.regs_mut().rax = STATUS_SUCCESS;
        }

        SYSTEM_FULL_MEMORY_INFORMATION
        | SYSTEM_SUMMARY_MEMORY_INFORMATION
        | SYSTEM_MEMORY_USAGE_INFORMATION
        | SYSTEM_TIME_ZONE_INFORMATION
        | SYSTEM_CURRENT_TIME_ZONE_INFORMATION
        | SYSTEM_DYNAMIC_TIME_ZONE_INFORMATION
        | SYSTEM_RANGE_START_INFORMATION
        | SYSTEM_NUMA_PROCESSOR_MAP
        | SYSTEM_NUMA_AVAILABLE_MEMORY
        | SYSTEM_LOGICAL_PROCESSOR_INFORMATION
        | SYSTEM_BOOT_ENVIRONMENT_INFORMATION
        | SYSTEM_HYPERVISOR_SHARED_PAGE_INFORMATION
        | SYSTEM_FLUSH_INFORMATION
        | SYSTEM_SUPPORTED_PROCESSOR_ARCHITECTURES
        | SYSTEM_SUPPORTED_PROCESSOR_ARCHITECTURES2 => {
            log_orange!(
                emu,
                "NtQuerySystemInformation: class 0x{:x} valid but unmodeled, returning STATUS_NOT_SUPPORTED",
                class
            );
            write_return_length(emu, ret_len_ptr, 0);
            emu.regs_mut().rax = STATUS_NOT_SUPPORTED;
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
/// Since we do not track real I/O completion ports, we accept any class and
/// return zeroed output — callers interpret 0 as "no queued items".
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
