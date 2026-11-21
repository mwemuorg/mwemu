//! Network-device and USB driver surface.
//!
//! Enough of the netdev / sk_buff / URB API to let a NIC or wifi driver's
//! `probe` run: the object-lifetime calls (allocate a `net_device`, an URB, an
//! `sk_buff`, and free them) funnel through the ledger so a double-free or
//! use-after-free on a probe/teardown error path is caught, while the rest
//! (carrier state, queue control, ethtool helpers) are benign no-ops.

use crate::api::abi::unix::UnixApi;
use crate::api::dispatch_api;
use crate::api::unix::{MemoryAccess, api_try, memory};
use crate::emu::Emu;
use crate::kernel::heap::Region;
use crate::kernel::{HandlerRegistry, kernel_handlers, return_zero};

/// Space reserved before the driver's private area in a `net_device`
/// allocation. `netdev_priv()` is inlined in the driver as
/// `(char*)dev + ALIGN(sizeof(struct net_device), 32)`, a fixed offset baked
/// in at build time; reserving comfortably more than any real `net_device`
/// keeps that inlined access inside the chunk we hand back.
const NETDEV_RESERVE: u64 = 0x2000;
const USB_COHERENT_MAX_SIZE: u64 = 0x100000;

/// Bytes reserved for the `struct ieee80211_hw` header before the driver's
/// private area, and the offset of its `priv` pointer field. The header size is
/// comfortably larger than any real `ieee80211_hw` so the driver's field
/// accesses stay inside the chunk; the priv offset tracks current x86-64
/// mac80211 (`priv` follows the embedded `struct ieee80211_conf`).
const IEEE80211_HW_RESERVE: u64 = 0x800;
const IEEE80211_PRIV_OFF: u64 = 0x58;

fn alloc(emu: &mut Emu, size: u64, cache: &str, api: &str, zeroed: bool) -> u64 {
    let call = UnixApi;
    let ptr = emu.kernel_alloc(Region::Slab, size, cache, api, zeroed);
    call.set_return(emu, ptr);
    ptr
}

fn free(emu: &mut Emu, ptr: u64, api: &str) {
    let call = UnixApi;
    if ptr != 0 {
        emu.kernel_free(ptr, api);
    }
    call.set_return(emu, 0);
}

pub fn dispatch(symbol: &str, emu: &mut Emu) -> bool {
    dispatch_api!(symbol, emu, registry handlers(),
        handled(()) => true,
        fallback => false,
    )
}

pub const fn handlers() -> HandlerRegistry {
    kernel_handlers! {
        // --- net_device -------------------------------------------------------
        // alloc_etherdev_mqs(sizeof_priv, txqs, rxqs) and the generic
        // alloc_netdev_mqs(sizeof_priv, name, assign, setup, txqs, rxqs).
        "alloc_etherdev_mqs" | "alloc_etherdev_mq" | "alloc_etherdev" => alloc_etherdev_mqs,
        "alloc_netdev_mqs" => alloc_etherdev_mqs,
        // devm_ variants take the device as arg0, so sizeof_priv is arg1. The
        // returned net_device is managed, but the driver reads netdev_priv() the
        // same way, so the reserve-then-priv layout is identical.
        "devm_alloc_etherdev_mqs" | "devm_alloc_etherdev" => devm_alloc_etherdev_mqs,
        "free_netdev" | "free_candev" => free_netdev,
        // mac80211: ieee80211_alloc_hw[_nm](priv_data_len, ops[, name]).
        // Returns a struct ieee80211_hw* whose `priv` field points at a
        // priv_data_len scratch area. A wifi driver's very first probe step is
        // `hw = ieee80211_alloc_hw(sizeof(priv), ...); priv = hw->priv;`, so
        // returning NULL (the old default) bailed every mac80211 probe at
        // -ENOMEM before any driver code ran. One chunk holds the hw header and
        // the priv area; `hw->priv` is wired to the priv area.
        "ieee80211_alloc_hw" | "ieee80211_alloc_hw_nm" => ieee80211_alloc_hw,
        "ieee80211_free_hw" => ieee80211_free_hw,
        "register_netdev"
        | "register_netdevice"
        | "cfg80211_register_netdevice"
        | "unregister_netdev"
        | "unregister_netdevice"
        | "unregister_netdevice_queue"
        | "dev_addr_mod"
        | "eth_validate_addr"
        | "eth_mac_addr"
        | "eth_type_trans"
        | "ether_setup"
        | "eth_hw_addr_random"
        | "eth_commit_mac_addr_change" => return_zero,
        // netif_* carrier / queue / rx — no scheduler here.

        // ethtool helpers, netdev logging.
        "netdev_notice"
        | "netdev_info"
        | "netdev_warn"
        | "netdev_err"
        | "netdev_dbg"
        | "__dynamic_netdev_dbg"
        | "__dynamic_dev_dbg"
        | "netdev_rx_csum_fault" => return_zero,

        // --- sk_buff ----------------------------------------------------------
        // __netdev_alloc_skb(dev, len, gfp) / napi_alloc_skb / __alloc_skb.
        // One chunk stands in for the sk_buff plus its linear data area; the
        // driver's skb_put/skb_pull walk pointers we return, not real state.
        "__netdev_alloc_skb" | "netdev_alloc_skb" | "napi_alloc_skb" | "__napi_alloc_skb"
        | "dev_alloc_skb" => __netdev_alloc_skb,
        "__alloc_skb" | "alloc_skb" | "build_skb" => __alloc_skb,
        "consume_skb"
        | "kfree_skb"
        | "kfree_skb_reason"
        | "dev_kfree_skb"
        | "dev_kfree_skb_any_reason"
        | "dev_kfree_skb_irq_reason"
        | "napi_consume_skb"
        | "__kfree_skb" => consume_skb,
        // --- USB --------------------------------------------------------------
        "usb_alloc_urb" => usb_alloc_urb,
        "usb_free_urb" => usb_free_urb,
        "usb_alloc_coherent" => usb_alloc_coherent,
        "usb_free_coherent" => usb_free_coherent,
        // Capture the struct usb_driver so its real .probe / id_table are
        // reachable after init. usb_register_driver(drv, owner, mod_name).
        "usb_register_driver" => usb_register_driver,
        // Vendor register access over a control transfer. For USB drivers the
        // chip's registers ARE these messages (not a mapped MMIO window), so a
        // read must return coherent data or device-detection reads 0 and bails.
        // usb_control_msg(dev, pipe, request, requesttype, value, index, data,
        //                 size, timeout): register addr = value (arg4).
        "usb_control_msg" => usb_control_msg,
        // usb_control_msg_recv/_send(dev, ep, request, requesttype, value,
        //   index, buf, len, timeout, memflags): recv = read, send = write.
        // Return 0 on success (their convention), not the byte count.
        "usb_control_msg_recv" => usb_control_msg_recv,
        "usb_control_msg_send" => usb_control_msg_send,
        "usb_deregister"
        | "usb_submit_urb"
        | "usb_kill_urb"
        | "usb_unlink_urb"
        | "usb_bulk_msg"
        | "usb_check_bulk_endpoints"
        | "usb_check_int_endpoints"
        | "usb_get_dev"
        | "usb_put_dev"
        | "usb_set_intfdata"
        | "usb_get_intfdata"
        | "usb_reset_device" => return_zero,

        // --- tasklets / NAPI scheduling --------------------------------------
        "tasklet_setup"
        | "tasklet_init"
        | "tasklet_kill"
        | "__tasklet_schedule"
        | "__tasklet_hi_schedule"
        | "netif_napi_add"
        | "netif_napi_add_weight"
        | "__netif_napi_del"
        | "netif_napi_del"
        | "napi_enable"
        | "napi_disable"
        | "napi_complete_done"
        | "napi_schedule"
        | "__napi_schedule" => return_zero,
        guarded {
            is_netif_api => return_zero,
            is_ethtool_api => return_zero,
        }
    }
}

pub const SURFACE: &[&str] = handlers().names;

pub fn has_handler(symbol: &str) -> bool {
    handlers().find(symbol).is_some()
}

fn alloc_etherdev_mqs(emu: &mut Emu, symbol: &str) {
    let call = UnixApi;
    let priv_sz = api_try!(emu, call.arg(emu, 0)).min(0x10000);
    alloc(emu, NETDEV_RESERVE + priv_sz, "net_device", symbol, true);
}

fn devm_alloc_etherdev_mqs(emu: &mut Emu, symbol: &str) {
    let call = UnixApi;
    let priv_sz = api_try!(emu, call.arg(emu, 1)).min(0x10000);
    alloc(emu, NETDEV_RESERVE + priv_sz, "net_device", symbol, true);
}

fn free_netdev(emu: &mut Emu, symbol: &str) {
    let call = UnixApi;
    let dev = api_try!(emu, call.arg(emu, 0));
    free(emu, dev, symbol);
}

fn ieee80211_alloc_hw(emu: &mut Emu, symbol: &str) {
    let call = UnixApi;
    let priv_sz = api_try!(emu, call.arg(emu, 0)).min(0x10000);
    let hw = emu.kernel_alloc(
        Region::Slab,
        IEEE80211_HW_RESERVE + priv_sz,
        "ieee80211_hw",
        symbol,
        true,
    );
    if hw != 0 {
        // `void *priv` sits at this offset in struct ieee80211_hw on
        // current x86-64 kernels (after struct ieee80211_conf).
        emu.maps
            .write_qword(hw + IEEE80211_PRIV_OFF, hw + IEEE80211_HW_RESERVE);
    }
    call.set_return(emu, hw);
}

fn ieee80211_free_hw(emu: &mut Emu, symbol: &str) {
    let call = UnixApi;
    let hw = api_try!(emu, call.arg(emu, 0));
    free(emu, hw, symbol);
}

fn __netdev_alloc_skb(emu: &mut Emu, symbol: &str) {
    let call = UnixApi;
    let len = api_try!(emu, call.arg(emu, 1)).min(0x10000);
    alloc(emu, 0x100 + len + 0x100, "skbuff_head_cache", symbol, true);
}

fn __alloc_skb(emu: &mut Emu, symbol: &str) {
    let call = UnixApi;
    let len = api_try!(emu, call.arg(emu, 0)).min(0x10000);
    alloc(emu, 0x100 + len + 0x100, "skbuff_head_cache", symbol, true);
}

fn consume_skb(emu: &mut Emu, symbol: &str) {
    let call = UnixApi;
    let skb = api_try!(emu, call.arg(emu, 0));
    free(emu, skb, symbol);
}

fn usb_alloc_urb(emu: &mut Emu, symbol: &str) {
    // struct urb is ~0xc0 bytes on x86_64; round up.
    alloc(emu, 0x100, "urb", symbol, true);
}

fn usb_free_urb(emu: &mut Emu, symbol: &str) {
    let call = UnixApi;
    let urb = api_try!(emu, call.arg(emu, 0));
    free(emu, urb, symbol);
}

fn usb_alloc_coherent(emu: &mut Emu, symbol: &str) {
    let call = UnixApi;
    let size = api_try!(emu, call.arg(emu, 1));
    let dma_handle = api_try!(emu, call.arg(emu, 3));
    if size == 0 || size > USB_COHERENT_MAX_SIZE {
        call.set_return(emu, 0);
        return;
    }
    api_try!(
        emu,
        memory::range(
            emu,
            dma_handle,
            size_of::<u64>() as u64,
            MemoryAccess::Write
        )
    );
    let buffer = emu.kernel_alloc(Region::Slab, size, "usb_coherent", symbol, true);
    if buffer != 0 {
        api_try!(emu, memory::write(emu, dma_handle, &buffer.to_le_bytes()));
    }
    call.set_return(emu, buffer);
}

fn usb_free_coherent(emu: &mut Emu, symbol: &str) {
    let call = UnixApi;
    // usb_free_coherent(dev, size, addr, dma) — buffer is arg2.
    let addr = api_try!(emu, call.arg(emu, 2));
    free(emu, addr, symbol);
}

fn usb_register_driver(emu: &mut Emu, _symbol: &str) {
    let call = UnixApi;
    let drv = api_try!(emu, call.arg(emu, 0));
    let probe = emu.kernel_register_driver("usb", drv);
    let d = emu.kernel_registered_drivers();
    let last = d.last();
    emu.kernel_log_line(format!(
        "usb_register_driver: struct {:#x} probe {:#x} ({}) id_table {:#x}",
        drv,
        probe,
        last.map(|r| r.probe_name.as_str()).unwrap_or(""),
        last.map(|r| r.id_table).unwrap_or(0),
    ));
    call.set_return(emu, 0);
}

fn usb_control_msg(emu: &mut Emu, _symbol: &str) {
    let call = UnixApi;
    let requesttype = api_try!(emu, call.arg(emu, 3));
    let addr = api_try!(emu, call.arg(emu, 4));
    let data = api_try!(emu, call.arg(emu, 6));
    let size = api_try!(emu, call.arg(emu, 7));
    let dir_in = requesttype & 0x80 != 0; // USB_DIR_IN
    let n = emu.kernel_usb_register_xfer(addr, data, size, dir_in);
    call.set_return(emu, n); // bytes transferred (>= 0 = success)
}

fn usb_control_msg_recv(emu: &mut Emu, _symbol: &str) {
    let call = UnixApi;
    let addr = api_try!(emu, call.arg(emu, 4));
    let buf = api_try!(emu, call.arg(emu, 6));
    let len = api_try!(emu, call.arg(emu, 7));
    emu.kernel_usb_register_xfer(addr, buf, len, true);
    call.set_return(emu, 0);
}

fn usb_control_msg_send(emu: &mut Emu, _symbol: &str) {
    let call = UnixApi;
    let addr = api_try!(emu, call.arg(emu, 4));
    let buf = api_try!(emu, call.arg(emu, 6));
    let len = api_try!(emu, call.arg(emu, 7));
    emu.kernel_usb_register_xfer(addr, buf, len, false);
    call.set_return(emu, 0);
}

fn is_netif_api(symbol: &str) -> bool {
    symbol.starts_with("netif_") || symbol.starts_with("__netif_")
}

fn is_ethtool_api(symbol: &str) -> bool {
    symbol.starts_with("ethtool_")
}
