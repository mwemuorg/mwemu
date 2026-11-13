//! The harness: what the I/O manager does *to* a driver.
//!
//! Loading a `.sys` gives an image with a `DriverEntry`; nothing is reachable
//! until the kernel calls it with a `DRIVER_OBJECT` and the driver fills the
//! `MajorFunction` table and creates a device. From then on every interaction
//! is an IRP delivered to a dispatch routine. This module builds those
//! objects and requests the way the I/O manager does, so the driver's own
//! code runs unmodified:
//!
//! - [`Emu::run_driver_entry`] — `DriverEntry(DriverObject, RegistryPath)`
//! - [`Emu::kernel_ioctl`] — `IRP_MJ_DEVICE_CONTROL` with the buffering
//!   method the control code asks for (buffered, direct, neither)
//! - [`Emu::kernel_create`] / [`Emu::kernel_close`] / [`Emu::kernel_read`] /
//!   [`Emu::kernel_write`] — the other requests a user-mode client can send
//! - [`Emu::kernel_scan_ioctls`] — harvest candidate control codes from the
//!   driver's own code and try each, reporting which privileged operation it
//!   reaches: the BYOVD question.

use super::structs::*;
use super::*;
use crate::err::MwemuError;
use crate::kernel::heap::Region;

/// Outcome of one request delivered to the driver.
#[derive(Debug, Clone, Default)]
pub struct IrpResult {
    /// NTSTATUS the dispatch routine returned.
    pub status: u32,
    /// `IoStatus.Status`, what the driver completed the IRP with.
    pub io_status: u32,
    /// `IoStatus.Information`: bytes transferred.
    pub information: u64,
    /// The output buffer after the request (whole buffer, not just
    /// `information` bytes — drivers forget to set it).
    pub output: Vec<u8>,
    /// Whether the driver called `IofCompleteRequest` on the IRP.
    pub completed: bool,
    /// Error from the run loop, if the driver faulted.
    pub error: Option<String>,
    /// Privileged operations this request performed (MSR, port, physical).
    pub privops: Vec<crate::kernel::PrivilegedOp>,
    /// Kernel APIs this request called that have no implementation.
    pub unimplemented: Vec<String>,
}

impl IrpResult {
    pub fn succeeded(&self) -> bool {
        self.status & 0x8000_0000 == 0 && self.error.is_none()
    }
}

/// One line of an IOCTL scan.
#[derive(Debug, Clone)]
pub struct IoctlProbe {
    pub code: u32,
    /// Input length that got the furthest (fewest rejections).
    pub input_len: usize,
    pub result: IrpResult,
}

impl IoctlProbe {
    pub fn report(&self) -> String {
        let method = match self.code & 3 {
            METHOD_BUFFERED => "buffered",
            METHOD_IN_DIRECT => "in_direct",
            METHOD_OUT_DIRECT => "out_direct",
            _ => "neither",
        };
        let mut s = format!(
            "ioctl 0x{:08x} ({}, in={}) -> status 0x{:08x} info {}",
            self.code, method, self.input_len, self.result.status, self.result.information
        );
        if let Some(e) = &self.result.error {
            s.push_str(&format!(" [fault: {}]", e));
        }
        for p in &self.result.privops {
            s.push_str(&format!("\n    {}", p.report()));
        }
        if !self.result.unimplemented.is_empty() {
            s.push_str(&format!(
                "\n    unimplemented: {}",
                self.result.unimplemented.join(", ")
            ));
        }
        s
    }
}

/// Build the `DRIVER_OBJECT` DriverEntry receives: every `MajorFunction`
/// slot points at `IopInvalidDeviceRequest`, exactly as on a real kernel.
fn build_driver_object(emu: &mut Emu, name: &str) -> Option<u64> {
    let drv = emu.kernel_data_for("win.driver_object", DRIVER_OBJECT_SIZE)?;
    let ext = emu.kernel_data_for("win.driver_extension", DRIVER_EXTENSION_SIZE)?;
    let (base, size) = emu
        .kernel
        .as_ref()
        .map(|k| (k.module.base, k.module.size))?;
    emu.maps
        .write_bytes(drv, &[0u8; DRIVER_OBJECT_SIZE as usize]);
    emu.maps.write_word(drv + DRVOBJ_TYPE, IO_TYPE_DRIVER);
    emu.maps
        .write_word(drv + DRVOBJ_SIZE, DRIVER_OBJECT_SIZE as u16);
    emu.maps.write_qword(drv + DRVOBJ_DRIVER_START, base);
    emu.maps.write_dword(drv + DRVOBJ_DRIVER_SIZE, size as u32);
    emu.maps.write_qword(drv + DRVOBJ_DRIVER_EXTENSION, ext);
    emu.maps.write_qword(ext + DRVEXT_DRIVER_OBJECT, drv);
    let init = emu.kernel.as_ref().and_then(|k| k.module.init).unwrap_or(0);
    emu.maps.write_qword(drv + DRVOBJ_DRIVER_INIT, init);

    // I/O manager objects live in kernel data, not in the pool: the ledger
    // only tracks what the driver itself allocates.
    let driver_name = format!("\\Driver\\{}", name);
    let buf = emu.kernel_data_for("win.driver_name", 0x100)?;
    emu.maps.write_wide_string(buf, &driver_name);
    let len = driver_name.encode_utf16().count() as u64 * 2;
    write_unicode_header(emu, drv + DRVOBJ_DRIVER_NAME, buf, len, 0x100);
    let service = emu.kernel_data_for("win.service_key_name", 0x80)?;
    emu.maps
        .write_wide_string(service + UNICODE_STRING_SIZE, name);
    write_unicode_header(
        emu,
        service,
        service + UNICODE_STRING_SIZE,
        name.len() as u64 * 2,
        0x80 - UNICODE_STRING_SIZE,
    );
    emu.maps.memcpy(
        ext + DRVEXT_SERVICE_KEY_NAME,
        service,
        UNICODE_STRING_SIZE as usize,
    );

    let invalid = emu.kernel_stub_for("IopInvalidDeviceRequest").unwrap_or(0);
    for i in 0..=IRP_MJ_MAXIMUM_FUNCTION {
        emu.maps
            .write_qword(drv + DRVOBJ_MAJOR_FUNCTION + i * 8, invalid);
    }
    Some(drv)
}

fn build_registry_path(emu: &mut Emu, name: &str) -> Option<u64> {
    let us = emu.kernel_data_for("win.registry_path", 0x200)?;
    let path = format!(
        "\\REGISTRY\\MACHINE\\SYSTEM\\ControlSet001\\Services\\{}",
        name
    );
    let buf = us + UNICODE_STRING_SIZE;
    emu.maps.write_wide_string(buf, &path);
    write_unicode_header(
        emu,
        us,
        buf,
        path.encode_utf16().count() as u64 * 2,
        0x200 - UNICODE_STRING_SIZE,
    );
    Some(us)
}

/// The `FILE_OBJECT` the harness's requests arrive on, created on first use.
pub fn ensure_file_object(emu: &mut Emu, device: u64) -> u64 {
    if let Some(f) = emu
        .kernel
        .as_ref()
        .map(|k| k.win.file_object)
        .filter(|f| *f != 0)
    {
        return f;
    }
    let f = emu
        .kernel_data_for("win.file_object", FILE_OBJECT_SIZE)
        .unwrap_or(0);
    if f != 0 {
        emu.maps.write_word(f + FILEOBJ_TYPE, IO_TYPE_FILE);
        emu.maps
            .write_word(f + FILEOBJ_SIZE, FILE_OBJECT_SIZE as u16);
        emu.maps.write_qword(f + FILEOBJ_DEVICE_OBJECT, device);
        emu.maps.write_dword(f + FILEOBJ_FLAGS, 0x40000); // FO_HANDLE_CREATED
        write_unicode_header(emu, f + FILEOBJ_FILE_NAME, 0, 0, 0);
        if let Some(k) = emu.kernel.as_mut() {
            k.win.file_object = f;
        }
    }
    f
}

/// Describe one request to deliver.
struct Request {
    major: u8,
    minor: u8,
    code: u32,
    input: Vec<u8>,
    output_len: usize,
    byte_offset: u64,
}

impl Emu {
    /// The `DRIVER_OBJECT` the driver was given (0 before DriverEntry).
    pub fn kernel_driver_object(&self) -> u64 {
        self.kernel
            .as_ref()
            .map(|k| k.win.driver_object)
            .unwrap_or(0)
    }

    /// Devices the driver created, in creation order.
    pub fn kernel_devices(&self) -> Vec<KernelDevice> {
        self.kernel
            .as_ref()
            .map(|k| k.win.devices.clone())
            .unwrap_or_default()
    }

    /// `\DosDevices\X -> \Device\Y` links the driver created.
    pub fn kernel_symlinks(&self) -> Vec<(String, String)> {
        self.kernel
            .as_ref()
            .map(|k| k.win.symlinks.clone())
            .unwrap_or_default()
    }

    /// Address of the driver's `MajorFunction[major]`, or None when the slot
    /// still holds the kernel's default (`IopInvalidDeviceRequest`).
    pub fn kernel_dispatch(&self, major: u8) -> Option<u64> {
        let drv = self.kernel_driver_object();
        if drv == 0 || major as u64 > IRP_MJ_MAXIMUM_FUNCTION {
            return None;
        }
        let f = self
            .maps
            .read_qword(drv + DRVOBJ_MAJOR_FUNCTION + major as u64 * 8)?;
        let module = &self.kernel.as_ref()?.module;
        if f != 0 && module.contains(f) {
            Some(f)
        } else {
            None
        }
    }

    /// Every `MajorFunction` the driver installed: (index, name, address).
    pub fn kernel_dispatch_table(&self) -> Vec<(u8, &'static str, u64)> {
        (0..=IRP_MJ_MAXIMUM_FUNCTION as u8)
            .filter_map(|i| {
                self.kernel_dispatch(i)
                    .map(|f| (i, IRP_MJ_NAMES[i as usize], f))
            })
            .collect()
    }

    /// Call `DriverEntry(DriverObject, RegistryPath)` as the I/O manager
    /// would, then capture what the driver installed: its unload routine,
    /// its dispatch table (exposed as module symbols named after the
    /// `IRP_MJ_*` slot) and its devices.
    pub fn run_driver_entry(&mut self) -> Result<u64, MwemuError> {
        let init = self
            .kernel
            .as_ref()
            .and_then(|k| k.module.init)
            .ok_or_else(|| MwemuError::new("driver has no DriverEntry"))?;
        let name = self
            .kernel
            .as_ref()
            .map(|k| k.module.name.clone())
            .unwrap_or_default();
        bootstrap(self);
        let drv = build_driver_object(self, &name)
            .ok_or_else(|| MwemuError::new("cannot build DRIVER_OBJECT"))?;
        let reg = build_registry_path(self, &name)
            .ok_or_else(|| MwemuError::new("cannot build registry path"))?;
        if let Some(k) = self.kernel.as_mut() {
            k.win.driver_object = drv;
            k.win.registry_path = reg;
        }
        log::info!("DriverEntry(0x{:x}, 0x{:x}) at 0x{:x}", drv, reg, init);
        let ret = self.kernel_call(init, &[drv, reg])?;
        self.capture_driver_object();
        Ok(ret)
    }

    /// Read back what DriverEntry stored in the driver object.
    fn capture_driver_object(&mut self) {
        let drv = self.kernel_driver_object();
        if drv == 0 {
            return;
        }
        let unload = self
            .maps
            .read_qword(drv + DRVOBJ_DRIVER_UNLOAD)
            .unwrap_or(0);
        let table = self.kernel_dispatch_table();
        let Some(k) = self.kernel.as_mut() else {
            return;
        };
        if unload != 0 && k.module.contains(unload) {
            k.module.exit = Some(unload);
            k.module
                .symbols
                .push(rs_header::elf::relocatable::RelSymbol {
                    name: "DriverUnload".to_string(),
                    addr: unload,
                    size: 0,
                    is_func: true,
                    is_global: true,
                });
        }
        for (_, name, addr) in &table {
            if k.module.symbol(name).is_none() {
                k.module
                    .symbols
                    .push(rs_header::elf::relocatable::RelSymbol {
                        name: name.to_string(),
                        addr: *addr,
                        size: 0,
                        is_func: true,
                        is_global: true,
                    });
            }
        }
        let add_device = self
            .maps
            .read_qword(drv + DRVOBJ_DRIVER_EXTENSION)
            .and_then(|ext| self.maps.read_qword(ext + DRVEXT_ADD_DEVICE))
            .unwrap_or(0);
        let Some(k) = self.kernel.as_mut() else {
            return;
        };
        if add_device != 0 && k.module.contains(add_device) {
            k.module
                .symbols
                .push(rs_header::elf::relocatable::RelSymbol {
                    name: "AddDevice".to_string(),
                    addr: add_device,
                    size: 0,
                    is_func: true,
                    is_global: true,
                });
        }
        for (_, name, addr) in &table {
            log::info!("dispatch {} -> 0x{:x}", name, addr);
        }
        for d in &k.win.devices {
            log::info!(
                "device '{}' at 0x{:x} (type 0x{:x})",
                d.name,
                d.addr,
                d.device_type
            );
        }
    }

    /// Resolve the device a request goes to: an explicit object, or the
    /// first device the driver created.
    fn target_device(&self, device: u64) -> Result<u64, MwemuError> {
        if device != 0 {
            return Ok(device);
        }
        self.kernel
            .as_ref()
            .and_then(|k| k.win.default_device())
            .ok_or_else(|| MwemuError::new("the driver created no device object"))
    }

    /// Build an IRP for `req`, deliver it to `device`'s dispatch routine and
    /// collect the outcome. The buffers are mapped where a user-mode caller's
    /// would be, and the system buffer / MDL are set up per the transfer
    /// method, so the driver's validation code sees a realistic request.
    fn deliver(&mut self, device: u64, req: Request) -> Result<IrpResult, MwemuError> {
        let device = self.target_device(device)?;
        let dispatch = self.kernel_dispatch(req.major).ok_or_else(|| {
            MwemuError::new(&format!(
                "the driver installed no {} handler",
                IRP_MJ_NAMES[req.major as usize]
            ))
        })?;
        let file = ensure_file_object(self, device);
        let stack = self
            .maps
            .read_byte(device + DEVOBJ_STACK_SIZE)
            .unwrap_or(1)
            .max(1);
        let total = IRP_SIZE + stack as u64 * IO_STACK_LOCATION_SIZE;
        let irp = self.kernel_alloc(Region::Slab, total, "IRP", "IoAllocateIrp", true);
        if irp == 0 {
            return Err(MwemuError::new("cannot allocate an IRP"));
        }
        io::init_irp(self, irp, stack);
        // The I/O manager positions the current location on the last slot
        // before calling the top driver.
        let sl = irp + IRP_SIZE + (stack as u64 - 1) * IO_STACK_LOCATION_SIZE;
        self.maps.write_qword(irp + IRP_CURRENT_STACK_LOCATION, sl);
        self.maps.write_byte(irp + IRP_CURRENT_LOCATION, 1);
        self.maps.write_byte(sl + IOSL_MAJOR_FUNCTION, req.major);
        self.maps.write_byte(sl + IOSL_MINOR_FUNCTION, req.minor);
        self.maps.write_qword(sl + IOSL_DEVICE_OBJECT, device);
        self.maps.write_qword(sl + IOSL_FILE_OBJECT, file);
        self.maps.write_qword(irp + IRP_ORIGINAL_FILE_OBJECT, file);
        let thread = self
            .kernel
            .as_ref()
            .map(|k| k.win.current_thread)
            .unwrap_or(0);
        self.maps.write_qword(irp + IRP_TAIL_THREAD, thread);

        // User-mode view of the buffers.
        let user_in = if req.input.is_empty() {
            0
        } else {
            let p = alloc_user_buffer(self, req.input.len() as u64, "in");
            self.maps.write_bytes(p, &req.input);
            p
        };
        let user_out = if req.output_len == 0 {
            0
        } else {
            alloc_user_buffer(self, req.output_len as u64, "out")
        };
        let in_len = req.input.len() as u32;
        let out_len = req.output_len as u32;
        let mut system_buffer = 0;
        let mut output_source = user_out;

        match req.major {
            IRP_MJ_DEVICE_CONTROL | IRP_MJ_INTERNAL_DEVICE_CONTROL => {
                self.maps.write_dword(sl + IOSL_IOCTL_CODE, req.code);
                self.maps.write_dword(sl + IOSL_IOCTL_INPUT_LENGTH, in_len);
                self.maps
                    .write_dword(sl + IOSL_IOCTL_OUTPUT_LENGTH, out_len);
                match req.code & 3 {
                    METHOD_BUFFERED => {
                        let n = in_len.max(out_len) as u64;
                        if n > 0 {
                            system_buffer = self.kernel_alloc(
                                Region::Slab,
                                n,
                                "SystemBuffer",
                                "IoAllocateIrp",
                                true,
                            );
                            if !req.input.is_empty() {
                                self.maps.write_bytes(system_buffer, &req.input);
                            }
                        }
                        self.maps
                            .write_qword(irp + IRP_SYSTEM_BUFFER, system_buffer);
                        self.maps.write_qword(irp + IRP_USER_BUFFER, user_out);
                        self.maps.write_dword(
                            irp + IRP_FLAGS,
                            IRP_BUFFERED_IO | IRP_DEALLOCATE_BUFFER | IRP_INPUT_OPERATION,
                        );
                        output_source = system_buffer;
                    }
                    METHOD_IN_DIRECT | METHOD_OUT_DIRECT => {
                        if in_len > 0 {
                            system_buffer = self.kernel_alloc(
                                Region::Slab,
                                in_len as u64,
                                "SystemBuffer",
                                "IoAllocateIrp",
                                true,
                            );
                            self.maps.write_bytes(system_buffer, &req.input);
                            self.maps
                                .write_qword(irp + IRP_SYSTEM_BUFFER, system_buffer);
                            self.maps.write_dword(
                                irp + IRP_FLAGS,
                                IRP_BUFFERED_IO | IRP_DEALLOCATE_BUFFER,
                            );
                        }
                        if user_out != 0 {
                            let mdl =
                                io::alloc_mdl(self, user_out, out_len as u64, "IoAllocateMdl");
                            self.maps.write_qword(irp + IRP_MDL_ADDRESS, mdl);
                        }
                    }
                    _ => {
                        self.maps.write_qword(sl + IOSL_IOCTL_TYPE3_INPUT, user_in);
                        self.maps.write_qword(irp + IRP_USER_BUFFER, user_out);
                    }
                }
            }
            IRP_MJ_READ | IRP_MJ_WRITE => {
                let len = if req.major == IRP_MJ_READ {
                    out_len
                } else {
                    in_len
                };
                self.maps.write_dword(sl + IOSL_RW_LENGTH, len);
                self.maps
                    .write_qword(sl + IOSL_RW_BYTE_OFFSET, req.byte_offset);
                let flags = self.maps.read_dword(device + DEVOBJ_FLAGS).unwrap_or(0);
                const DO_BUFFERED_IO: u32 = 0x4;
                const DO_DIRECT_IO: u32 = 0x10;
                let user = if req.major == IRP_MJ_READ {
                    user_out
                } else {
                    user_in
                };
                if flags & DO_BUFFERED_IO != 0 {
                    if len > 0 {
                        system_buffer = self.kernel_alloc(
                            Region::Slab,
                            len as u64,
                            "SystemBuffer",
                            "IoAllocateIrp",
                            true,
                        );
                        if req.major == IRP_MJ_WRITE {
                            self.maps.write_bytes(system_buffer, &req.input);
                        }
                    }
                    self.maps
                        .write_qword(irp + IRP_SYSTEM_BUFFER, system_buffer);
                    self.maps.write_qword(irp + IRP_USER_BUFFER, user);
                    self.maps.write_dword(
                        irp + IRP_FLAGS,
                        IRP_BUFFERED_IO
                            | IRP_DEALLOCATE_BUFFER
                            | if req.major == IRP_MJ_READ {
                                IRP_INPUT_OPERATION
                            } else {
                                0
                            },
                    );
                    output_source = system_buffer;
                } else if flags & DO_DIRECT_IO != 0 {
                    if user != 0 && len > 0 {
                        let mdl = io::alloc_mdl(self, user, len as u64, "IoAllocateMdl");
                        self.maps.write_qword(irp + IRP_MDL_ADDRESS, mdl);
                    }
                } else {
                    self.maps.write_qword(irp + IRP_USER_BUFFER, user);
                }
            }
            _ => {}
        }

        let before_privops = self.kernel.as_ref().map(|k| k.privops.len()).unwrap_or(0);
        let before_unimpl = self
            .kernel
            .as_ref()
            .map(|k| k.unimplemented.len())
            .unwrap_or(0);
        log::info!(
            "sending {} (code 0x{:x}, in {} B, out {} B) to device 0x{:x} via 0x{:x}",
            IRP_MJ_NAMES[req.major as usize],
            req.code,
            in_len,
            out_len,
            device,
            dispatch
        );
        let call = self.kernel_call(dispatch, &[device, irp]);
        let (status, error) = match call {
            Ok(v) => (v as u32, None),
            Err(e) => (0xC000_0001, Some(e.to_string())),
        };

        let io_status = self.maps.read_dword(irp + IRP_IOSTATUS_STATUS).unwrap_or(0);
        let information = self
            .maps
            .read_qword(irp + IRP_IOSTATUS_INFORMATION)
            .unwrap_or(0);
        let completed = self
            .kernel
            .as_ref()
            .is_some_and(|k| k.win.completed_irps.contains(&irp));
        let mut output = vec![0u8; req.output_len];
        if output_source != 0 {
            for (i, b) in output.iter_mut().enumerate() {
                *b = self.maps.read_byte(output_source + i as u64).unwrap_or(0);
            }
        }
        let (privops, unimplemented) = self
            .kernel
            .as_ref()
            .map(|k| {
                (
                    k.privops[before_privops..].to_vec(),
                    k.unimplemented[before_unimpl..].to_vec(),
                )
            })
            .unwrap_or_default();

        // Release what the I/O manager owns. The driver may have kept the
        // IRP (STATUS_PENDING) — then it stays, like a real pended request.
        if status != STATUS_PENDING as u32 {
            if system_buffer != 0 {
                self.kernel_free(system_buffer, "IoCompleteRequest");
            }
            let mdl = self.maps.read_qword(irp + IRP_MDL_ADDRESS).unwrap_or(0);
            if mdl != 0 {
                self.kernel_free(mdl, "IoFreeMdl");
            }
            if let Some(k) = self.kernel.as_mut() {
                k.win.completed_irps.retain(|i| *i != irp);
            }
            self.kernel_free(irp, "IoFreeIrp");
        }
        Ok(IrpResult {
            status,
            io_status,
            information,
            output,
            completed,
            error,
            privops,
            unimplemented,
        })
    }

    /// Send `IRP_MJ_DEVICE_CONTROL` with `code` to `device` (0 = the first
    /// device the driver created). `input` is the caller's input buffer and
    /// `output_len` the size of its output buffer.
    pub fn kernel_ioctl(
        &mut self,
        device: u64,
        code: u32,
        input: &[u8],
        output_len: usize,
    ) -> Result<IrpResult, MwemuError> {
        self.deliver(
            device,
            Request {
                major: IRP_MJ_DEVICE_CONTROL,
                minor: 0,
                code,
                input: input.to_vec(),
                output_len,
                byte_offset: 0,
            },
        )
    }

    /// Send a request with no buffers: `IRP_MJ_CREATE`, `IRP_MJ_CLOSE`,
    /// `IRP_MJ_CLEANUP`, `IRP_MJ_SHUTDOWN`, ...
    pub fn kernel_send_irp(
        &mut self,
        device: u64,
        major: u8,
        minor: u8,
    ) -> Result<IrpResult, MwemuError> {
        self.deliver(
            device,
            Request {
                major,
                minor,
                code: 0,
                input: Vec::new(),
                output_len: 0,
                byte_offset: 0,
            },
        )
    }

    /// `IRP_MJ_CREATE`: what `CreateFile("\\\\.\\Device")` sends.
    pub fn kernel_create(&mut self, device: u64) -> Result<IrpResult, MwemuError> {
        self.kernel_send_irp(device, IRP_MJ_CREATE, 0)
    }

    /// `IRP_MJ_CLEANUP` followed by `IRP_MJ_CLOSE`: what `CloseHandle` sends.
    pub fn kernel_close(&mut self, device: u64) -> Result<IrpResult, MwemuError> {
        if self.kernel_dispatch(IRP_MJ_CLEANUP).is_some() {
            self.kernel_send_irp(device, IRP_MJ_CLEANUP, 0)?;
        }
        self.kernel_send_irp(device, IRP_MJ_CLOSE, 0)
    }

    /// `IRP_MJ_READ` of `len` bytes at `offset`.
    pub fn kernel_read(
        &mut self,
        device: u64,
        len: usize,
        offset: u64,
    ) -> Result<IrpResult, MwemuError> {
        self.deliver(
            device,
            Request {
                major: IRP_MJ_READ,
                minor: 0,
                code: 0,
                input: Vec::new(),
                output_len: len,
                byte_offset: offset,
            },
        )
    }

    /// `IRP_MJ_WRITE` of `data` at `offset`.
    pub fn kernel_write(
        &mut self,
        device: u64,
        data: &[u8],
        offset: u64,
    ) -> Result<IrpResult, MwemuError> {
        self.deliver(
            device,
            Request {
                major: IRP_MJ_WRITE,
                minor: 0,
                code: 0,
                input: data.to_vec(),
                output_len: 0,
                byte_offset: offset,
            },
        )
    }

    /// Run the driver's unload routine, as `sc stop` would.
    pub fn run_driver_unload(&mut self) -> Result<u64, MwemuError> {
        let unload = self
            .kernel
            .as_ref()
            .and_then(|k| k.module.exit)
            .ok_or_else(|| MwemuError::new("the driver installed no DriverUnload"))?;
        let drv = self.kernel_driver_object();
        self.kernel_call(unload, &[drv])
    }

    /// Candidate IOCTL codes: 32-bit immediates in the driver's code whose
    /// shape is a control code (`CTL_CODE(device, function, method, access)`
    /// with a plausible device type and function number). Drivers compare
    /// `IoControlCode` against these constants in their dispatch routine, so
    /// harvesting them is a reliable static source of what to send.
    pub fn kernel_ioctl_candidates(&self) -> Vec<u32> {
        use iced_x86::{Decoder, DecoderOptions, Mnemonic, OpKind, Register};
        let Some(k) = self.kernel.as_ref() else {
            return Vec::new();
        };
        let mut codes: Vec<u32> = Vec::new();
        let mut push = |code: u32| {
            if looks_like_ioctl(code) && !codes.contains(&code) {
                codes.push(code);
            }
        };
        for sec in k
            .module
            .sections
            .iter()
            .filter(|s| s.perm.execute && s.size > 0)
        {
            let mut bytes = vec![0u8; sec.size as usize];
            for (i, b) in bytes.iter_mut().enumerate() {
                *b = self.maps.read_byte(sec.addr + i as u64).unwrap_or(0);
            }
            let mut decoder = Decoder::with_ip(64, &bytes, sec.addr, DecoderOptions::NONE);
            // A dense switch compiles to `add/sub reg, -base` then `cmp reg,
            // range` + `ja`: the codes are base..=base+range, none of them an
            // immediate on its own. Remember the last rebased register.
            let mut rebased: Option<(Register, u32, u32)> = None; // (reg, base, age)
            while decoder.can_decode() {
                let ins = decoder.decode();
                let imm32 = |i: u32| match ins.op_kind(i) {
                    OpKind::Immediate32 => Some(ins.immediate32()),
                    OpKind::Immediate32to64 => Some(ins.immediate32to64() as u32),
                    OpKind::Immediate8to32 => Some(ins.immediate8to32() as u32),
                    OpKind::Immediate8to64 => Some(ins.immediate8to64() as u32),
                    OpKind::Immediate8 => Some(ins.immediate8() as u32),
                    _ => None,
                };
                for i in 0..ins.op_count() {
                    if let Some(imm) = imm32(i) {
                        push(imm);
                    }
                }
                if let Some((_, _, age)) = rebased.as_mut() {
                    *age += 1;
                }
                let is_reg0 = ins.op_count() >= 2 && ins.op0_kind() == OpKind::Register;
                match ins.mnemonic() {
                    Mnemonic::Add | Mnemonic::Sub if is_reg0 => {
                        if let Some(imm) = imm32(1) {
                            let base = if ins.mnemonic() == Mnemonic::Add {
                                imm.wrapping_neg()
                            } else {
                                imm
                            };
                            if looks_like_ioctl(base) {
                                rebased = Some((ins.op0_register().full_register32(), base, 0));
                            }
                        }
                    }
                    Mnemonic::Cmp if is_reg0 => {
                        if let (Some((reg, base, age)), Some(range)) = (rebased, imm32(1))
                            && age <= 4
                            && ins.op0_register().full_register32() == reg
                            && range <= 0x1000
                        {
                            for k in 0..=range {
                                let code = base.wrapping_add(k);
                                if code & 3 == base & 3 {
                                    push(code);
                                }
                            }
                            rebased = None;
                        }
                    }
                    _ => {}
                }
            }
        }
        codes.sort_unstable();
        codes
    }

    /// Try every candidate control code with a few input sizes and report
    /// what each reaches. A request that is rejected for its size (the usual
    /// first check) is retried with the next size; the probe keeps the
    /// attempt that got the furthest.
    pub fn kernel_scan_ioctls(&mut self, device: u64, output_len: usize) -> Vec<IoctlProbe> {
        const SIZES: [usize; 9] = [0, 4, 8, 16, 24, 32, 64, 256, 4096];
        let candidates = self.kernel_ioctl_candidates();
        let mut probes = Vec::new();
        for code in candidates {
            let mut best: Option<IoctlProbe> = None;
            for &n in &SIZES {
                let input = vec![0x41u8; n];
                let Ok(result) = self.kernel_ioctl(device, code, &input, output_len.max(n)) else {
                    break;
                };
                let score = (result.privops.len() * 4) as i64
                    + result.succeeded() as i64 * 2
                    + (result.io_status != STATUS_INVALID_PARAMETER as u32
                        && result.io_status != STATUS_BUFFER_TOO_SMALL as u32
                        && result.io_status != STATUS_INFO_LENGTH_MISMATCH as u32
                        && result.status != STATUS_INVALID_DEVICE_REQUEST as u32)
                        as i64;
                let probe = IoctlProbe {
                    code,
                    input_len: n,
                    result,
                };
                let better = best.as_ref().is_none_or(|b| {
                    let bs = (b.result.privops.len() * 4) as i64
                        + b.result.succeeded() as i64 * 2
                        + (b.result.io_status != STATUS_INVALID_PARAMETER as u32
                            && b.result.io_status != STATUS_BUFFER_TOO_SMALL as u32
                            && b.result.io_status != STATUS_INFO_LENGTH_MISMATCH as u32
                            && b.result.status != STATUS_INVALID_DEVICE_REQUEST as u32)
                            as i64;
                    score > bs
                });
                let reached_primitive = !probe.result.privops.is_empty();
                // A request that ran out of its instruction budget is looping
                // on something other than the input size: no point retrying.
                let looped = probe
                    .result
                    .error
                    .as_ref()
                    .is_some_and(|e| e.contains("budget"));
                if better {
                    best = Some(probe);
                }
                if reached_primitive || looped {
                    break;
                }
            }
            if let Some(p) = best {
                probes.push(p);
            }
        }
        probes
    }
}

/// `CTL_CODE(DeviceType, Function, Method, Access)`: device type in the
/// top 16 bits (a known type, or a vendor one with bit 15 set), function in
/// bits 2..13 (vendor codes start at 0x800 but drivers use anything), the
/// reserved bit 13 is clear for non-WDF codes.
fn looks_like_ioctl(v: u32) -> bool {
    let device_type = v >> 16;
    let function = (v >> 2) & 0xfff;
    if device_type == 0 || device_type == 0xffff {
        return false;
    }
    let known_type = device_type <= 0x60 || device_type & 0x8000 != 0;
    if !known_type {
        return false;
    }
    // A function of 0 with method 0 and access 0 is a bare device type.
    if function == 0 && v & 0xffff == 0 {
        return false;
    }
    // Function numbers above 0xfff cannot encode; above 0xd00 are rare
    // enough that they are almost always an unrelated constant.
    function != 0 || v & 0xc000 != 0
}
