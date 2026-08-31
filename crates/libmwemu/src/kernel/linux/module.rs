//! Driver registration and module bookkeeping.
//!
//! A driver's `init` function is mostly a sequence of "register me with
//! subsystem X" calls. None of those subsystems exist here, so the handlers
//! only have to succeed and say what was registered — that log is how an
//! analyst learns which entry points the module exposes and therefore which
//! functions are worth driving next.

use crate::emu::Emu;
use crate::kernel::heap::Region;

/// Allocate an opaque handle for a subsystem object the driver will hold onto
/// (a `struct class`, a `cdev`, a procfs entry). Real pointers, so a driver
/// that dereferences one does not immediately fault.
fn handle(emu: &mut Emu, what: &str, api: &str) -> u64 {
    let ptr = emu.kernel_alloc(Region::Slab, 0x100, what, api, true);
    emu.set_kernel_ret(ptr);
    ptr
}

pub fn dispatch(symbol: &str, emu: &mut Emu) -> bool {
    match symbol {
        // --- character devices -------------------------------------------------
        "__register_chrdev" | "register_chrdev" => {
            let major = emu.kernel_arg(0);
            let name = emu.maps.read_string(emu.kernel_arg(3));
            emu.kernel_log_line(format!(
                "registered char device \"{}\" major {}",
                name, major
            ));
            emu.set_kernel_ret(if major == 0 { 240 } else { 0 });
        }
        "__unregister_chrdev" | "unregister_chrdev" => emu.set_kernel_ret(0),
        "alloc_chrdev_region" => {
            let dev_out = emu.kernel_arg(0);
            let name = emu.maps.read_string(emu.kernel_arg(3));
            // MKDEV(240, 0)
            emu.maps.write_dword(dev_out, 240 << 20);
            emu.kernel_log_line(format!("allocated chrdev region for \"{}\"", name));
            emu.set_kernel_ret(0);
        }
        "register_chrdev_region" | "unregister_chrdev_region" => emu.set_kernel_ret(0),
        "cdev_init" | "cdev_del" | "cdev_put" => emu.set_kernel_ret(0),
        "cdev_add" | "cdev_device_add" => emu.set_kernel_ret(0),
        "cdev_alloc" => {
            handle(emu, "cdev", symbol);
        }

        // --- misc devices ------------------------------------------------------
        "misc_register" => {
            let dev = emu.kernel_arg(0);
            // struct miscdevice starts with { int minor; const char *name; }
            let name_ptr = emu.maps.read_qword(dev + 8).unwrap_or(0);
            let name = if name_ptr != 0 {
                emu.maps.read_string(name_ptr)
            } else {
                String::new()
            };
            emu.kernel_log_line(format!("registered misc device \"{}\"", name));
            emu.set_kernel_ret(0);
        }
        "misc_deregister" => emu.set_kernel_ret(0),

        // --- sysfs / device model ----------------------------------------------
        "class_create" | "__class_create" | "class_register" => {
            handle(emu, "class", symbol);
        }
        "class_destroy" | "class_unregister" => emu.set_kernel_ret(0),
        "device_create" | "device_create_with_groups" | "device_register" => {
            handle(emu, "device", symbol);
        }
        "device_destroy" | "device_unregister" | "put_device" | "get_device" => {
            emu.set_kernel_ret(0)
        }

        // --- procfs / debugfs / sysfs -------------------------------------------
        "proc_create" | "proc_create_data" | "proc_mkdir" | "proc_create_single_data" => {
            let name = emu.maps.read_string(emu.kernel_arg(0));
            emu.kernel_log_line(format!("created /proc/{}", name));
            handle(emu, "proc_dir_entry", symbol);
        }
        "remove_proc_entry" | "proc_remove" => emu.set_kernel_ret(0),
        "debugfs_create_file" | "debugfs_create_dir" | "debugfs_create_u32" => {
            let name = emu.maps.read_string(emu.kernel_arg(0));
            emu.kernel_log_line(format!("created debugfs entry \"{}\"", name));
            handle(emu, "dentry", symbol);
        }
        "debugfs_remove" | "debugfs_remove_recursive" => emu.set_kernel_ret(0),
        "sysfs_create_file" | "sysfs_create_group" | "device_create_file" => emu.set_kernel_ret(0),
        "sysfs_remove_file" | "sysfs_remove_group" | "device_remove_file" => emu.set_kernel_ret(0),

        // --- module refcounting --------------------------------------------------
        "try_module_get" => emu.set_kernel_ret(1),
        "module_put" | "__module_get" | "module_refcount" => emu.set_kernel_ret(0),

        // --- seq_file (procfs readers) --------------------------------------------
        "seq_printf" | "seq_puts" | "seq_write" | "single_open" | "single_release" | "seq_read"
        | "seq_lseek" => emu.set_kernel_ret(0),

        _ => return false,
    }
    true
}
