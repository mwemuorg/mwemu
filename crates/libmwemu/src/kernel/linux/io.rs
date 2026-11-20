//! MMIO / IRQ / resource acquisition.
//!
//! A driver's `probe` typically maps its registers and grabs an IRQ before it
//! does anything interesting; if those calls fail (NULL / negative) it bails
//! immediately down a short error path. To let probe reach the code that
//! actually allocates and frees — where the bugs are — the mapping calls hand
//! back a real, mapped MMIO window and the IRQ/resource calls report success.

use crate::api::abi::unix::UnixApi;
use crate::api::dispatch_api;
use crate::api::unix::api_try;
use crate::emu::Emu;
use crate::kernel::heap::Region;
use crate::kernel::{HandlerRegistry, kernel_handlers, return_zero};
use crate::maps::mem64::Permission;

/// A single shared MMIO window every `ioremap` hands back. One window is enough:
/// register reads/writes go here (or are absorbed by the lenient kernel MMU),
/// and the driver only needs a non-NULL, in-bounds base to proceed.
const MMIO_BASE: u64 = 0xffffc90001000000;
const MMIO_SIZE: u64 = 0x0010_0000;

fn mmio(emu: &mut Emu) -> u64 {
    if emu.maps.get_map_by_name("kernel.mmio").is_none() {
        let _ = emu
            .maps
            .create_map("kernel.mmio", MMIO_BASE, MMIO_SIZE, Permission::READ_WRITE);
    }
    MMIO_BASE
}

pub fn dispatch(symbol: &str, emu: &mut Emu) -> bool {
    dispatch_api!(symbol, emu, registry handlers(),
        handled(()) => true,
        fallback => false,
    )
}

pub const fn handlers() -> HandlerRegistry {
    kernel_handlers! {
        // --- register mapping: return a valid, mapped window ------------------
        "ioremap"
        | "ioremap_wc"
        | "ioremap_cache"
        | "ioremap_uc"
        | "ioremap_np"
        | "ioremap_wt"
        | "devm_ioremap"
        | "devm_ioremap_wc"
        | "devm_ioremap_nocache"
        | "devm_ioremap_resource"
        | "devm_ioremap_resource_wc"
        | "devm_platform_ioremap_resource"
        | "devm_platform_get_and_ioremap_resource"
        | "pci_iomap"
        | "pci_iomap_range"
        | "pcim_iomap"
        | "pci_ioremap_bar"
        | "of_iomap"
        | "ioport_map"
        | "devm_of_iomap" => mmio_mapping_stub,
        "iounmap" | "devm_iounmap" | "pci_iounmap" | "ioport_unmap" => return_zero,

        // --- resource lookup: hand back a small fake resource descriptor ------
        // struct resource { start; end; name; flags; ... } — a zeroed chunk is
        // enough for the inline resource_size()/->start reads that follow.
        "platform_get_resource"
        | "platform_get_resource_byname"
        | "pci_find_capability"
        | "of_get_property"
        | "of_find_property" => resource_stub,
        "platform_get_irq" | "platform_get_irq_byname" | "platform_get_irq_optional" => irq_stub,

        // --- IRQ / DMA: report success ---------------------------------------
        "request_irq"
        | "request_threaded_irq"
        | "devm_request_irq"
        | "devm_request_threaded_irq"
        | "pci_alloc_irq_vectors"
        | "pci_alloc_irq_vectors_affinity"
        | "request_any_context_irq"
        | "dma_set_mask"
        | "dma_set_coherent_mask"
        | "dma_set_mask_and_coherent"
        | "pci_enable_device"
        | "pcim_enable_device"
        | "pci_request_regions"
        | "pci_request_selected_regions"
        | "pci_enable_device_mem"
        | "dma_supported" => return_zero,
        "free_irq"
        | "devm_free_irq"
        | "pci_free_irq_vectors"
        | "pci_set_master"
        | "pci_clear_master"
        | "pci_release_regions"
        | "pci_disable_device"
        | "pci_irq_vector"
        | "synchronize_irq"
        | "disable_irq"
        | "enable_irq" => return_zero,

        // --- coherent DMA buffers: real allocations through the ledger --------
        "dma_alloc_coherent"
        | "dma_alloc_attrs"
        | "dmam_alloc_coherent"
        | "dma_alloc_noncoherent"
        | "dma_pool_alloc" => dma_alloc_coherent,
        "dma_free_coherent" | "dma_free_attrs" | "dmam_free_coherent" | "dma_pool_free" => dma_free_coherent,
        "dma_map_single" | "dma_map_page" | "dma_map_sg" | "dma_map_sg_attrs" => dma_map_single,
        "dma_unmap_single" | "dma_unmap_page" | "dma_unmap_sg" | "dma_unmap_sg_attrs"
        | "dma_mapping_error" => return_zero,
    }
}

pub const SURFACE: &[&str] = handlers().names;

pub fn has_handler(symbol: &str) -> bool {
    handlers().find(symbol).is_some()
}

fn mmio_mapping_stub(emu: &mut Emu, _symbol: &str) {
    let call = UnixApi;
    let base = mmio(emu);
    call.set_return(emu, base);
}

fn resource_stub(emu: &mut Emu, symbol: &str) {
    let call = UnixApi;
    let ptr = emu.kernel_alloc(Region::Slab, 0x40, "resource", symbol, true);
    call.set_return(emu, ptr);
}

fn irq_stub(emu: &mut Emu, _symbol: &str) {
    let call = UnixApi;
    call.set_return(emu, 16); // a plausible non-negative IRQ number
}

fn dma_alloc_coherent(emu: &mut Emu, symbol: &str) {
    let call = UnixApi;
    let size = api_try!(emu, call.arg(emu, 1)).clamp(1, 0x100000);
    let ptr = emu.kernel_alloc(Region::Vmalloc, size, "dma", symbol, true);
    // dma_alloc_coherent(dev, size, dma_handle*, gfp): write a bus addr.
    let handle_out = api_try!(emu, call.arg(emu, 2));
    if handle_out != 0 {
        emu.maps.write_qword(handle_out, ptr);
    }
    call.set_return(emu, ptr);
}

fn dma_free_coherent(emu: &mut Emu, symbol: &str) {
    let call = UnixApi;
    let ptr = api_try!(emu, call.arg(emu, 2)); // (dev, size, cpu_addr, dma_handle)
    if ptr != 0 {
        emu.kernel_free(ptr, symbol);
    }
    call.set_return(emu, 0);
}

fn dma_map_single(emu: &mut Emu, _symbol: &str) {
    let call = UnixApi;
    // return a non-zero "bus address" (reuse the cpu addr)
    let a = api_try!(emu, call.arg(emu, 1));
    call.set_return(emu, if a != 0 { a } else { MMIO_BASE });
}
