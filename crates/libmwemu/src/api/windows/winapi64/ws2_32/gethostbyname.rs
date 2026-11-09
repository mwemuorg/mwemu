use crate::emu;
use crate::maps::mem64::Permission;
use crate::windows::structures::Hostent;

pub fn gethostbyname(emu: &mut emu::Emu) {
    let domain_name_ptr = emu.regs().rcx;
    let domain_name = emu.maps.read_string(domain_name_ptr);

    log_red!(emu, "ws2_32!gethostbyname `{}`", domain_name);

    let addr = match emu.maps.alloc(1024) {
        Some(a) => a,
        None => {
            log::error!("ws2_32!gethostbyname: out of memory");
            emu.regs_mut().rax = 0;
            return;
        }
    };
    let str_addr = addr + 1024 - 100;
    let mem = match emu
        .maps
        .create_map("hostent", addr, 1024, Permission::READ_WRITE)
    {
        Ok(m) => m,
        Err(e) => {
            log::error!("ws2_32!gethostbyname: cannot create hostent map: {}", e);
            emu.regs_mut().rax = 0;
            return;
        }
    };
    mem.write_dword(addr, 0x04030201);
    mem.write_qword(addr + 8, addr);
    mem.write_qword(addr + 16, 0);
    mem.write_string(str_addr, &domain_name);

    let mut hostent = Hostent::new();
    hostent.hname = str_addr;
    hostent.alias_list = 0;
    hostent.length = 4;
    hostent.addr_list = addr + 8;
    hostent.save(addr + 30, &mut emu.maps);

    emu.regs_mut().rax = addr + 30;
}
