use crate::api::windows::common::fallback;
use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi64;

const ERROR_BUFFER_OVERFLOW: u64 = 111;
const ERROR_NOT_SUPPORTED: u64 = 50;
const NO_ERROR: u64 = 0;

pub fn gateway(addr: u64, emu: &mut emu::Emu) -> String {
    let api = winapi64::kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    match api {
        "GetAdaptersInfo" => GetAdaptersInfo(emu),
        "GetAdaptersAddresses" => GetAdaptersAddresses(emu),
        "GetBestRoute" => GetBestRoute(emu),
        "GetNetworkParams" => GetNetworkParams(emu),
        "GetIfTable" => GetIfTable(emu),
        "GetIpAddrTable" => GetIpAddrTable(emu),
        "GetTcpTable" => GetTcpTable(emu),
        "GetUdpTable" => GetUdpTable(emu),
        "GetBestInterface" => GetBestInterface(emu),
        "GetIpForwardTable" => GetIpForwardTable(emu),
        "GetIpNetTable" => GetIpNetTable(emu),
        "GetPerAdapterInfo" => GetPerAdapterInfo(emu),
        "GetExtendedTcpTable" => GetExtendedTcpTable(emu),
        "GetExtendedUdpTable" => GetExtendedUdpTable(emu),
        "NotifyAddrChange" => NotifyAddrChange(emu),
        "IcmpCreateFile" => IcmpCreateFile(emu),
        "IcmpCloseHandle" => IcmpCloseHandle(emu),
        "IcmpSendEcho" => IcmpSendEcho(emu),
        "SendARP" => SendARP(emu),
        "GetIpStatistics" => GetIpStatistics(emu),
        "GetTcpStatistics" => GetTcpStatistics(emu),
        "GetUdpStatistics" => GetUdpStatistics(emu),
        "GetIcmpStatistics" => GetIcmpStatistics(emu),
        "GetNumberOfInterfaces" => GetNumberOfInterfaces(emu),
        "GetInterfaceInfo" => GetInterfaceInfo(emu),
        "GetUniDirectionalAdapterInfo" => GetUniDirectionalAdapterInfo(emu),
        "GetFriendlyIfIndex" => GetFriendlyIfIndex(emu),
        "GetIfEntry" => GetIfEntry(emu),
        "AddIPAddress" => AddIPAddress(emu),
        "DeleteIPAddress" => DeleteIPAddress(emu),
        "GetIpForwardEntry2" => GetIpForwardEntry2(emu),
        "FlushIpNetTable" => FlushIpNetTable(emu),
        "Icmp6CreateFile" => Icmp6CreateFile(emu),
        "IcmpSendEcho2" => IcmpSendEcho2(emu),
        "GetTcp6Table" => GetTcp6Table(emu),
        "GetUdp6Table" => GetUdp6Table(emu),
        "GetIpStatisticsEx" => GetIpStatisticsEx(emu),
        "GetTcpStatisticsEx" => GetTcpStatisticsEx(emu),
        "GetUdpStatisticsEx" => GetUdpStatisticsEx(emu),
        "GetIcmpStatisticsEx" => GetIcmpStatisticsEx(emu),
        "GetExtendedTcpTable2" | "InternalGetExtendedTcpTable2" => GetExtendedTcpTable(emu),
        "GetOwnerModuleFromTcpEntry" => GetOwnerModuleFromTcpEntry(emu),
        "GetOwnerModuleFromUdpEntry" => GetOwnerModuleFromUdpEntry(emu),
        "GetOwnerModuleFromTcp6Entry" => GetOwnerModuleFromTcpEntry(emu),
        "GetOwnerModuleFromUdp6Entry" => GetOwnerModuleFromUdpEntry(emu),
        "GetBestRoute2" => GetBestRoute2(emu),
        "GetBestInterfaceEx" => GetBestInterfaceEx(emu),
        "GetAdapterIndex" => GetAdapterIndex(emu),
        "GetPerTcpConnectionEStats" => GetPerTcpConnectionEStats(emu),
        "GetRTTAndHopCount" => GetRTTAndHopCount(emu),
        "SetIpForwardEntry" => SetIpForwardEntry(emu),
        "CreateIpForwardEntry" => CreateIpForwardEntry(emu),
        "DeleteIpForwardEntry" => DeleteIpForwardEntry(emu),
        "SetIpStatistics" => SetIpStatistics(emu),
        "SetIpTTL" => SetIpTTL(emu),
        "GetIfTable2" => GetIfTable2(emu),
        "FreeMibTable" => FreeMibTable(emu),
        "ConvertInterfaceIndexToLuid" => ConvertInterfaceIndexToLuid(emu),
        "ConvertInterfaceLuidToIndex" => ConvertInterfaceLuidToIndex(emu),
        "ConvertInterfaceLuidToNameA" => ConvertInterfaceLuidToNameA(emu),
        "ConvertInterfaceLuidToNameW" => ConvertInterfaceLuidToNameW(emu),
        "ConvertInterfaceNameToLuidA" => ConvertInterfaceNameToLuidA(emu),
        "ConvertInterfaceNameToLuidW" => ConvertInterfaceNameToLuidW(emu),
        "if_nametoindex" => if_nametoindex(emu),
        "if_indextoname" => if_indextoname(emu),
        "GetIfEntry2" => GetIfEntry2(emu),
        "GetUnicastIpAddressTable" => GetUnicastIpAddressTable(emu),
        "GetAnycastIpAddressTable" => GetAnycastIpAddressTable(emu),
        "GetMulticastIpAddressTable" => GetMulticastIpAddressTable(emu),
        "NotifyIpInterfaceChange" => NotifyIpInterfaceChange(emu),
        "CancelMibChangeNotify2" => CancelMibChangeNotify2(emu),
        "ResolveIpNetEntry2" => ResolveIpNetEntry2(emu),
        "CreateUnicastIpAddressEntry" => CreateUnicastIpAddressEntry(emu),
        "DeleteUnicastIpAddressEntry" => DeleteUnicastIpAddressEntry(emu),
        "CreateIpForwardEntry2" => CreateIpForwardEntry2(emu),
        "DeleteIpForwardEntry2" => DeleteIpForwardEntry2(emu),
        "SetIpForwardEntry2" => SetIpForwardEntry2(emu),
        "GetIpForwardTable2" => GetIpForwardTable2(emu),
        "GetIpNetTable2" => GetIpNetTable2(emu),
        "GetUnicastIpAddressEntry" => GetUnicastIpAddressEntry(emu),
        "InitializeUnicastIpAddressEntry" => InitializeUnicastIpAddressEntry(emu),
        "NotifyRouteChange" => NotifyRouteChange(emu),
        "NotifyRouteChange2" => NotifyRouteChange2(emu),
        "NotifyUnicastIpAddressChange" => NotifyUnicastIpAddressChange(emu),
        "EnableRouter" => EnableRouter(emu),
        "UnenableRouter" => UnenableRouter(emu),

        _ => return fallback::unimplemented(addr, api, emu),
    }

    String::new()
}

fn GetAdaptersInfo(emu: &mut emu::Emu) {
    let adapter_info = emu.regs().rcx;
    let out_buf_len = emu.regs().rdx;

    log_red!(
        emu,
        "iphlpapi!GetAdaptersInfo pAdapterInfo:0x{:x} pOutBufLen:0x{:x}",
        adapter_info,
        out_buf_len
    );

    if out_buf_len != 0 {
        emu.maps.write_dword(out_buf_len, 640);
    }

    emu.regs_mut().rax = ERROR_BUFFER_OVERFLOW;
}

fn GetAdaptersAddresses(emu: &mut emu::Emu) {
    let family = emu.regs().rcx;
    let flags = emu.regs().rdx;
    let reserved = emu.regs().r8;
    let adapter_addresses = emu.regs().r9;
    let size_pointer = emu
        .maps
        .read_qword(emu.regs().rsp + 0x20)
        .unwrap_or(0);

    log_red!(
        emu,
        "iphlpapi!GetAdaptersAddresses family:{} flags:0x{:x} pAddresses:0x{:x}",
        family,
        flags,
        adapter_addresses
    );

    if size_pointer != 0 {
        emu.maps.write_dword(size_pointer, 640);
    }

    emu.regs_mut().rax = ERROR_BUFFER_OVERFLOW;
}

fn GetBestRoute(emu: &mut emu::Emu) {
    let dest_addr = emu.regs().rcx;
    let source_addr = emu.regs().rdx;
    let best_route = emu.regs().r8;

    log_red!(
        emu,
        "iphlpapi!GetBestRoute dest:0x{:x} src:0x{:x} pBestRoute:0x{:x}",
        dest_addr,
        source_addr,
        best_route
    );

    // Fill minimal MIB_IPFORWARDROW (76 bytes) with zeroes if pointer is valid
    if best_route != 0 {
        for i in 0..19 {
            emu.maps.write_dword(best_route + i * 4, 0);
        }
    }

    emu.regs_mut().rax = NO_ERROR;
}

fn GetNetworkParams(emu: &mut emu::Emu) {
    let fixed_info = emu.regs().rcx;
    let out_buf_len = emu.regs().rdx;

    log_red!(
        emu,
        "iphlpapi!GetNetworkParams pFixedInfo:0x{:x} pOutBufLen:0x{:x}",
        fixed_info,
        out_buf_len
    );

    if out_buf_len != 0 {
        emu.maps.write_dword(out_buf_len, 312);
    }

    emu.regs_mut().rax = ERROR_BUFFER_OVERFLOW;
}

fn GetIfTable(emu: &mut emu::Emu) {
    let table = emu.regs().rcx;
    let size = emu.regs().rdx;
    let order = emu.regs().r8;

    log_red!(
        emu,
        "iphlpapi!GetIfTable table:0x{:x} order:{}",
        table,
        order
    );

    if size != 0 {
        emu.maps.write_dword(size, 1024);
    }

    emu.regs_mut().rax = ERROR_BUFFER_OVERFLOW;
}

fn GetIpAddrTable(emu: &mut emu::Emu) {
    let table = emu.regs().rcx;
    let size = emu.regs().rdx;
    let order = emu.regs().r8;

    log_red!(
        emu,
        "iphlpapi!GetIpAddrTable table:0x{:x} order:{}",
        table,
        order
    );

    if size != 0 {
        emu.maps.write_dword(size, 512);
    }

    emu.regs_mut().rax = ERROR_BUFFER_OVERFLOW;
}

fn GetTcpTable(emu: &mut emu::Emu) {
    let table = emu.regs().rcx;
    let size = emu.regs().rdx;
    let order = emu.regs().r8;

    log_red!(
        emu,
        "iphlpapi!GetTcpTable table:0x{:x} order:{}",
        table,
        order
    );

    if size != 0 {
        emu.maps.write_dword(size, 256);
    }

    emu.regs_mut().rax = ERROR_BUFFER_OVERFLOW;
}

fn GetUdpTable(emu: &mut emu::Emu) {
    let table = emu.regs().rcx;
    let size = emu.regs().rdx;
    let order = emu.regs().r8;

    log_red!(
        emu,
        "iphlpapi!GetUdpTable table:0x{:x} order:{}",
        table,
        order
    );

    if size != 0 {
        emu.maps.write_dword(size, 256);
    }

    emu.regs_mut().rax = ERROR_BUFFER_OVERFLOW;
}
