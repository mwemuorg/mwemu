# tlm (Windows)

The WDM twin of [`../../linux/tlm`](../../linux/tlm): a deliberately
vulnerable telemetry driver that mwemu's `.sys` kernel-mode emulation is
tested against. **Never load it into a real system.**

```
make           # tlm.sys with mingw-w64 (x86_64-w64-mingw32-gcc + DDK headers)
make install   # -> ../../../test/windows_uaf_driver.sys (what the tests load)
```

What it exercises:

- `DriverEntry` creates `\Device\TlmWin` and `\DosDevices\TlmWin`, installs
  `IRP_MJ_CREATE` / `CLOSE` / `DEVICE_CONTROL`, reads the `NtBuildNumber`
  data import, logs through `DbgPrint`.
- `TLM_IOC_CREATE` / `WRITE` / `DESTROY` (`0x80002004` / `08` / `0c`): pool
  objects behind a fast mutex with a one-entry hot cache that holds no
  reference and is not invalidated on destroy — write after destroy is a
  **use-after-free** the ledger reports with the pool tag `TlmC`.
- `TLM_IOC_MAPPHYS` (`0x80002010`): `MmMapIoSpace` on a caller-chosen physical
  address behind a magic value — the BYOVD physical-memory primitive.
- `TLM_IOC_RDMSR` / `WRMSR` (`0x80002014` / `18`): `rdmsr` / `wrmsr` on a
  caller-chosen MSR.

`cargo test -p libmwemu tests::kernel::windows` drives all of it; the tests
skip when the artefact is absent, so CI without mingw stays green.
