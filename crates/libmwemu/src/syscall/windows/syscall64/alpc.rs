use crate::emu::Emu;
use crate::windows::constants::*;

use super::sync;

/// Generic ALPC handle-creating stub: writes a fake handle to [RCX] and returns STATUS_SUCCESS.
fn alpc_create_handle(emu: &mut Emu, nr: u64, name: &str) {
    let handle_out = emu.regs().rcx;
    log_orange!(emu, "syscall 0x{:x}: {} out: 0x{:x}", nr, name, handle_out);
    if handle_out != 0 && emu.maps.is_mapped(handle_out) {
        let h = sync::next_handle();
        let _ = emu.maps.write_qword(handle_out, h);
    }
    emu.regs_mut().rax = STATUS_SUCCESS;
}

/// Generic ALPC info-query stub: zero-fills the output buffer and returns STATUS_SUCCESS.
fn alpc_query_info(emu: &mut Emu, nr: u64, name: &str) {
    let info_buf = emu.regs().r9;
    let rsp = emu.regs().rsp;
    let length = emu.maps.read_qword(rsp + 0x28).unwrap_or(0);
    log_orange!(
        emu,
        "syscall 0x{:x}: {} buf: 0x{:x} len: 0x{:x}",
        nr,
        name,
        info_buf,
        length
    );
    if info_buf != 0 && emu.maps.is_mapped(info_buf) {
        let sz = length.min(0x1000) as usize;
        for i in 0..sz {
            let _ = emu.maps.write_byte(info_buf + i as u64, 0);
        }
    }
    emu.regs_mut().rax = STATUS_SUCCESS;
}

/// Generic ALPC stub that simply returns STATUS_SUCCESS (no output).
fn alpc_noop(emu: &mut Emu, nr: u64, name: &str) {
    log_orange!(emu, "syscall 0x{:x}: {} (stub)", nr, name);
    emu.regs_mut().rax = STATUS_SUCCESS;
}

/// `NtAlpcConnectPort` — client connects to an ALPC server port.
///
/// x64: RCX=PortHandle(out), RDX=PortName, R8=ObjAttr, R9=PortAttr,
/// [rsp+28]=Flags, [rsp+30]=RequiredServerSid, [rsp+38]=ConnectionMessage(in/out),
/// [rsp+40]=BufferLength.
/// Writes a fake handle, zero-fills ConnectionMessage reply so ReturnValue=0=SUCCESS.
pub fn nt_alpc_connect_port(emu: &mut Emu) {
    let handle_out = emu.regs().rcx;
    let rsp = emu.regs().rsp;
    let conn_msg_ptr = emu.maps.read_qword(rsp + 0x38).unwrap_or(0);

    log_orange!(
        emu,
        "syscall 0x{:x}: NtAlpcConnectPort out: 0x{:x}, conn_msg: 0x{:x}",
        WIN64_NTALPCCONNECTPORT,
        handle_out,
        conn_msg_ptr,
    );

    if handle_out != 0 && emu.maps.is_mapped(handle_out) {
        let h = sync::next_handle();
        let _ = emu.maps.write_qword(handle_out, h);
    }

    // Zero-fill the connection message reply so ReturnValue == STATUS_SUCCESS.
    if conn_msg_ptr != 0 && emu.maps.is_mapped(conn_msg_ptr) {
        for off in 0..0x200u64 {
            let _ = emu.maps.write_byte(conn_msg_ptr + off, 0);
        }
    }

    emu.regs_mut().rax = STATUS_SUCCESS;
}
pub fn nt_alpc_connect_port_ex(emu: &mut Emu) {
    alpc_create_handle(emu, WIN64_NTALPCCONNECTPORTEX, "NtAlpcConnectPortEx");
}
pub fn nt_alpc_create_port(emu: &mut Emu) {
    alpc_create_handle(emu, WIN64_NTALPCCREATEPORT, "NtAlpcCreatePort");
}
pub fn nt_alpc_create_port_section(emu: &mut Emu) {
    alpc_create_handle(
        emu,
        WIN64_NTALPCCREATEPORTSECTION,
        "NtAlpcCreatePortSection",
    );
}
pub fn nt_alpc_create_resource_reserve(emu: &mut Emu) {
    alpc_create_handle(
        emu,
        WIN64_NTALPCCREATERESOURCERESERVE,
        "NtAlpcCreateResourceReserve",
    );
}
pub fn nt_alpc_create_section_view(emu: &mut Emu) {
    alpc_create_handle(
        emu,
        WIN64_NTALPCCREATESECTIONVIEW,
        "NtAlpcCreateSectionView",
    );
}
pub fn nt_alpc_create_security_context(emu: &mut Emu) {
    alpc_create_handle(
        emu,
        WIN64_NTALPCCREATESECURITYCONTEXT,
        "NtAlpcCreateSecurityContext",
    );
}

pub fn nt_alpc_query_information(emu: &mut Emu) {
    alpc_query_info(emu, WIN64_NTALPCQUERYINFORMATION, "NtAlpcQueryInformation");
}

/// `NtAlpcQueryInformationMessage` — 0x8c.
///
/// RCX = PortHandle, RDX = PortMessage, R8 = MessageInfoClass,
/// R9 = MessageInformation (out), [rsp+28] = Length, [rsp+30] = ReturnLength.
pub fn nt_alpc_query_information_message(emu: &mut Emu) {
    let port = emu.regs().rcx;
    let class = emu.regs().r8;
    let info = emu.regs().r9;
    let rsp = emu.regs().rsp;
    let length = emu.maps.read_qword(rsp + 0x28).unwrap_or(0);
    let ret_len = emu.maps.read_qword(rsp + 0x30).unwrap_or(0);

    log_orange!(
        emu,
        "syscall 0x{:x}: NtAlpcQueryInformationMessage port: 0x{:x}, class: 0x{:x}, buf: 0x{:x}, len: 0x{:x}",
        WIN64_NTALPCQUERYINFORMATIONMESSAGE,
        port,
        class,
        info,
        length,
    );

    if info != 0 && emu.maps.is_mapped(info) {
        let sz = length.min(0x1000) as usize;
        for i in 0..sz {
            let _ = emu.maps.write_byte(info + i as u64, 0);
        }
    }
    if ret_len != 0 && emu.maps.is_mapped(ret_len) {
        let _ = emu.maps.write_qword(ret_len, length.min(0x1000));
    }
    emu.regs_mut().rax = STATUS_SUCCESS;
}

pub fn nt_alpc_cancel_message(emu: &mut Emu) {
    alpc_noop(emu, WIN64_NTALPCCANCELMESSAGE, "NtAlpcCancelMessage");
}
pub fn nt_alpc_delete_port_section(emu: &mut Emu) {
    alpc_noop(
        emu,
        WIN64_NTALPCDELETEPORTSECTION,
        "NtAlpcDeletePortSection",
    );
}
pub fn nt_alpc_delete_resource_reserve(emu: &mut Emu) {
    alpc_noop(
        emu,
        WIN64_NTALPCDELETERESOURCERESERVE,
        "NtAlpcDeleteResourceReserve",
    );
}
pub fn nt_alpc_delete_section_view(emu: &mut Emu) {
    alpc_noop(
        emu,
        WIN64_NTALPCDELETESECTIONVIEW,
        "NtAlpcDeleteSectionView",
    );
}
pub fn nt_alpc_delete_security_context(emu: &mut Emu) {
    alpc_noop(
        emu,
        WIN64_NTALPCDELETESECURITYCONTEXT,
        "NtAlpcDeleteSecurityContext",
    );
}
pub fn nt_alpc_disconnect_port(emu: &mut Emu) {
    alpc_noop(emu, WIN64_NTALPCDISCONNECTPORT, "NtAlpcDisconnectPort");
}
pub fn nt_alpc_impersonate_client_container_of_port(emu: &mut Emu) {
    alpc_noop(
        emu,
        WIN64_NTALPCIMPERSONATECLIENTCONTAINEROFPORT,
        "NtAlpcImpersonateClientContainerOfPort",
    );
}
pub fn nt_alpc_impersonate_client_of_port(emu: &mut Emu) {
    alpc_noop(
        emu,
        WIN64_NTALPCIMPERSONATECLIENTOFPORT,
        "NtAlpcImpersonateClientOfPort",
    );
}
pub fn nt_alpc_open_sender_process(emu: &mut Emu) {
    alpc_create_handle(
        emu,
        WIN64_NTALPCOPENSENDERPROCESS,
        "NtAlpcOpenSenderProcess",
    );
}
pub fn nt_alpc_open_sender_thread(emu: &mut Emu) {
    alpc_create_handle(emu, WIN64_NTALPCOPENSENDERTHREAD, "NtAlpcOpenSenderThread");
}
pub fn nt_alpc_revoke_security_context(emu: &mut Emu) {
    alpc_noop(
        emu,
        WIN64_NTALPCREVOKESECURITYCONTEXT,
        "NtAlpcRevokeSecurityContext",
    );
}

// PORT_MESSAGE reply type (Windows LPC).
const LPC_REPLY: u16 = 2;
// x64 PORT_MESSAGE size.
const PORT_MESSAGE_SIZE: u64 = 0x28;

/// `NtAlpcSendWaitReceivePort` — 0x8c.
///
/// x64: RCX=PortHandle, RDX=Flags, R8=SendMessage, R9=SendMsgAttr,
/// [rsp+28]=ReceiveMessage(out), [rsp+30]=BufferLength, [rsp+38]=RecvMsgAttr, [rsp+40]=Timeout.
///
/// Writes a minimal PORT_MESSAGE header with Type=LPC_REPLY to the receive buffer so ntdll's
/// CsrClientConnectToServer reply checks succeed.
pub fn nt_alpc_send_wait_receive_port(emu: &mut Emu) {
    let port = emu.regs().rcx;
    let send_msg_ptr = emu.regs().r8;
    let rsp = emu.regs().rsp;
    let recv_msg_ptr = emu.maps.read_qword(rsp + 0x28).unwrap_or(0);

    // Read DataLength from the send PORT_MESSAGE header (+0x00 = u1.s1.DataLength, CSHORT).
    let send_data_len = if send_msg_ptr != 0 && emu.maps.is_mapped(send_msg_ptr) {
        emu.maps.read_word(send_msg_ptr).unwrap_or(0)
    } else {
        0
    };

    log_orange!(
        emu,
        "syscall 0x{:x}: NtAlpcSendWaitReceivePort port: 0x{:x}, send: 0x{:x}, recv_buf: 0x{:x}",
        WIN64_NTALPCSENDWAITRECEIVEPORT,
        port,
        send_msg_ptr,
        recv_msg_ptr,
    );

    if recv_msg_ptr != 0 && emu.maps.is_mapped(recv_msg_ptr) {
        // Zero-fill data portion (after PORT_MESSAGE header) for any ReturnValue checks.
        let data_len = send_data_len.min(0x1e0) as u64;
        for off in 0..(PORT_MESSAGE_SIZE + data_len) {
            let _ = emu.maps.write_byte(recv_msg_ptr + off, 0);
        }
        // Write PORT_MESSAGE header: DataLength, TotalLength, then Type=LPC_REPLY at +0x04.
        let _ = emu.maps.write_word(recv_msg_ptr, send_data_len);
        let _ = emu.maps.write_word(
            recv_msg_ptr + 0x02,
            PORT_MESSAGE_SIZE as u16 + send_data_len,
        );
        let _ = emu.maps.write_word(recv_msg_ptr + 0x04, LPC_REPLY);
    }

    emu.regs_mut().rax = STATUS_SUCCESS;
}

pub fn nt_alpc_set_information(emu: &mut Emu) {
    alpc_noop(emu, WIN64_NTALPCSETINFORMATION, "NtAlpcSetInformation");
}
