use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::maps::mem64::Permission;
use crate::threading::context::ThreadContext;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CreateRemoteThread(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let proc_hndl = api_try!(emu, call.arg(emu, 0));
    let sec = api_try!(emu, call.arg(emu, 1));
    let stack_size = api_try!(emu, call.arg(emu, 2));
    let addr = api_try!(emu, call.arg(emu, 3));
    let param = api_try!(emu, call.arg(emu, 4));
    let flags = api_try!(emu, call.arg(emu, 5));
    let out_tid = api_try!(emu, call.arg(emu, 6));

    log_red!(
        emu,
        "kernel32!CreateRemoteThread hproc: 0x{:x} addr: 0x{:x}",
        proc_hndl,
        addr
    );
    let new_thread_id = 0x1000 + emu.threads.len();
    let mut new_thread = ThreadContext::new(new_thread_id as u64, emu.cfg.arch);

    // Initialize thread context with entry point and parameter
    {
        let regs = new_thread.regs_x86_mut();
        regs.rip = addr;
        regs.rcx = param;
        regs.rax = 0;
    }

    // Allocate stack if requested (otherwise will share/reuse current stack)
    if stack_size > 0
        && let Some(stack_base) = emu.maps.alloc(stack_size)
    {
        let regs = new_thread.regs_x86_mut();
        regs.rsp = stack_base + stack_size - 8; // Stack grows down
        regs.rbp = regs.rsp;
        emu.maps
            .create_map(
                &format!("remote_thread_stack_{:x}", new_thread_id),
                stack_base,
                stack_size,
                Permission::READ_WRITE,
            )
            .ok();
    }

    // Sync FPU instruction pointer
    if let crate::threading::context::ArchThreadState::X86 { fpu, .. } = &mut new_thread.arch {
        fpu.set_ip(addr);
    }

    emu.threads.push(new_thread);

    if out_tid > 0 {
        emu.maps.write_dword(out_tid, new_thread_id as u32);
    }
    call.set_return(
        emu,
        helper::handler_create(&format!("tid://0x{:x}", new_thread_id)),
    );
}
