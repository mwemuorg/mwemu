use std::sync::atomic::{AtomicU64, Ordering};

use crate::emu::Emu;
use crate::windows::constants::*;

static NEXT_FAKE_SYNC_HANDLE: AtomicU64 = AtomicU64::new(0x1000);

pub fn next_handle() -> u64 {
    NEXT_FAKE_SYNC_HANDLE.fetch_add(4, Ordering::Relaxed)
}

/// `NtCreateEvent` — x64: RCX `EventHandle`, RDX `DesiredAccess`, R8 `ObjectAttributes`, R9 `EventType`,
/// 5th arg `InitialState` at `[rsp+0x28]` (BOOLEAN).
pub fn nt_create_event(emu: &mut Emu) {
    let handle_out = emu.regs().rcx;
    let desired_access = emu.regs().rdx;
    let object_attributes = emu.regs().r8;
    let event_type = emu.regs().r9;
    let rsp = emu.regs().rsp;
    let initial_state = emu.maps.read_byte(rsp + 0x28).unwrap_or(0);

    log_orange!(
        emu,
        "syscall 0x{:x}: NtCreateEvent out: 0x{:x}, access: 0x{:x}, obj_attr: 0x{:x}, type: 0x{:x}, initial: {}",
        WIN64_NTCREATEEVENT,
        handle_out,
        desired_access,
        object_attributes,
        event_type,
        initial_state
    );

    if handle_out == 0 {
        emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
        return;
    }

    if !emu.maps.is_mapped(handle_out) {
        emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
        return;
    }

    let h = next_handle();
    if !emu.maps.write_qword(handle_out, h) {
        emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
        return;
    }

    emu.regs_mut().rax = STATUS_SUCCESS;
}

/// `NtSetEvent` — RCX `EventHandle`, RDX `PreviousState` (optional, can be NULL).
pub fn nt_set_event(emu: &mut Emu) {
    let handle = emu.regs().rcx;
    let previous_state = emu.regs().rdx;

    log_orange!(
        emu,
        "syscall 0x{:x}: NtSetEvent handle: 0x{:x}, prev_state: 0x{:x}",
        WIN64_NTSETEVENT,
        handle,
        previous_state
    );

    if previous_state != 0 && emu.maps.is_mapped(previous_state) {
        emu.maps.write_dword(previous_state, 0);
    }

    emu.regs_mut().rax = STATUS_SUCCESS;
}

/// `NtWaitForAlertByThreadId` — x64: RCX `UniqueThreadId` (HANDLE-sized id), RDX `Timeout` (`PLARGE_INTEGER`).
///
/// Waits until another thread alerts this wait; we have no second thread or alert delivery, so an
/// unimplemented syscall left **RAX** equal to the syscall number (`0x1e3`), which is **not**
/// `STATUS_SUCCESS`, and ntdll can spin forever. Return immediately as if the wait completed.
pub fn nt_wait_for_alert_by_thread_id(emu: &mut Emu) {
    let unique_thread_id = emu.regs().rcx;
    let timeout = emu.regs().rdx;

    log_orange!(
        emu,
        "syscall 0x{:x}: NtWaitForAlertByThreadId thread_id: 0x{:x}, timeout_ptr: 0x{:x}",
        WIN64_NTWAITFORALERTBYTHREADID,
        unique_thread_id,
        timeout
    );

    emu.regs_mut().rax = STATUS_SUCCESS;
}

/// `NtAlpcAcceptConnectPort` — x64 syscall 0x79.
///
/// RCX = PortHandle (out), RDX = ConnectionPortHandle, R8 = Flags,
/// R9 = ObjectAttributes, [rsp+28] = PortAttributes, [rsp+30] = PortContext,
/// [rsp+38] = ConnectionRequest, [rsp+40] = ConnMsgAttributes, [rsp+48] = AcceptConnection.
/// Stub: write a fake handle and return STATUS_SUCCESS (accept).
pub fn nt_alpc_accept_connect_port(emu: &mut Emu) {
    let handle_out = emu.regs().rcx;
    let conn_port = emu.regs().rdx;

    log_orange!(
        emu,
        "syscall 0x{:x}: NtAlpcAcceptConnectPort out: 0x{:x}, conn: 0x{:x}",
        WIN64_NTALPCACCEPTCONNECTPORT,
        handle_out,
        conn_port,
    );

    if handle_out != 0 && emu.maps.is_mapped(handle_out) {
        let h = next_handle();
        let _ = emu.maps.write_qword(handle_out, h);
    }
    emu.regs_mut().rax = STATUS_SUCCESS;
}

/// `NtWaitForSingleObject` — x64 syscall 0x4.
///
/// RCX = Handle, RDX = Alertable (BOOLEAN), R8 = Timeout (PLARGE_INTEGER, optional).
/// Stub: returns STATUS_SUCCESS (object signaled) immediately.
pub fn nt_wait_for_single_object(emu: &mut Emu) {
    let handle = emu.regs().rcx;
    let alertable = emu.regs().rdx;
    let timeout = emu.regs().r8;

    log_orange!(
        emu,
        "syscall 0x{:x}: NtWaitForSingleObject handle: 0x{:x}, alertable: {}, timeout: 0x{:x}",
        WIN64_NTWAITFORSINGLEOBJECT,
        handle,
        alertable,
        timeout,
    );

    emu.regs_mut().rax = STATUS_SUCCESS; // WAIT_OBJECT_0
}

/// `NtOpenEvent` — x64 syscall 0x40.
///
/// RCX = EventHandle (out), RDX = DesiredAccess, R8 = ObjectAttributes.
/// Returns a fake handle; callers use it as an opaque synchronisation token.
pub fn nt_open_event(emu: &mut Emu) {
    let handle_out = emu.regs().rcx;
    let desired_access = emu.regs().rdx;
    let object_attr = emu.regs().r8;

    log_orange!(
        emu,
        "syscall 0x{:x}: NtOpenEvent out: 0x{:x}, access: 0x{:x}, obj: 0x{:x}",
        WIN64_NTOPENEVENT,
        handle_out,
        desired_access,
        object_attr,
    );

    if handle_out == 0 || !emu.maps.is_mapped(handle_out) {
        emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
        return;
    }

    let h = next_handle();
    let _ = emu.maps.write_qword(handle_out, h);
    emu.regs_mut().rax = STATUS_SUCCESS;
}

/// `NtCreateTimer2` — x64 syscall 0xcc.
///
/// RCX = TimerHandle (out), RDX = DesiredAccess, R8 = ObjectAttributes,
/// R9 = TimerType (0=Synchronization, 1=Notification), [rsp+0x28] = TimerAttributes.
/// Returns a fake handle.
pub fn nt_create_timer2(emu: &mut Emu) {
    let handle_out = emu.regs().rcx;
    let desired_access = emu.regs().rdx;
    let timer_type = emu.regs().r9;

    log_orange!(
        emu,
        "syscall 0x{:x}: NtCreateTimer2 out: 0x{:x}, access: 0x{:x}, type: {}",
        WIN64_NTCREATETIMER2,
        handle_out,
        desired_access,
        timer_type,
    );

    if handle_out == 0 || !emu.maps.is_mapped(handle_out) {
        emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
        return;
    }

    let h = next_handle();
    let _ = emu.maps.write_qword(handle_out, h);
    emu.regs_mut().rax = STATUS_SUCCESS;
}

/// `NtClose` — RCX `Handle`.
pub fn nt_close(emu: &mut Emu) {
    let handle = emu.regs().rcx;

    log_orange!(
        emu,
        "syscall 0x{:x}: NtClose handle: 0x{:x}",
        WIN64_NTCLOSE,
        handle
    );

    emu.regs_mut().rax = STATUS_SUCCESS;
}
