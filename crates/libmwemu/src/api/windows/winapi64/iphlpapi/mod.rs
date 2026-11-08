use crate::api::windows::common::fallback;
use crate::emu;
use crate::winapi::winapi64;


mod get_adapters_addresses;
use get_adapters_addresses::GetAdaptersAddresses;
mod get_adapters_info;
use get_adapters_info::GetAdaptersInfo;
mod get_best_route;
use get_best_route::GetBestRoute;
mod get_if_table;
use get_if_table::GetIfTable;
mod get_ip_addr_table;
use get_ip_addr_table::GetIpAddrTable;
mod get_network_params;
use get_network_params::GetNetworkParams;
mod get_tcp_table;
use get_tcp_table::GetTcpTable;
mod get_udp_table;
use get_udp_table::GetUdpTable;

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

