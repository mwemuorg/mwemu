//! Driver registration and module bookkeeping.
//!
//! A driver's `init` function is mostly a sequence of "register me with
//! subsystem X" calls. None of those subsystems exist here, so the handlers
//! only have to succeed and say what was registered — that log is how an
//! analyst learns which entry points the module exposes and therefore which
//! functions are worth driving next.

use crate::api::abi::unix::ArgumentError;
use crate::api::abi::unix::UnixApi;
use crate::api::dispatch_api;
use crate::api::unix::api_try;
use crate::emu::Emu;
use crate::kernel::heap::Region;
use crate::kernel::{HandlerRegistry, kernel_handlers, return_one, return_zero};

/// Allocate an opaque handle for a subsystem object the driver will hold onto
/// (a `struct class`, a `cdev`, a procfs entry). Real pointers, so a driver
/// that dereferences one does not immediately fault.
fn handle(emu: &mut Emu, what: &str, api: &str) -> u64 {
    let call = UnixApi;
    let ptr = emu.kernel_alloc(Region::Slab, 0x100, what, api, true);
    call.set_return(emu, ptr);
    ptr
}

/// Capture a driver ops struct passed to a bus `*_register_driver` and log the
/// resolved probe. The struct is the first argument of every such helper.
fn capture(emu: &mut Emu, bus: &str, api: &str) -> Result<(), ArgumentError> {
    let call = UnixApi;
    let drv = call.arg(emu, 0)?;
    let probe = emu.kernel_register_driver(bus, drv);
    let drivers = emu.kernel_registered_drivers();
    let last = drivers.last();
    emu.kernel_log_line(format!(
        "{} {}: struct {:#x} probe {:#x} ({}) id_table {:#x}",
        api,
        bus,
        drv,
        probe,
        last.map(|r| r.probe_name.as_str()).unwrap_or(""),
        last.map(|r| r.id_table).unwrap_or(0),
    ));
    call.set_return(emu, 0);
    Ok(())
}

pub fn dispatch(symbol: &str, emu: &mut Emu) -> bool {
    dispatch_api!(symbol, emu, registry handlers(),
        handled(()) => true,
        fallback => false,
    )
}

pub const fn handlers() -> HandlerRegistry {
    kernel_handlers! {
        // --- character devices -------------------------------------------------
        "__register_chrdev" | "register_chrdev" => __register_chrdev,
        "__unregister_chrdev" | "unregister_chrdev" => return_zero,
        "alloc_chrdev_region" => alloc_chrdev_region,
        "register_chrdev_region" | "unregister_chrdev_region" => return_zero,
        "cdev_init" | "cdev_del" | "cdev_put" => return_zero,
        "cdev_add" | "cdev_device_add" => return_zero,
        "cdev_alloc" => cdev_alloc,

        // --- misc devices ------------------------------------------------------
        "misc_register" => misc_register,
        "misc_deregister" => return_zero,

        // --- sysfs / device model ----------------------------------------------
        "class_create" | "__class_create" | "class_register" => class_stub,
        "class_destroy" | "class_unregister" => return_zero,
        "device_create" | "device_create_with_groups" | "device_register" => device_stub,
        "device_destroy" | "device_unregister" | "put_device" | "get_device" => device_lifetime_stub,

        // --- procfs / debugfs / sysfs -------------------------------------------
        "proc_create" | "proc_create_data" | "proc_mkdir" | "proc_create_single_data" => proc_create,
        "remove_proc_entry" | "proc_remove" => return_zero,
        "debugfs_create_file" | "debugfs_create_dir" | "debugfs_create_u32" => debugfs_create_file,
        "debugfs_remove" | "debugfs_remove_recursive" => return_zero,
        "sysfs_create_file" | "sysfs_create_group" | "device_create_file" => return_zero,
        "sysfs_remove_file" | "sysfs_remove_group" | "device_remove_file" => return_zero,

        // --- bus driver registration --------------------------------------------
        // Every one of these takes a pointer to a per-bus driver ops struct as
        // its first argument, and that struct holds the driver's real `.probe`
        // and `id_table`. Capturing it here is what makes probe reachable after
        // init (see Emu::kernel_register_driver). Registration succeeds (0).
        "__pci_register_driver" | "pci_register_driver" => __pci_register_driver,
        "__sdio_register_driver" | "sdio_register_driver" => __sdio_register_driver,
        "platform_driver_register" | "__platform_driver_register" | "platform_driver_probe" => platform_driver_register,
        "spi_register_driver" | "__spi_register_driver" => spi_register_driver,
        "i2c_add_driver" | "i2c_register_driver" | "__i2c_add_driver" => i2c_add_driver,
        "__usb_serial_register_drivers" | "usb_serial_register_drivers" => __usb_serial_register_drivers,
        "mmc_register_driver" | "__mmc_register_driver" => mmc_register_driver,
        "register_virtio_driver" | "__register_virtio_driver" => register_virtio_driver,
        "pci_unregister_driver"
        | "sdio_unregister_driver"
        | "platform_driver_unregister"
        | "spi_unregister_driver"
        | "i2c_del_driver"
        | "usb_serial_deregister_drivers"
        | "mmc_unregister_driver"
        | "unregister_virtio_driver" => return_zero,

        // --- module refcounting --------------------------------------------------
        "try_module_get" => return_one,
        "module_put" | "__module_get" | "module_refcount" => return_zero,

        // --- seq_file (procfs readers) --------------------------------------------
        "seq_printf" | "seq_puts" | "seq_write" | "single_open" | "single_release" | "seq_read"
        | "seq_lseek" => return_zero,
    }
}

pub const SURFACE: &[&str] = handlers().names;

pub fn has_handler(symbol: &str) -> bool {
    handlers().find(symbol).is_some()
}

fn __register_chrdev(emu: &mut Emu, _symbol: &str) {
    let call = UnixApi;
    let major = api_try!(emu, call.arg(emu, 0));
    let name = emu.maps.read_string(api_try!(emu, call.arg(emu, 3)));
    emu.kernel_log_line(format!(
        "registered char device \"{}\" major {}",
        name, major
    ));
    call.set_return(emu, if major == 0 { 240 } else { 0 });
}

fn alloc_chrdev_region(emu: &mut Emu, _symbol: &str) {
    let call = UnixApi;
    let dev_out = api_try!(emu, call.arg(emu, 0));
    let name = emu.maps.read_string(api_try!(emu, call.arg(emu, 3)));
    // MKDEV(240, 0)
    emu.maps.write_dword(dev_out, 240 << 20);
    emu.kernel_log_line(format!("allocated chrdev region for \"{}\"", name));
    call.set_return(emu, 0);
}

fn cdev_alloc(emu: &mut Emu, symbol: &str) {
    handle(emu, "cdev", symbol);
}

fn misc_register(emu: &mut Emu, _symbol: &str) {
    let call = UnixApi;
    let dev = api_try!(emu, call.arg(emu, 0));
    // struct miscdevice starts with { int minor; const char *name; }
    let name_ptr = emu.maps.read_qword(dev + 8).unwrap_or(0);
    let name = if name_ptr != 0 {
        emu.maps.read_string(name_ptr)
    } else {
        String::new()
    };
    emu.kernel_log_line(format!("registered misc device \"{}\"", name));
    call.set_return(emu, 0);
}

fn class_stub(emu: &mut Emu, symbol: &str) {
    handle(emu, "class", symbol);
}

fn device_stub(emu: &mut Emu, symbol: &str) {
    handle(emu, "device", symbol);
}

fn device_lifetime_stub(emu: &mut Emu, _symbol: &str) {
    let call = UnixApi;
    call.set_return(emu, 0);
}

fn proc_create(emu: &mut Emu, symbol: &str) {
    let call = UnixApi;
    let name = emu.maps.read_string(api_try!(emu, call.arg(emu, 0)));
    emu.kernel_log_line(format!("created /proc/{}", name));
    handle(emu, "proc_dir_entry", symbol);
}

fn debugfs_create_file(emu: &mut Emu, symbol: &str) {
    let call = UnixApi;
    let name = emu.maps.read_string(api_try!(emu, call.arg(emu, 0)));
    emu.kernel_log_line(format!("created debugfs entry \"{}\"", name));
    handle(emu, "dentry", symbol);
}

fn __pci_register_driver(emu: &mut Emu, symbol: &str) {
    api_try!(emu, capture(emu, "pci", symbol));
}

fn __sdio_register_driver(emu: &mut Emu, symbol: &str) {
    api_try!(emu, capture(emu, "sdio", symbol));
}

fn platform_driver_register(emu: &mut Emu, symbol: &str) {
    api_try!(emu, capture(emu, "platform", symbol));
}

fn spi_register_driver(emu: &mut Emu, symbol: &str) {
    api_try!(emu, capture(emu, "spi", symbol));
}

fn i2c_add_driver(emu: &mut Emu, symbol: &str) {
    api_try!(emu, capture(emu, "i2c", symbol));
}

fn __usb_serial_register_drivers(emu: &mut Emu, symbol: &str) {
    api_try!(emu, capture(emu, "usb_serial", symbol));
}

fn mmc_register_driver(emu: &mut Emu, symbol: &str) {
    api_try!(emu, capture(emu, "mmc", symbol));
}

fn register_virtio_driver(emu: &mut Emu, symbol: &str) {
    api_try!(emu, capture(emu, "virtio", symbol));
}
