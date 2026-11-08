use crate::emu;

pub fn getaddrinfo(emu: &mut emu::Emu) {
    let node_name_ptr = emu.regs().rcx;
    let service_name_ptr = emu.regs().rdx;
    let hints_ptr = emu.regs().r8;
    let result_ptr_ptr = emu.regs().r9;

    let node_name = if node_name_ptr != 0 {
        emu.maps.read_string(node_name_ptr)
    } else {
        "NULL".to_string()
    };

    let service_name = if service_name_ptr != 0 {
        emu.maps.read_string(service_name_ptr)
    } else {
        "NULL".to_string()
    };

    log_red!(
        emu,
        "ws2_32!getaddrinfo node: `{}` service: `{}`",
        node_name,
        service_name
    );

    // Read hints if provided
    let mut hints_flags = 0;
    let mut hints_family = 0;
    let mut hints_socktype = 0;
    let mut hints_protocol = 0;

    if hints_ptr != 0 {
        hints_flags = emu.maps.read_dword(hints_ptr).unwrap_or(0) as i32;
        hints_family = emu.maps.read_dword(hints_ptr + 4).unwrap_or(0) as i32;
        hints_socktype = emu.maps.read_dword(hints_ptr + 8).unwrap_or(0) as i32;
        hints_protocol = emu.maps.read_dword(hints_ptr + 12).unwrap_or(0) as i32;
    }

    // Create a dummy ADDRINFO structure
    let addrinfo_size = 48; // Size of ADDRINFOA structure (approximate)
    let sockaddr_in_size = 16; // Size of sockaddr_in structure

    // Allocate memory for the result
    let heap_management = emu.heap_mut();
    let addrinfo_addr =
        match heap_management.allocate((addrinfo_size + sockaddr_in_size + 100) as usize) {
            Some(addr) => addr,
            None => {
                log::warn!("ws2_32!getaddrinfo: heap allocation failed");
                emu.regs_mut().rax = 1;
                return;
            }
        };
    let sockaddr_addr = addrinfo_addr + addrinfo_size;
    let canonname_addr = sockaddr_addr + sockaddr_in_size;

    // Create a dummy sockaddr_in (IPv4 address 127.0.0.1, port based on service)
    let ip_addr = 0x0100007f; // 127.0.0.1 in network byte order

    // Determine port based on service name
    let port = if service_name == "http" || service_name == "80" {
        80u16
    } else if service_name == "https" || service_name == "443" {
        443u16
    } else if service_name == "ftp" || service_name == "21" {
        21u16
    } else if service_name == "ssh" || service_name == "22" {
        22u16
    } else if service_name == "smtp" || service_name == "25" {
        25u16
    } else if service_name == "dns" || service_name == "53" {
        53u16
    } else {
        service_name.parse().unwrap_or(80u16)
    };

    // Write sockaddr_in structure
    emu.maps.write_word(sockaddr_addr, 2); // AF_INET = 2
    emu.maps.write_word(sockaddr_addr + 2, port.to_be()); // Port in network byte order
    emu.maps.write_dword(sockaddr_addr + 4, ip_addr); // IP address (127.0.0.1)
    emu.maps.memset(sockaddr_addr + 8, 0, 8); // Zero out the rest

    // Write ADDRINFO structure
    emu.maps.write_dword(addrinfo_addr, hints_flags as u32); // ai_flags
    emu.maps.write_dword(
        addrinfo_addr + 4,
        if hints_family != 0 {
            hints_family as u32
        } else {
            2
        },
    ); // ai_family (AF_INET)
    emu.maps.write_dword(
        addrinfo_addr + 8,
        if hints_socktype != 0 {
            hints_socktype as u32
        } else {
            1
        },
    ); // ai_socktype (SOCK_STREAM)
    emu.maps.write_dword(
        addrinfo_addr + 12,
        if hints_protocol != 0 {
            hints_protocol as u32
        } else {
            6
        },
    ); // ai_protocol (IPPROTO_TCP)
    emu.maps.write_qword(addrinfo_addr + 16, sockaddr_in_size); // ai_addrlen
    emu.maps.write_qword(addrinfo_addr + 24, canonname_addr); // ai_canonname
    emu.maps.write_qword(addrinfo_addr + 32, sockaddr_addr); // ai_addr

    // Set ai_canonname to the node name or "localhost"
    let canon_name = if node_name == "NULL" || node_name.is_empty() || node_name == "localhost" {
        "localhost.localdomain".to_string()
    } else if node_name == "127.0.0.1" {
        "localhost".to_string()
    } else {
        format!("{}.localdomain", node_name)
    };
    emu.maps.write_string(canonname_addr, &canon_name);

    // ai_next is NULL (end of list)
    emu.maps.write_qword(addrinfo_addr + 40, 0);

    // Store the result pointer in the ppResult parameter
    emu.maps.write_qword(result_ptr_ptr, addrinfo_addr);

    log::trace!(
        "\tcreated dummy ADDRINFO for {}:{} at 0x{:x}",
        node_name,
        service_name,
        addrinfo_addr
    );
    log::trace!(
        "\tsockaddr at 0x{:x}, canonname at 0x{:x}",
        sockaddr_addr,
        canonname_addr
    );

    // Return 0 for success (WSA error code)
    emu.regs_mut().rax = 0;
}
