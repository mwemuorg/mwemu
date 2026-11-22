use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn GlobalReAlloc(emu: &mut emu::Emu, addr: u64) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    let size = api_try!(emu, call.arg(emu, 1));
    let flags = api_try!(emu, call.arg(emu, 2));
    let new_ptr = if size > 0 {
        if let Some(addr) = emu.maps.alloc(size) {
            // Copy old data if we had it
            if let Some(old_mem) = emu.maps.get_mem_by_addr(handle) {
                let copy_size = old_mem.size().min(size as usize);
                for i in 0..copy_size {
                    let b = emu.maps.read_byte(handle + i as u64).unwrap_or(0);
                    emu.maps.write_byte(addr + i as u64, b);
                }
            }
            addr
        } else {
            0
        }
    } else {
        handle
    };
    log_red!(
        emu,
        "kernel32!GlobalReAlloc handle=0x{:x} size={} flags=0x{:x} => 0x{:x}",
        handle,
        size,
        flags,
        new_ptr
    );
    emu.set_last_error(0);
    call.set_return(emu, new_ptr);
}
