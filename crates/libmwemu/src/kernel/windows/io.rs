//! The I/O manager: device objects, symbolic links, IRPs, MDLs, work items.
//!
//! `IoCreateDevice` is the moment a driver becomes reachable from user mode,
//! so the device is recorded (name, type, extension) and linked into the
//! driver object exactly as the real I/O manager does; the harness in
//! [`super::irp`] then delivers requests to it.

use super::structs::*;
use super::*;
use crate::kernel::DeferredCall;
use crate::kernel::heap::Region;

/// Size of the MDL for `length` bytes starting at `va`: header plus one PFN
/// per page touched.
fn mdl_size_for(va: u64, length: u64) -> u64 {
    let first = va & !0xfff;
    let last = (va + length.max(1) - 1) & !0xfff;
    let pages = (last - first) / 0x1000 + 1;
    MDL_SIZE + pages * 8
}

/// Build an MDL describing `[va, va + length)`. Pages are "locked" and mapped
/// at their own address, so `MmGetSystemAddressForMdlSafe` (inline in the
/// driver) finds `MappedSystemVa` set and never calls into the kernel.
pub fn build_mdl(emu: &mut Emu, mdl: u64, va: u64, length: u64) {
    let size = mdl_size_for(va, length);
    emu.maps.write_qword(mdl + MDL_NEXT, 0);
    emu.maps.write_word(mdl + MDL_SIZE_FIELD, size as u16);
    emu.maps.write_word(
        mdl + MDL_FLAGS,
        MDL_MAPPED_TO_SYSTEM_VA | MDL_PAGES_LOCKED | MDL_SOURCE_IS_NONPAGED_POOL,
    );
    emu.maps.write_qword(mdl + MDL_PROCESS, 0);
    emu.maps.write_qword(mdl + MDL_MAPPED_SYSTEM_VA, va);
    emu.maps.write_qword(mdl + MDL_START_VA, va & !0xfff);
    emu.maps.write_dword(mdl + MDL_BYTE_COUNT, length as u32);
    emu.maps
        .write_dword(mdl + MDL_BYTE_OFFSET, (va & 0xfff) as u32);
    // PFN array: identity "physical" pages.
    let pages = (size - MDL_SIZE) / 8;
    for i in 0..pages {
        emu.maps
            .write_qword(mdl + MDL_SIZE + i * 8, ((va & !0xfff) >> 12) + i);
    }
}

/// Allocate and initialise an MDL for `[va, va + length)`.
pub fn alloc_mdl(emu: &mut Emu, va: u64, length: u64, api: &str) -> u64 {
    let size = mdl_size_for(va, length);
    let mdl = emu.kernel_alloc(Region::Slab, size, "MDL", api, true);
    if mdl != 0 {
        build_mdl(emu, mdl, va, length);
    }
    mdl
}

/// Address of the `IO_STACK_LOCATION` the current driver sees.
pub fn current_stack_location(emu: &Emu, irp: u64) -> u64 {
    emu.maps
        .read_qword(irp + IRP_CURRENT_STACK_LOCATION)
        .unwrap_or(0)
}

/// Initialise an IRP the way `IoInitializeIrp` does: type, size, stack
/// count, and the current stack location one past the last slot (the
/// first `IoCallDriver` moves it onto the last slot).
pub fn init_irp(emu: &mut Emu, irp: u64, stack_size: u8) {
    let total = IRP_SIZE + stack_size as u64 * IO_STACK_LOCATION_SIZE;
    emu.maps.write_bytes(irp, &vec![0u8; total as usize]);
    emu.maps.write_word(irp + IRP_TYPE, IO_TYPE_IRP);
    emu.maps.write_word(irp + IRP_SIZE_FIELD, total as u16);
    emu.maps.write_byte(irp + IRP_STACK_COUNT, stack_size);
    emu.maps
        .write_byte(irp + IRP_CURRENT_LOCATION, stack_size.wrapping_add(1));
    emu.maps.write_qword(
        irp + IRP_CURRENT_STACK_LOCATION,
        irp + IRP_SIZE + stack_size as u64 * IO_STACK_LOCATION_SIZE,
    );
    emu.maps.write_byte(irp + IRP_REQUESTOR_MODE, 1);
}

fn device_name_of(emu: &Emu, dev: u64) -> String {
    emu.kernel
        .as_ref()
        .and_then(|k| k.win.devices.iter().find(|d| d.addr == dev))
        .map(|d| d.name.clone())
        .unwrap_or_default()
}

/// `IoCreateDevice` and `IoCreateDeviceSecure` share the first six
/// arguments; only the out-pointer position differs (the secure form takes
/// an SDDL string and a class GUID in between).
fn create_device(emu: &mut Emu, symbol: &str, out_index: usize) {
    let drv = emu.kernel_arg(0);
    let ext = emu.kernel_arg(1) as u32;
    let name_us = emu.kernel_arg(2);
    let dev_type = emu.kernel_arg(3) as u32;
    let chars = emu.kernel_arg(4) as u32;
    let exclusive = emu.kernel_arg(5) as u8;
    let out = emu.kernel_arg(out_index);
    let name = read_unicode_string(emu, name_us);
    if symbol == "IoCreateDeviceSecure" {
        let sddl = emu.maps.read_wide_string(emu.kernel_arg(6));
        log::info!("IoCreateDeviceSecure: SDDL '{}'", sddl);
    }
    let total = DEVICE_OBJECT_SIZE + (ext as u64).next_multiple_of(16);
    let dev = emu.kernel_alloc(Region::Slab, total, "DEVICE_OBJECT", symbol, true);
    if dev == 0 {
        emu.set_kernel_ret(STATUS_NO_MEMORY);
        return;
    }
    emu.maps.write_word(dev + DEVOBJ_TYPE, IO_TYPE_DEVICE);
    emu.maps.write_word(dev + DEVOBJ_SIZE, total as u16);
    emu.maps.write_dword(dev + DEVOBJ_REFERENCE_COUNT, 1);
    emu.maps.write_qword(dev + DEVOBJ_DRIVER_OBJECT, drv);
    let mut flags = DO_DEVICE_INITIALIZING;
    if exclusive != 0 {
        flags |= DO_EXCLUSIVE;
    }
    emu.maps.write_dword(dev + DEVOBJ_FLAGS, flags);
    emu.maps.write_dword(dev + DEVOBJ_CHARACTERISTICS, chars);
    if ext != 0 {
        emu.maps
            .write_qword(dev + DEVOBJ_DEVICE_EXTENSION, dev + DEVICE_OBJECT_SIZE);
    }
    emu.maps.write_dword(dev + DEVOBJ_DEVICE_TYPE, dev_type);
    emu.maps.write_byte(dev + DEVOBJ_STACK_SIZE, 1);
    emu.maps.write_dword(dev + DEVOBJ_ALIGNMENT_REQUIREMENT, 0);
    // Link at the head of the driver's device list.
    if drv != 0 {
        let next = emu.maps.read_qword(drv + DRVOBJ_DEVICE_OBJECT).unwrap_or(0);
        emu.maps.write_qword(dev + DEVOBJ_NEXT_DEVICE, next);
        emu.maps.write_qword(drv + DRVOBJ_DEVICE_OBJECT, dev);
    }
    write_out_qword(emu, out, dev);
    log::info!(
        "IoCreateDevice: '{}' type 0x{:x} ext {} -> 0x{:x}",
        name,
        dev_type,
        ext,
        dev
    );
    if let Some(k) = emu.kernel.as_mut() {
        k.win.devices.push(KernelDevice {
            addr: dev,
            name,
            device_type: dev_type,
            extension_size: ext,
        });
    }
    emu.set_kernel_ret(STATUS_SUCCESS);
}

pub fn dispatch(symbol: &str, emu: &mut Emu) -> bool {
    match symbol {
        // IoCreateDevice(DriverObject, DeviceExtensionSize, DeviceName,
        //                DeviceType, DeviceCharacteristics, Exclusive, *DeviceObject)
        "IoCreateDevice" => create_device(emu, symbol, 6),
        // IoCreateDeviceSecure(..., Exclusive, DefaultSDDLString, DeviceClassGuid, *DeviceObject)
        "IoCreateDeviceSecure" | "WdmlibIoCreateDeviceSecure" => create_device(emu, symbol, 8),
        "IoDeleteDevice" => {
            let dev = emu.kernel_arg(0);
            // Unlink from the driver's list, then return the object: a later
            // dereference of the device extension is a use-after-free.
            let drv = emu.maps.read_qword(dev + DEVOBJ_DRIVER_OBJECT).unwrap_or(0);
            if drv != 0 {
                let mut cur = emu.maps.read_qword(drv + DRVOBJ_DEVICE_OBJECT).unwrap_or(0);
                let next = emu.maps.read_qword(dev + DEVOBJ_NEXT_DEVICE).unwrap_or(0);
                if cur == dev {
                    emu.maps.write_qword(drv + DRVOBJ_DEVICE_OBJECT, next);
                } else {
                    let mut guard = 0;
                    while cur != 0 && guard < 64 {
                        let n = emu.maps.read_qword(cur + DEVOBJ_NEXT_DEVICE).unwrap_or(0);
                        if n == dev {
                            emu.maps.write_qword(cur + DEVOBJ_NEXT_DEVICE, next);
                            break;
                        }
                        cur = n;
                        guard += 1;
                    }
                }
            }
            let name = device_name_of(emu, dev);
            log::info!("IoDeleteDevice: '{}' 0x{:x}", name, dev);
            if let Some(k) = emu.kernel.as_mut() {
                k.win.devices.retain(|d| d.addr != dev);
            }
            emu.kernel_free(dev, symbol);
            emu.set_kernel_ret(0);
        }
        "IoCreateSymbolicLink" | "IoCreateUnprotectedSymbolicLink" => {
            let link = read_unicode_string(emu, emu.kernel_arg(0));
            let target = read_unicode_string(emu, emu.kernel_arg(1));
            log::info!("{}: {} -> {}", symbol, link, target);
            if let Some(k) = emu.kernel.as_mut() {
                k.win.symlinks.push((link, target));
            }
            emu.set_kernel_ret(STATUS_SUCCESS);
        }
        "IoDeleteSymbolicLink" => {
            let link = read_unicode_string(emu, emu.kernel_arg(0));
            if let Some(k) = emu.kernel.as_mut() {
                k.win.symlinks.retain(|(l, _)| *l != link);
            }
            emu.set_kernel_ret(STATUS_SUCCESS);
        }
        // IoGetDeviceObjectPointer(ObjectName, DesiredAccess, *FileObject, *DeviceObject)
        "IoGetDeviceObjectPointer" => {
            let name = read_unicode_string(emu, emu.kernel_arg(0));
            let file_out = emu.kernel_arg(2);
            let dev_out = emu.kernel_arg(3);
            let dev = emu
                .kernel
                .as_ref()
                .and_then(|k| {
                    k.win
                        .devices
                        .iter()
                        .find(|d| d.name.eq_ignore_ascii_case(&name))
                        .map(|d| d.addr)
                })
                .unwrap_or(0);
            if dev == 0 {
                log::info!("IoGetDeviceObjectPointer: '{}' not found", name);
                emu.set_kernel_ret(STATUS_OBJECT_NAME_NOT_FOUND);
            } else {
                let file = irp::ensure_file_object(emu, dev);
                write_out_qword(emu, file_out, file);
                write_out_qword(emu, dev_out, dev);
                emu.set_kernel_ret(STATUS_SUCCESS);
            }
        }
        "IoAttachDeviceToDeviceStack"
        | "IoGetAttachedDeviceReference"
        | "IoGetAttachedDevice"
        | "IoGetRelatedDeviceObject"
        | "IoGetLowerDeviceObject"
        | "IoGetDeviceAttachmentBaseRef" => {
            // Single-device stacks: the target is the device itself.
            let idx = if symbol == "IoAttachDeviceToDeviceStack" {
                1
            } else {
                0
            };
            let target = emu.kernel_arg(idx);
            emu.set_kernel_ret(target);
        }
        "IoAttachDeviceToDeviceStackSafe" => {
            let target = emu.kernel_arg(1);
            let out = emu.kernel_arg(2);
            write_out_qword(emu, out, target);
            emu.set_kernel_ret(STATUS_SUCCESS);
        }
        "IoAttachDevice" => {
            let out = emu.kernel_arg(2);
            let dev = emu.kernel_arg(0);
            write_out_qword(emu, out, dev);
            emu.set_kernel_ret(STATUS_SUCCESS);
        }
        "IoDetachDevice"
        | "IoStartPacket"
        | "IoStartNextPacket"
        | "IoStartNextPacketByKey"
        | "IoSetStartIoAttributes"
        | "IoStartTimer"
        | "IoStopTimer"
        | "IoSetShareAccess"
        | "IoUpdateShareAccess"
        | "IoRemoveShareAccess"
        | "IoReuseIrp"
        | "IoSetTopLevelIrp"
        | "IoInvalidateDeviceRelations"
        | "IoInvalidateDeviceState"
        | "IoRequestDeviceEject"
        | "IoSetHardErrorOrVerifyDevice"
        | "IoReleaseVpbSpinLock"
        | "IoAcquireVpbSpinLock"
        | "IoCancelFileOpen"
        | "IoRegisterFileSystem"
        | "IoUnregisterFileSystem"
        | "IoRegisterFsRegistrationChange"
        | "IoUnregisterFsRegistrationChange"
        | "IoSetDeviceToVerify"
        | "IoWriteErrorLogEntry"
        | "IoUnregisterShutdownNotification"
        | "IoUnregisterPlugPlayNotification"
        | "IoUnregisterPlugPlayNotificationEx"
        | "IoDisconnectInterrupt"
        | "IoDisconnectInterruptEx"
        | "IoFreeController"
        | "IoDeleteController"
        | "IoFreeErrorLogEntry"
        | "IoCancelIrp"
        | "IoReleaseCancelSpinLock"
        | "IoReleaseRemoveLockEx"
        | "IoReleaseRemoveLockAndWaitEx"
        | "IoInitializeRemoveLockEx"
        | "IoSetCompletionRoutineEx"
        | "IoFreeIrpUndo"
        | "IoRaiseInformationalHardError"
        | "IoRaiseHardError"
        | "IoUninitializeWorkItem"
        | "IoFlushAdapterBuffers"
        | "IoFreeAdapterChannel"
        | "IoFreeMapRegisters"
        | "IoSetSystemPartition" => {
            emu.set_kernel_ret(0);
        }
        "IoAcquireRemoveLockEx"
        | "IoRegisterShutdownNotification"
        | "IoRegisterLastChanceShutdownNotification"
        | "IoRegisterDeviceInterface"
        | "IoSetDeviceInterfaceState"
        | "IoRegisterPlugPlayNotification"
        | "IoWMIRegistrationControl"
        | "IoWMIWriteEvent"
        | "IoWMISuggestInstanceName"
        | "IoReportTargetDeviceChange"
        | "IoReportTargetDeviceChangeAsynchronous"
        | "IoReportResourceUsage"
        | "IoReportResourceForDetection"
        | "IoReportDetectedDevice"
        | "IoAssignResources"
        | "IoCreateDriver"
        | "IoCsqInitialize"
        | "IoCsqInitializeEx"
        | "IoSynchronousInvalidateDeviceRelations"
        | "IoSetDevicePropertyData"
        | "IoRegisterBootDriverCallback"
        | "IoVerifyPartitionTable"
        | "IoReadPartitionTable"
        | "IoReadPartitionTableEx"
        | "IoSetPartitionInformation"
        | "IoWritePartitionTable"
        | "IoCheckShareAccess"
        | "IoCheckShareAccessEx"
        | "IoValidateDeviceIoControlAccess"
        | "IoRegisterContainerNotification"
        | "IoReplacePartitionUnit"
        | "IoSetIoCompletion"
        | "IoWMIAllocateInstanceIds"
        | "IoWMIDeviceObjectToInstanceName"
        | "IoWMIOpenBlock"
        | "IoWMIQueryAllData"
        | "IoWMIQuerySingleInstance"
        | "IoWMISetNotificationCallback"
        | "IoWMIHandleToInstanceName"
        | "IoSetFileOrigin"
        | "IoSetInformation"
        | "IoCheckEaBufferValidity"
        | "IoCheckQuotaBufferValidity"
        | "IoCheckQuerySetFileInformation"
        | "IoCheckQuerySetVolumeInformation"
        | "IoVerifyVolume"
        | "IoRegisterBootDriverReinitialization"
        | "IoRegisterDriverReinitialization"
        | "IoFlushAdapterBuffersEx"
        | "IoEnumerateDeviceObjectList"
        | "IoGetDeviceInterfaces"
        | "IoGetDeviceInterfaceAlias"
        | "IoGetDeviceNumaNode"
        | "IoGetIoPriorityHint"
        | "IoSetIoPriorityHint" => {
            // Modelled as a plain success; reinit routines are queued like
            // work so the harness can drain them.
            if symbol == "IoRegisterBootDriverReinitialization"
                || symbol == "IoRegisterDriverReinitialization"
            {
                let drv = emu.kernel_arg(0);
                let func = emu.kernel_arg(1);
                let ctx = emu.kernel_arg(2);
                emu.kernel_defer(DeferredCall {
                    kind: "reinit".to_string(),
                    func,
                    args: vec![drv, ctx, 1],
                });
            }
            emu.set_kernel_ret(STATUS_SUCCESS);
        }
        "IoConnectInterrupt" | "IoConnectInterruptEx" => {
            let out = emu.kernel_arg(0);
            let h = new_handle(emu, "interrupt".to_string());
            write_out_qword(emu, out, h);
            emu.set_kernel_ret(STATUS_SUCCESS);
        }
        "IoIsWdmVersionAvailable"
        | "IoIs32bitProcess"
        | "IoIsOperationSynchronous"
        | "IoIsSystemThread"
        | "IoIsValidNameGraftingBuffer"
        | "IoIsFileOriginRemote" => {
            let v = if symbol == "IoIsWdmVersionAvailable" || symbol == "IoIsOperationSynchronous" {
                1
            } else {
                0
            };
            emu.set_kernel_ret(v);
        }
        // IofCompleteRequest(Irp, PriorityBoost): the request is done; what the
        // driver wrote into IoStatus is what the harness reads back.
        "IofCompleteRequest" | "IoCompleteRequest" => {
            let irp = emu.kernel_arg(0);
            let status = emu.maps.read_dword(irp + IRP_IOSTATUS_STATUS).unwrap_or(0);
            let info = emu
                .maps
                .read_qword(irp + IRP_IOSTATUS_INFORMATION)
                .unwrap_or(0);
            log::info!(
                "IofCompleteRequest: irp 0x{:x} status 0x{:x} information 0x{:x}",
                irp,
                status,
                info
            );
            // An IRP the driver allocated itself and completes is released
            // through IoFreeIrp by its completion routine or owner; the harness
            // frees its own. Flag the completion so a second completion of the
            // same IRP is visible.
            if let Some(k) = emu.kernel.as_mut() {
                if k.win.completed_irps.contains(&irp) {
                    log::warn!("IofCompleteRequest: irp 0x{:x} completed twice", irp);
                }
                k.win.completed_irps.push(irp);
            }
            emu.set_kernel_ret(0);
        }
        "IoGetCurrentIrpStackLocation" => {
            let irp = emu.kernel_arg(0);
            let sl = current_stack_location(emu, irp);
            emu.set_kernel_ret(sl);
        }
        "IoGetNextIrpStackLocation" => {
            let irp = emu.kernel_arg(0);
            let sl = current_stack_location(emu, irp);
            emu.set_kernel_ret(sl.wrapping_sub(IO_STACK_LOCATION_SIZE));
        }
        "IoSkipCurrentIrpStackLocation" => {
            let irp = emu.kernel_arg(0);
            let sl = current_stack_location(emu, irp);
            emu.maps.write_qword(
                irp + IRP_CURRENT_STACK_LOCATION,
                sl + IO_STACK_LOCATION_SIZE,
            );
            let loc = emu.maps.read_byte(irp + IRP_CURRENT_LOCATION).unwrap_or(0);
            emu.maps
                .write_byte(irp + IRP_CURRENT_LOCATION, loc.wrapping_add(1));
            emu.set_kernel_ret(0);
        }
        "IoCopyCurrentIrpStackLocationToNext" => {
            let irp = emu.kernel_arg(0);
            let sl = current_stack_location(emu, irp);
            let next = sl.wrapping_sub(IO_STACK_LOCATION_SIZE);
            emu.maps
                .memcpy(next, sl, (IOSL_COMPLETION_ROUTINE) as usize);
            emu.maps.write_byte(next + IOSL_CONTROL, 0);
            emu.set_kernel_ret(0);
        }
        // IoAllocateIrp(StackSize, ChargeQuota)
        "IoAllocateIrp" | "IoAllocateIrpEx" => {
            let stack = emu.kernel_arg(0) as u8;
            let total = IRP_SIZE + stack as u64 * IO_STACK_LOCATION_SIZE;
            let irp = emu.kernel_alloc(Region::Slab, total, "IRP", symbol, true);
            if irp != 0 {
                init_irp(emu, irp, stack);
            }
            emu.set_kernel_ret(irp);
        }
        "IoInitializeIrp" => {
            let irp = emu.kernel_arg(0);
            let stack = emu.kernel_arg(2) as u8;
            init_irp(emu, irp, stack);
            emu.set_kernel_ret(0);
        }
        "IoFreeIrp" => {
            let irp = emu.kernel_arg(0);
            emu.kernel_free(irp, symbol);
            emu.set_kernel_ret(0);
        }
        "IoMakeAssociatedIrp" => {
            let stack = emu.kernel_arg(1) as u8;
            let total = IRP_SIZE + stack as u64 * IO_STACK_LOCATION_SIZE;
            let irp = emu.kernel_alloc(Region::Slab, total, "IRP", symbol, true);
            if irp != 0 {
                init_irp(emu, irp, stack);
            }
            emu.set_kernel_ret(irp);
        }
        // IoBuildDeviceIoControlRequest(IoControlCode, DeviceObject, InputBuffer,
        //   InputBufferLength, OutputBuffer, OutputBufferLength, InternalDeviceIoControl,
        //   Event, IoStatusBlock)
        "IoBuildDeviceIoControlRequest" => {
            let code = emu.kernel_arg(0) as u32;
            let dev = emu.kernel_arg(1);
            let input = emu.kernel_arg(2);
            let in_len = emu.kernel_arg(3) as u32;
            let output = emu.kernel_arg(4);
            let out_len = emu.kernel_arg(5) as u32;
            let internal = emu.kernel_arg(6) as u8;
            let iosb = emu.kernel_arg(8);
            let stack = emu
                .maps
                .read_byte(dev + DEVOBJ_STACK_SIZE)
                .unwrap_or(1)
                .max(1);
            let total = IRP_SIZE + stack as u64 * IO_STACK_LOCATION_SIZE;
            let irp = emu.kernel_alloc(Region::Slab, total, "IRP", symbol, true);
            if irp != 0 {
                init_irp(emu, irp, stack);
                // The next stack location is the one the target will see.
                let sl = irp + IRP_SIZE + (stack as u64 - 1) * IO_STACK_LOCATION_SIZE;
                let major = if internal != 0 {
                    IRP_MJ_INTERNAL_DEVICE_CONTROL
                } else {
                    IRP_MJ_DEVICE_CONTROL
                };
                emu.maps.write_byte(sl + IOSL_MAJOR_FUNCTION, major);
                emu.maps.write_dword(sl + IOSL_IOCTL_CODE, code);
                emu.maps.write_dword(sl + IOSL_IOCTL_INPUT_LENGTH, in_len);
                emu.maps.write_dword(sl + IOSL_IOCTL_OUTPUT_LENGTH, out_len);
                emu.maps.write_qword(sl + IOSL_DEVICE_OBJECT, dev);
                emu.maps.write_qword(irp + IRP_USER_IOSB, iosb);
                match code & 3 {
                    METHOD_BUFFERED => {
                        let n = in_len.max(out_len) as u64;
                        let sys =
                            emu.kernel_alloc(Region::Slab, n.max(1), "SystemBuffer", symbol, true);
                        if sys != 0 && input != 0 && in_len > 0 {
                            emu.maps.memcpy(sys, input, in_len as usize);
                        }
                        emu.maps.write_qword(irp + IRP_SYSTEM_BUFFER, sys);
                        emu.maps.write_qword(irp + IRP_USER_BUFFER, output);
                        emu.maps
                            .write_dword(irp + IRP_FLAGS, IRP_BUFFERED_IO | IRP_DEALLOCATE_BUFFER);
                    }
                    METHOD_IN_DIRECT | METHOD_OUT_DIRECT => {
                        if input != 0 && in_len > 0 {
                            let sys = emu.kernel_alloc(
                                Region::Slab,
                                in_len as u64,
                                "SystemBuffer",
                                symbol,
                                true,
                            );
                            if sys != 0 {
                                emu.maps.memcpy(sys, input, in_len as usize);
                            }
                            emu.maps.write_qword(irp + IRP_SYSTEM_BUFFER, sys);
                        }
                        if output != 0 && out_len > 0 {
                            let mdl = alloc_mdl(emu, output, out_len as u64, symbol);
                            emu.maps.write_qword(irp + IRP_MDL_ADDRESS, mdl);
                        }
                    }
                    _ => {
                        emu.maps.write_qword(sl + IOSL_IOCTL_TYPE3_INPUT, input);
                        emu.maps.write_qword(irp + IRP_USER_BUFFER, output);
                    }
                }
            }
            emu.set_kernel_ret(irp);
        }
        // IoBuildSynchronousFsdRequest(MajorFunction, DeviceObject, Buffer, Length,
        //   StartingOffset, Event, IoStatusBlock) / IoBuildAsynchronousFsdRequest
        "IoBuildSynchronousFsdRequest" | "IoBuildAsynchronousFsdRequest" => {
            let major = emu.kernel_arg(0) as u8;
            let dev = emu.kernel_arg(1);
            let buffer = emu.kernel_arg(2);
            let length = emu.kernel_arg(3) as u32;
            let offset_ptr = emu.kernel_arg(4);
            let stack = emu
                .maps
                .read_byte(dev + DEVOBJ_STACK_SIZE)
                .unwrap_or(1)
                .max(1);
            let total = IRP_SIZE + stack as u64 * IO_STACK_LOCATION_SIZE;
            let irp = emu.kernel_alloc(Region::Slab, total, "IRP", symbol, true);
            if irp != 0 {
                init_irp(emu, irp, stack);
                let sl = irp + IRP_SIZE + (stack as u64 - 1) * IO_STACK_LOCATION_SIZE;
                emu.maps.write_byte(sl + IOSL_MAJOR_FUNCTION, major);
                emu.maps.write_dword(sl + IOSL_RW_LENGTH, length);
                if offset_ptr != 0 {
                    let off = emu.maps.read_qword(offset_ptr).unwrap_or(0);
                    emu.maps.write_qword(sl + IOSL_RW_BYTE_OFFSET, off);
                }
                emu.maps.write_qword(sl + IOSL_DEVICE_OBJECT, dev);
                emu.maps.write_qword(irp + IRP_USER_BUFFER, buffer);
                if buffer != 0 && length > 0 {
                    let mdl = alloc_mdl(emu, buffer, length as u64, symbol);
                    emu.maps.write_qword(irp + IRP_MDL_ADDRESS, mdl);
                }
            }
            emu.set_kernel_ret(irp);
        }
        // IofCallDriver(DeviceObject, Irp): the request goes down one level.
        // There is only one driver in this kernel, so the next device is a
        // modelled lower driver that completes the request successfully. The
        // stack location is still advanced so a driver that reads the IRP
        // afterwards sees what it would see on a real stack.
        "IofCallDriver" | "IoCallDriver" | "PoCallDriver" | "PoCallDriverEx" => {
            let irp = emu.kernel_arg(1);
            let sl = current_stack_location(emu, irp);
            let next = sl.wrapping_sub(IO_STACK_LOCATION_SIZE);
            emu.maps.write_qword(irp + IRP_CURRENT_STACK_LOCATION, next);
            let loc = emu.maps.read_byte(irp + IRP_CURRENT_LOCATION).unwrap_or(1);
            emu.maps
                .write_byte(irp + IRP_CURRENT_LOCATION, loc.wrapping_sub(1));
            emu.maps.write_dword(irp + IRP_IOSTATUS_STATUS, 0);
            let iosb = emu.maps.read_qword(irp + IRP_USER_IOSB).unwrap_or(0);
            if iosb != 0 {
                emu.maps.write_dword(iosb, 0);
                emu.maps.write_qword(iosb + 8, 0);
            }
            // Signal the caller's event so a KeWaitForSingleObject after
            // STATUS_PENDING returns.
            let event = emu.maps.read_qword(irp + IRP_USER_EVENT).unwrap_or(0);
            if event != 0 {
                emu.maps.write_dword(event + DISPATCHER_SIGNAL_STATE, 1);
            }
            log::info!("{}: irp 0x{:x} forwarded to lower device", symbol, irp);
            emu.set_kernel_ret(STATUS_SUCCESS);
        }
        "IoForwardIrpSynchronously" | "IoForwardAndCatchIrp" => {
            emu.set_kernel_ret(1);
        }
        "IoSetCancelRoutine" => {
            let irp = emu.kernel_arg(0);
            let routine = emu.kernel_arg(1);
            let old = emu.maps.read_qword(irp + IRP_CANCEL_ROUTINE).unwrap_or(0);
            emu.maps.write_qword(irp + IRP_CANCEL_ROUTINE, routine);
            emu.set_kernel_ret(old);
        }
        "IoAcquireCancelSpinLock" => {
            let out = emu.kernel_arg(0);
            if out != 0 {
                emu.maps.write_byte(out, 0);
            }
            emu.set_kernel_ret(0);
        }

        // --- MDL ----------------------------------------------------------------
        // IoAllocateMdl(VirtualAddress, Length, SecondaryBuffer, ChargeQuota, Irp)
        "IoAllocateMdl" => {
            let va = emu.kernel_arg(0);
            let len = emu.kernel_arg(1) as u32 as u64;
            let irp = emu.kernel_arg(4);
            let mdl = alloc_mdl(emu, va, len, symbol);
            if irp != 0 && mdl != 0 {
                emu.maps.write_qword(irp + IRP_MDL_ADDRESS, mdl);
            }
            emu.set_kernel_ret(mdl);
        }
        "IoFreeMdl" => {
            let mdl = emu.kernel_arg(0);
            emu.kernel_free(mdl, symbol);
            emu.set_kernel_ret(0);
        }
        // IoBuildPartialMdl(SourceMdl, TargetMdl, VirtualAddress, Length)
        "IoBuildPartialMdl" => {
            let target = emu.kernel_arg(1);
            let va = emu.kernel_arg(2);
            let mut len = emu.kernel_arg(3) as u32 as u64;
            if len == 0 {
                let src = emu.kernel_arg(0);
                let count = emu.maps.read_dword(src + MDL_BYTE_COUNT).unwrap_or(0) as u64;
                let start = emu
                    .maps
                    .read_qword(src + MDL_MAPPED_SYSTEM_VA)
                    .unwrap_or(va);
                len = count.saturating_sub(va.saturating_sub(start));
            }
            build_mdl(emu, target, va, len);
            emu.set_kernel_ret(0);
        }

        // --- work items ---------------------------------------------------------
        "IoSizeofWorkItem" => emu.set_kernel_ret(IO_WORKITEM_SIZE),
        "IoAllocateWorkItem" => {
            let dev = emu.kernel_arg(0);
            let item =
                emu.kernel_alloc(Region::Slab, IO_WORKITEM_SIZE, "IO_WORKITEM", symbol, true);
            if item != 0 {
                emu.maps.write_qword(item + IO_WORKITEM_DEVICE, dev);
            }
            emu.set_kernel_ret(item);
        }
        "IoInitializeWorkItem" => {
            let dev = emu.kernel_arg(0);
            let item = emu.kernel_arg(1);
            emu.maps.write_qword(item + IO_WORKITEM_DEVICE, dev);
            emu.set_kernel_ret(0);
        }
        "IoFreeWorkItem" => {
            let item = emu.kernel_arg(0);
            emu.kernel_free(item, symbol);
            emu.set_kernel_ret(0);
        }
        // IoQueueWorkItem(IoWorkItem, WorkerRoutine, QueueType, Context):
        // WorkerRoutine(DeviceObject, Context). The Ex form gets
        // (IoObject, Context, IoWorkItem).
        "IoQueueWorkItem" | "IoQueueWorkItemEx" => {
            let item = emu.kernel_arg(0);
            let routine = emu.kernel_arg(1);
            let ctx = emu.kernel_arg(3);
            let dev = emu.maps.read_qword(item + IO_WORKITEM_DEVICE).unwrap_or(0);
            let args = if symbol == "IoQueueWorkItemEx" {
                vec![dev, ctx, item]
            } else {
                vec![dev, ctx]
            };
            emu.kernel_defer(DeferredCall {
                kind: "io_workitem".to_string(),
                func: routine,
                args,
            });
            emu.set_kernel_ret(0);
        }
        // ExQueueWorkItem(WorkItem, QueueType): WorkerRoutine(Parameter).
        "ExQueueWorkItem" => {
            let item = emu.kernel_arg(0);
            let routine = emu
                .maps
                .read_qword(item + WORK_QUEUE_ITEM_ROUTINE)
                .unwrap_or(0);
            let param = emu
                .maps
                .read_qword(item + WORK_QUEUE_ITEM_PARAMETER)
                .unwrap_or(0);
            emu.kernel_defer(DeferredCall {
                kind: "ex_workitem".to_string(),
                func: routine,
                args: vec![param],
            });
            emu.set_kernel_ret(0);
        }

        // --- misc -----------------------------------------------------------------
        "IoGetCurrentProcess" | "IoThreadToProcess" | "IoGetRequestorProcess" => {
            let p = emu
                .kernel
                .as_ref()
                .map(|k| k.win.system_process)
                .unwrap_or(0);
            emu.set_kernel_ret(p);
        }
        "IoGetRequestorProcessId" | "IoGetRequestorSessionId" => emu.set_kernel_ret(4),
        "IoGetStackLimits" => {
            let low = emu.kernel_arg(0);
            let high = emu.kernel_arg(1);
            let (base, size) = emu
                .kernel
                .as_ref()
                .map(|k| (k.layout.stack_base, k.layout.stack_size))
                .unwrap_or((0, 0));
            write_out_qword(emu, low, base);
            write_out_qword(emu, high, base + size);
            emu.set_kernel_ret(0);
        }
        "IoGetInitialStack" => {
            let top = emu
                .kernel
                .as_ref()
                .map(|k| k.layout.stack_base + k.layout.stack_size)
                .unwrap_or(0);
            emu.set_kernel_ret(top);
        }
        "IoGetConfigurationInformation" => {
            let p = emu
                .kernel_data_for("win.configuration_information", 0x100)
                .unwrap_or(0);
            emu.set_kernel_ret(p);
        }
        "IoAllocateErrorLogEntry" => emu.set_kernel_ret(0),
        "IoCreateNotificationEvent" | "IoCreateSynchronizationEvent" => {
            let out = emu.kernel_arg(1);
            let ev = emu.kernel_alloc(Region::Slab, KEVENT_SIZE, "KEVENT", symbol, true);
            if ev != 0 {
                let t = if symbol == "IoCreateNotificationEvent" {
                    NOTIFICATION_EVENT
                } else {
                    SYNCHRONIZATION_EVENT
                };
                emu.maps.write_byte(ev + DISPATCHER_TYPE, t);
                emu.maps.write_dword(ev + DISPATCHER_SIGNAL_STATE, 1);
            }
            let h = new_handle(emu, format!("event 0x{:x}", ev));
            write_out_qword(emu, out, h);
            emu.set_kernel_ret(ev);
        }
        // IoCreateFile / IoCreateFileEx: a handle is enough.
        "IoCreateFile" | "IoCreateFileEx" | "IoCreateFileSpecifyDeviceObjectHint" => {
            let out = emu.kernel_arg(0);
            let attrs = emu.kernel_arg(2);
            let name = read_unicode_string(
                emu,
                emu.maps
                    .read_qword(attrs + OBJATTR_OBJECT_NAME)
                    .unwrap_or(0),
            );
            let iosb = emu.kernel_arg(3);
            log::info!("{}: '{}'", symbol, name);
            let h = new_handle(emu, format!("file {}", name));
            write_out_qword(emu, out, h);
            if iosb != 0 {
                emu.maps.write_dword(iosb, 0);
                emu.maps.write_qword(iosb + 8, 1);
            }
            emu.set_kernel_ret(STATUS_SUCCESS);
        }
        "IoOpenDeviceRegistryKey"
        | "IoOpenDeviceInterfaceRegistryKey"
        | "IoOpenDriverRegistryKey" => {
            let out = emu.kernel_arg(3);
            let h = new_handle(emu, "device registry key".to_string());
            write_out_qword(emu, out, h);
            emu.set_kernel_ret(STATUS_SUCCESS);
        }
        "IoGetDeviceProperty" | "IoGetDevicePropertyData" => {
            let ret_len = emu.kernel_arg(3);
            write_out_dword(emu, ret_len, 0);
            emu.set_kernel_ret(STATUS_BUFFER_TOO_SMALL);
        }
        "IoQueryFileDosDeviceName"
        | "IoVolumeDeviceToDosName"
        | "IoQueryFileInformation"
        | "IoQueryVolumeInformation"
        | "IoGetDeviceObjectPointerEx"
        | "IoGetBootDiskInformation"
        | "IoGetDmaAdapter"
        | "IoGetAffinityInterrupt"
        | "IoGetDriverObjectExtension"
        | "IoAllocateDriverObjectExtension"
        | "IoGetDeviceInterfacesEx"
        | "IoCreateController"
        | "IoGetDiskDeviceObject"
        | "IoGetPagingIoPriority"
        | "IoGetTopLevelIrp"
        | "IoGetDeviceToVerify"
        | "IoGetFileObjectGenericMapping"
        | "IoGetFileObjectFilterContext"
        | "IoGetTransactionParameterBlock"
        | "IoGetBaseFileSystemDeviceObject"
        | "IoGetSiloParameters"
        | "IoGetContainerInformation" => {
            if symbol == "IoAllocateDriverObjectExtension" {
                let size = emu.kernel_arg(2);
                let out = emu.kernel_arg(3);
                let p = emu.kernel_alloc(Region::Slab, size, "DriverObjectExtension", symbol, true);
                write_out_qword(emu, out, p);
                emu.set_kernel_ret(STATUS_SUCCESS);
            } else if symbol == "IoGetFileObjectGenericMapping" {
                let p = emu
                    .kernel_data_for("win.file_generic_mapping", 0x10)
                    .unwrap_or(0);
                emu.set_kernel_ret(p);
            } else {
                emu.set_kernel_ret(
                    if symbol.starts_with("IoQuery")
                        || symbol.starts_with("IoVolume")
                        || symbol == "IoGetBootDiskInformation"
                    {
                        STATUS_NOT_SUPPORTED
                    } else {
                        0
                    },
                );
            }
        }
        "IopInvalidDeviceRequest" => {
            // Default MajorFunction entry: the request is not supported.
            let irp = emu.kernel_arg(1);
            emu.maps.write_dword(
                irp + IRP_IOSTATUS_STATUS,
                STATUS_INVALID_DEVICE_REQUEST as u32,
            );
            emu.maps.write_qword(irp + IRP_IOSTATUS_INFORMATION, 0);
            emu.set_kernel_ret(STATUS_INVALID_DEVICE_REQUEST);
        }
        _ => return false,
    }
    true
}

pub const SURFACE: &[&str] = &[
    "IoCreateDevice",
    "IoCreateDeviceSecure",
    "WdmlibIoCreateDeviceSecure",
    "IoValidateDeviceIoControlAccess",
    "IoDeleteDevice",
    "IoCreateSymbolicLink",
    "IoCreateUnprotectedSymbolicLink",
    "IoDeleteSymbolicLink",
    "IoGetDeviceObjectPointer",
    "IoAttachDeviceToDeviceStack",
    "IoAttachDeviceToDeviceStackSafe",
    "IoAttachDevice",
    "IoDetachDevice",
    "IoGetAttachedDeviceReference",
    "IoGetAttachedDevice",
    "IoGetRelatedDeviceObject",
    "IoGetLowerDeviceObject",
    "IofCompleteRequest",
    "IoCompleteRequest",
    "IoGetCurrentIrpStackLocation",
    "IoGetNextIrpStackLocation",
    "IoSkipCurrentIrpStackLocation",
    "IoCopyCurrentIrpStackLocationToNext",
    "IoAllocateIrp",
    "IoAllocateIrpEx",
    "IoInitializeIrp",
    "IoFreeIrp",
    "IoMakeAssociatedIrp",
    "IoBuildDeviceIoControlRequest",
    "IoBuildSynchronousFsdRequest",
    "IoBuildAsynchronousFsdRequest",
    "IofCallDriver",
    "IoCallDriver",
    "PoCallDriver",
    "IoForwardIrpSynchronously",
    "IoSetCancelRoutine",
    "IoAcquireCancelSpinLock",
    "IoReleaseCancelSpinLock",
    "IoCancelIrp",
    "IoAllocateMdl",
    "IoFreeMdl",
    "IoBuildPartialMdl",
    "IoSizeofWorkItem",
    "IoAllocateWorkItem",
    "IoInitializeWorkItem",
    "IoUninitializeWorkItem",
    "IoFreeWorkItem",
    "IoQueueWorkItem",
    "IoQueueWorkItemEx",
    "ExQueueWorkItem",
    "IoGetCurrentProcess",
    "IoThreadToProcess",
    "IoGetRequestorProcess",
    "IoGetRequestorProcessId",
    "IoGetStackLimits",
    "IoGetInitialStack",
    "IoGetConfigurationInformation",
    "IoAllocateErrorLogEntry",
    "IoWriteErrorLogEntry",
    "IoCreateNotificationEvent",
    "IoCreateSynchronizationEvent",
    "IoCreateFile",
    "IoCreateFileEx",
    "IoOpenDeviceRegistryKey",
    "IoGetDeviceProperty",
    "IoIsWdmVersionAvailable",
    "IoIs32bitProcess",
    "IoRegisterShutdownNotification",
    "IoUnregisterShutdownNotification",
    "IoRegisterDeviceInterface",
    "IoSetDeviceInterfaceState",
    "IoRegisterPlugPlayNotification",
    "IoUnregisterPlugPlayNotification",
    "IoWMIRegistrationControl",
    "IoWMIWriteEvent",
    "IoReportResourceUsage",
    "IoReportResourceForDetection",
    "IoReportDetectedDevice",
    "IoConnectInterrupt",
    "IoConnectInterruptEx",
    "IoDisconnectInterrupt",
    "IoDisconnectInterruptEx",
    "IoAcquireRemoveLockEx",
    "IoReleaseRemoveLockEx",
    "IoReleaseRemoveLockAndWaitEx",
    "IoInitializeRemoveLockEx",
    "IoRegisterBootDriverReinitialization",
    "IoRegisterDriverReinitialization",
    "IoStartPacket",
    "IoStartNextPacket",
    "IoSetCompletionRoutineEx",
    "IoCsqInitialize",
    "IoCsqInitializeEx",
    "IoAllocateDriverObjectExtension",
    "IoGetDriverObjectExtension",
    "IoGetFileObjectGenericMapping",
    "IoQueryFileDosDeviceName",
    "IoVolumeDeviceToDosName",
    "IoGetDmaAdapter",
    "IopInvalidDeviceRequest",
];
