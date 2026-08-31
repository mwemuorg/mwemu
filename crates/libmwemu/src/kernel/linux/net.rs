//! Network-device and USB driver surface.
//!
//! Enough of the netdev / sk_buff / URB API to let a NIC or wifi driver's
//! `probe` run: the object-lifetime calls (allocate a `net_device`, an URB, an
//! `sk_buff`, and free them) funnel through the ledger so a double-free or
//! use-after-free on a probe/teardown error path is caught, while the rest
//! (carrier state, queue control, ethtool helpers) are benign no-ops.

use crate::emu::Emu;
use crate::kernel::heap::Region;

/// Space reserved before the driver's private area in a `net_device`
/// allocation. `netdev_priv()` is inlined in the driver as
/// `(char*)dev + ALIGN(sizeof(struct net_device), 32)`, a fixed offset baked
/// in at build time; reserving comfortably more than any real `net_device`
/// keeps that inlined access inside the chunk we hand back.
const NETDEV_RESERVE: u64 = 0x2000;

fn alloc(emu: &mut Emu, size: u64, cache: &str, api: &str, zeroed: bool) -> u64 {
    let ptr = emu.kernel_alloc(Region::Slab, size, cache, api, zeroed);
    emu.set_kernel_ret(ptr);
    ptr
}

fn free(emu: &mut Emu, ptr: u64, api: &str) {
    if ptr != 0 {
        emu.kernel_free(ptr, api);
    }
    emu.set_kernel_ret(0);
}

pub fn dispatch(symbol: &str, emu: &mut Emu) -> bool {
    match symbol {
        // --- net_device -------------------------------------------------------
        // alloc_etherdev_mqs(sizeof_priv, txqs, rxqs) and the generic
        // alloc_netdev_mqs(sizeof_priv, name, assign, setup, txqs, rxqs).
        "alloc_etherdev_mqs" | "alloc_etherdev_mq" | "alloc_etherdev" => {
            let priv_sz = emu.kernel_arg(0).min(0x10000);
            alloc(emu, NETDEV_RESERVE + priv_sz, "net_device", symbol, true);
        }
        "alloc_netdev_mqs" => {
            let priv_sz = emu.kernel_arg(0).min(0x10000);
            alloc(emu, NETDEV_RESERVE + priv_sz, "net_device", symbol, true);
        }
        "free_netdev" | "free_candev" => {
            let dev = emu.kernel_arg(0);
            free(emu, dev, symbol);
        }
        "register_netdev" | "register_netdevice" | "cfg80211_register_netdevice"
        | "unregister_netdev" | "unregister_netdevice" | "unregister_netdevice_queue"
        | "dev_addr_mod" | "eth_validate_addr" | "eth_mac_addr" | "eth_type_trans"
        | "ether_setup" | "eth_hw_addr_random" | "eth_commit_mac_addr_change" => {
            emu.set_kernel_ret(0)
        }
        // netif_* carrier / queue / rx — no scheduler here.
        s if s.starts_with("netif_") || s.starts_with("__netif_") => emu.set_kernel_ret(0),
        // ethtool helpers, netdev logging.
        s if s.starts_with("ethtool_") => emu.set_kernel_ret(0),
        "netdev_notice" | "netdev_info" | "netdev_warn" | "netdev_err" | "netdev_dbg"
        | "__dynamic_netdev_dbg" | "__dynamic_dev_dbg" | "netdev_rx_csum_fault" => {
            emu.set_kernel_ret(0)
        }

        // --- sk_buff ----------------------------------------------------------
        // __netdev_alloc_skb(dev, len, gfp) / napi_alloc_skb / __alloc_skb.
        // One chunk stands in for the sk_buff plus its linear data area; the
        // driver's skb_put/skb_pull walk pointers we return, not real state.
        "__netdev_alloc_skb" | "netdev_alloc_skb" | "napi_alloc_skb"
        | "__napi_alloc_skb" | "dev_alloc_skb" => {
            let len = emu.kernel_arg(1).min(0x10000);
            alloc(emu, 0x100 + len + 0x100, "skbuff_head_cache", symbol, true);
        }
        "__alloc_skb" | "alloc_skb" | "build_skb" => {
            let len = emu.kernel_arg(0).min(0x10000);
            alloc(emu, 0x100 + len + 0x100, "skbuff_head_cache", symbol, true);
        }
        "consume_skb" | "kfree_skb" | "kfree_skb_reason" | "dev_kfree_skb"
        | "dev_kfree_skb_any_reason" | "dev_kfree_skb_irq_reason"
        | "napi_consume_skb" | "__kfree_skb" => {
            let skb = emu.kernel_arg(0);
            free(emu, skb, symbol);
        }
        // skb data-pointer accessors: hand back the caller's skb pointer so the
        // driver has a non-NULL, in-bounds pointer to work with.
        "skb_put" | "skb_push" | "skb_pull" | "__skb_pull" | "skb_trim"
        | "__skb_put" | "skb_reserve" | "__skb_pad" | "skb_copy_bits" => {
            let skb = emu.kernel_arg(0);
            emu.set_kernel_ret(skb);
        }

        // --- USB --------------------------------------------------------------
        "usb_alloc_urb" => {
            // struct urb is ~0xc0 bytes on x86_64; round up.
            alloc(emu, 0x100, "urb", symbol, true);
        }
        "usb_free_urb" => {
            let urb = emu.kernel_arg(0);
            free(emu, urb, symbol);
        }
        "usb_alloc_coherent" => {
            let size = emu.kernel_arg(1).max(1).min(0x100000);
            alloc(emu, size, "usb_coherent", symbol, true);
        }
        "usb_free_coherent" => {
            // usb_free_coherent(dev, size, addr, dma) — buffer is arg2.
            let addr = emu.kernel_arg(2);
            free(emu, addr, symbol);
        }
        "usb_register_driver" | "usb_deregister" | "usb_submit_urb" | "usb_kill_urb"
        | "usb_unlink_urb" | "usb_control_msg" | "usb_control_msg_send"
        | "usb_control_msg_recv" | "usb_bulk_msg" | "usb_check_bulk_endpoints"
        | "usb_check_int_endpoints" | "usb_get_dev" | "usb_put_dev"
        | "usb_set_intfdata" | "usb_get_intfdata" | "usb_reset_device" => {
            emu.set_kernel_ret(0)
        }

        // --- tasklets / NAPI scheduling --------------------------------------
        "tasklet_setup" | "tasklet_init" | "tasklet_kill" | "__tasklet_schedule"
        | "__tasklet_hi_schedule" | "netif_napi_add" | "netif_napi_add_weight"
        | "__netif_napi_del" | "netif_napi_del" | "napi_enable" | "napi_disable"
        | "napi_complete_done" | "napi_schedule" | "__napi_schedule" => {
            emu.set_kernel_ret(0)
        }

        _ => return false,
    }
    true
}
