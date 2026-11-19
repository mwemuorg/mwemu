use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::maps::mem64::Permission;
use crate::winapi::policy::api_try;
use crate::windows::structures::Hostent;

pub fn gethostbyname(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let name = emu
        .maps
        .read_string(api_try!(emu, call.arg_u32(emu, 0)) as u64);
    log_red!(emu, "ws2_32!gethostbyname `{}`", name);
    // One block holds the hostent, the address and the name string; the
    // resolved address is a fixed 1.2.3.4 so callers get a well-formed result.
    let block = match emu.maps.alloc(1024) {
        Some(a) => a,
        None => {
            log::error!("ws2_32!gethostbyname: out of memory");
            call.cleanup_args(emu, 1);
            call.set_return(emu, 0);
            return;
        }
    };
    if emu
        .maps
        .create_map("hostent", block, 1024, Permission::READ_WRITE)
        .is_err()
    {
        log::error!("ws2_32!gethostbyname: cannot create hostent map");
        call.cleanup_args(emu, 1);
        call.set_return(emu, 0);
        return;
    }
    let name_addr = block + 1024 - 100;
    emu.maps.write_dword(block, 0x04030201); // the IP
    emu.maps.write_dword(block + 8, block as u32); // addr_list[0]
    emu.maps.write_dword(block + 12, 0); // addr_list NULL terminator
    emu.maps.write_string(name_addr, &name);
    // 32-bit hostent: h_name, h_aliases, h_addrtype, h_length, h_addr_list
    let hostent = block + 30;
    emu.maps.write_dword(hostent, name_addr as u32);
    emu.maps.write_dword(hostent + 4, 0);
    emu.maps.write_word(hostent + 8, 2); // AF_INET
    emu.maps.write_word(hostent + 10, 4);
    emu.maps.write_dword(hostent + 12, (block + 8) as u32);
    let _ = Hostent::size();
    call.set_return(emu, hostent);
    call.cleanup_args(emu, 1);
}
