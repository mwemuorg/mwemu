/*
 * tlm — a deliberately vulnerable telemetry driver for Windows, the WDM
 * counterpart of drivers/linux/tlm. It is the reference target for mwemu's
 * `.sys` kernel-mode emulation and is only ever loaded inside mwemu.
 *
 * Shape: a device with a symbolic link, a list of refcounted channel objects
 * behind a fast mutex, each with a separately allocated payload buffer, and
 * an IOCTL surface. Two things are wrong with it on purpose:
 *
 *  1. The use-after-free from the Linux version: a one-entry "hot channel"
 *     cache that holds no reference and is not invalidated by TLM_IOC_DESTROY,
 *     so the next write to the destroyed id runs against freed pool.
 *
 *  2. The BYOVD primitives vendor drivers ship by accident: an IOCTL that maps
 *     an arbitrary physical address with MmMapIoSpace and reads it back, and
 *     one that executes rdmsr / wrmsr on a caller-chosen MSR, with nothing
 *     but a magic number standing between user mode and ring 0.
 *
 * Build: make (mingw-w64), never load it into a real system.
 */

#include <ntddk.h>

__declspec(dllimport) extern USHORT NtBuildNumber;

#define TLM_TAG          0x436d6c54u  /* 'TlmC' as the debugger shows it */
#define TLM_BUF_TAG      0x426d6c54u  /* 'TlmB' */
#define TLM_NAME_LEN     24
#define TLM_MAX_BUF      4096
#define TLM_MAGIC        0x544c4d30u  /* "TLM0" */

#define TLM_DEVICE_TYPE  0x8000
#define TLM_IOCTL(fn)    CTL_CODE(TLM_DEVICE_TYPE, 0x800 + (fn), METHOD_BUFFERED, FILE_ANY_ACCESS)

#define TLM_IOC_CREATE   TLM_IOCTL(1)   /* 0x80002004 */
#define TLM_IOC_WRITE    TLM_IOCTL(2)   /* 0x80002008 */
#define TLM_IOC_DESTROY  TLM_IOCTL(3)   /* 0x8000200c */
#define TLM_IOC_MAPPHYS  TLM_IOCTL(4)   /* 0x80002010 */
#define TLM_IOC_RDMSR    TLM_IOCTL(5)   /* 0x80002014 */
#define TLM_IOC_WRMSR    TLM_IOCTL(6)   /* 0x80002018 */

#pragma pack(push, 1)
struct tlm_create_req {
    char   name[TLM_NAME_LEN];
    ULONG  buf_len;
    ULONG  encoding;
    ULONG  id_out;
};

struct tlm_write_req {
    ULONG  id;
    ULONG  len;
    UCHAR  data[1];
};

struct tlm_id_req {
    ULONG  id;
};

struct tlm_phys_req {
    ULONG     magic;
    ULONG     size;
    ULONGLONG phys;
    ULONGLONG value_out;
};

struct tlm_msr_req {
    ULONG     magic;
    ULONG     msr;
    ULONGLONG value;
};
#pragma pack(pop)

struct tlm_channel {
    LIST_ENTRY   link;
    ULONG        magic;
    ULONG        id;
    LONG         refs;
    ULONG        capacity;
    ULONG        used;
    ULONG        encoding;
    PUCHAR       buf;
    char         name[TLM_NAME_LEN];
};

static LIST_ENTRY   g_channels;
static FAST_MUTEX   g_lock;
static ULONG        g_next_id = 1;
static PDEVICE_OBJECT g_device;

/* The bug: a cache that holds no reference. */
static struct tlm_channel *g_hot;

static struct tlm_channel *tlm_lookup(ULONG id)
{
    PLIST_ENTRY e;
    for (e = g_channels.Flink; e != &g_channels; e = e->Flink) {
        struct tlm_channel *ch = CONTAINING_RECORD(e, struct tlm_channel, link);
        if (ch->id == id)
            return ch;
    }
    return NULL;
}

static void tlm_release(struct tlm_channel *ch)
{
    if (InterlockedDecrement(&ch->refs) == 0) {
        ExFreePoolWithTag(ch->buf, TLM_BUF_TAG);
        ExFreePoolWithTag(ch, TLM_TAG);
    }
}

static NTSTATUS tlm_create(struct tlm_create_req *req, ULONG in_len, ULONG_PTR *info)
{
    struct tlm_channel *ch;

    if (in_len < sizeof(*req) || req->buf_len == 0 || req->buf_len > TLM_MAX_BUF)
        return STATUS_INVALID_PARAMETER;

    ch = ExAllocatePoolWithTag(NonPagedPool, sizeof(*ch), TLM_TAG);
    if (!ch)
        return STATUS_INSUFFICIENT_RESOURCES;
    RtlZeroMemory(ch, sizeof(*ch));
    ch->buf = ExAllocatePoolWithTag(NonPagedPool, req->buf_len, TLM_BUF_TAG);
    if (!ch->buf) {
        ExFreePoolWithTag(ch, TLM_TAG);
        return STATUS_INSUFFICIENT_RESOURCES;
    }
    ch->magic = TLM_MAGIC;
    ch->capacity = req->buf_len;
    ch->encoding = req->encoding;
    ch->refs = 1;
    RtlCopyMemory(ch->name, req->name, TLM_NAME_LEN - 1);

    ExAcquireFastMutex(&g_lock);
    ch->id = g_next_id++;
    InsertTailList(&g_channels, &ch->link);
    ExReleaseFastMutex(&g_lock);

    req->id_out = ch->id;
    *info = sizeof(*req);
    DbgPrint("tlm: channel %u '%s' created (%u bytes)\n", ch->id, ch->name, ch->capacity);
    return STATUS_SUCCESS;
}

static NTSTATUS tlm_write(struct tlm_write_req *req, ULONG in_len, ULONG_PTR *info)
{
    struct tlm_channel *ch;
    ULONG n;

    if (in_len < 8 || req->len > in_len - 8)
        return STATUS_INVALID_PARAMETER;

    /* Hot path: the last channel written to, no lookup, no reference. */
    ch = g_hot;
    if (ch == NULL || ch->id != req->id) {
        ExAcquireFastMutex(&g_lock);
        ch = tlm_lookup(req->id);
        if (ch)
            InterlockedIncrement(&ch->refs);
        ExReleaseFastMutex(&g_lock);
        if (!ch)
            return STATUS_NOT_FOUND;
        g_hot = ch;
        tlm_release(ch);
    }

    if (ch->magic != TLM_MAGIC)
        return STATUS_INVALID_HANDLE;
    n = req->len;
    if (n > ch->capacity - ch->used)
        n = ch->capacity - ch->used;
    RtlCopyMemory(ch->buf + ch->used, req->data, n);
    ch->used += n;
    *info = n;
    return STATUS_SUCCESS;
}

static NTSTATUS tlm_destroy(struct tlm_id_req *req, ULONG in_len)
{
    struct tlm_channel *ch;

    if (in_len < sizeof(*req))
        return STATUS_INVALID_PARAMETER;
    ExAcquireFastMutex(&g_lock);
    ch = tlm_lookup(req->id);
    if (ch)
        RemoveEntryList(&ch->link);
    ExReleaseFastMutex(&g_lock);
    if (!ch)
        return STATUS_NOT_FOUND;
    DbgPrint("tlm: channel %u destroyed\n", ch->id);
    /* g_hot is deliberately not cleared here. */
    tlm_release(ch);
    return STATUS_SUCCESS;
}

static NTSTATUS tlm_map_phys(struct tlm_phys_req *req, ULONG in_len, ULONG_PTR *info)
{
    PHYSICAL_ADDRESS pa;
    PVOID va;

    if (in_len < sizeof(*req) || req->magic != TLM_MAGIC)
        return STATUS_INVALID_PARAMETER;
    if (req->size < 8 || req->size > 0x10000)
        return STATUS_INVALID_PARAMETER;
    pa.QuadPart = (LONGLONG)req->phys;
    va = MmMapIoSpace(pa, req->size, MmNonCached);
    if (!va)
        return STATUS_UNSUCCESSFUL;
    req->value_out = *(volatile ULONGLONG *)va;
    MmUnmapIoSpace(va, req->size);
    *info = sizeof(*req);
    return STATUS_SUCCESS;
}

static ULONGLONG tlm_rdmsr(ULONG msr)
{
    ULONG lo, hi;
    __asm__ __volatile__("rdmsr" : "=a"(lo), "=d"(hi) : "c"(msr));
    return ((ULONGLONG)hi << 32) | lo;
}

static void tlm_wrmsr(ULONG msr, ULONGLONG value)
{
    ULONG lo = (ULONG)value, hi = (ULONG)(value >> 32);
    __asm__ __volatile__("wrmsr" : : "c"(msr), "a"(lo), "d"(hi));
}

static NTSTATUS tlm_msr(struct tlm_msr_req *req, ULONG in_len, BOOLEAN write, ULONG_PTR *info)
{
    if (in_len < sizeof(*req) || req->magic != TLM_MAGIC)
        return STATUS_INVALID_PARAMETER;
    if (write)
        tlm_wrmsr(req->msr, req->value);
    else
        req->value = tlm_rdmsr(req->msr);
    *info = sizeof(*req);
    return STATUS_SUCCESS;
}

static NTSTATUS tlm_dispatch(PDEVICE_OBJECT dev, PIRP irp)
{
    PIO_STACK_LOCATION sl = IoGetCurrentIrpStackLocation(irp);
    NTSTATUS status = STATUS_SUCCESS;
    ULONG_PTR info = 0;

    (void)dev;
    if (sl->MajorFunction == IRP_MJ_DEVICE_CONTROL) {
        PVOID buf = irp->AssociatedIrp.SystemBuffer;
        ULONG in_len = sl->Parameters.DeviceIoControl.InputBufferLength;
        switch (sl->Parameters.DeviceIoControl.IoControlCode) {
        case TLM_IOC_CREATE:  status = tlm_create(buf, in_len, &info); break;
        case TLM_IOC_WRITE:   status = tlm_write(buf, in_len, &info); break;
        case TLM_IOC_DESTROY: status = tlm_destroy(buf, in_len); break;
        case TLM_IOC_MAPPHYS: status = tlm_map_phys(buf, in_len, &info); break;
        case TLM_IOC_RDMSR:   status = tlm_msr(buf, in_len, FALSE, &info); break;
        case TLM_IOC_WRMSR:   status = tlm_msr(buf, in_len, TRUE, &info); break;
        default:              status = STATUS_INVALID_DEVICE_REQUEST; break;
        }
    } else if (sl->MajorFunction == IRP_MJ_CLOSE) {
        g_hot = NULL;
    }

    irp->IoStatus.Status = status;
    irp->IoStatus.Information = info;
    IoCompleteRequest(irp, IO_NO_INCREMENT);
    return status;
}

static void tlm_unload(PDRIVER_OBJECT drv)
{
    UNICODE_STRING link;
    (void)drv;
    g_hot = NULL;
    while (!IsListEmpty(&g_channels)) {
        PLIST_ENTRY e = RemoveHeadList(&g_channels);
        tlm_release(CONTAINING_RECORD(e, struct tlm_channel, link));
    }
    RtlInitUnicodeString(&link, L"\\DosDevices\\TlmWin");
    IoDeleteSymbolicLink(&link);
    if (g_device)
        IoDeleteDevice(g_device);
    DbgPrint("tlm: unloaded\n");
}

NTSTATUS DriverEntry(PDRIVER_OBJECT drv, PUNICODE_STRING reg)
{
    UNICODE_STRING name, link;
    NTSTATUS status;

    (void)reg;
    InitializeListHead(&g_channels);
    ExInitializeFastMutex(&g_lock);

    RtlInitUnicodeString(&name, L"\\Device\\TlmWin");
    status = IoCreateDevice(drv, 0, &name, FILE_DEVICE_UNKNOWN, 0, FALSE, &g_device);
    if (!NT_SUCCESS(status))
        return status;
    RtlInitUnicodeString(&link, L"\\DosDevices\\TlmWin");
    status = IoCreateSymbolicLink(&link, &name);
    if (!NT_SUCCESS(status)) {
        IoDeleteDevice(g_device);
        return status;
    }

    drv->MajorFunction[IRP_MJ_CREATE] = tlm_dispatch;
    drv->MajorFunction[IRP_MJ_CLOSE] = tlm_dispatch;
    drv->MajorFunction[IRP_MJ_DEVICE_CONTROL] = tlm_dispatch;
    drv->DriverUnload = tlm_unload;
    g_device->Flags &= ~DO_DEVICE_INITIALIZING;

    DbgPrint("tlm: telemetry driver loaded, build %u\n", NtBuildNumber);
    return STATUS_SUCCESS;
}
