# Build the mwemu Windows kernel-mode test target with mingw-w64.
#
#   make            -> tlm.sys (x64, native subsystem, imports ntoskrnl.exe)
#   make install    -> copy it into ../../../test/ where the Rust tests look
#
# The driver is deliberately vulnerable and is only ever loaded inside mwemu.

CC      := x86_64-w64-mingw32-gcc
TESTDIR ?= $(CURDIR)/../../../test

# Locate the mingw-w64 DDK headers so this works on Linux
# (/usr/x86_64-w64-mingw32/include/ddk) and on macOS/Homebrew
# (/opt/homebrew/Cellar/mingw-w64/*/toolchain-x86_64/.../include/ddk) alike.
# The DDK headers live under the sysroot, not in gcc's internal search dirs,
# so we derive the sysroot from the compiler and fall back to its install
# prefix (dirname of the gcc binary, minus /bin) if -print-sysroot is empty.
SYSROOT := $(shell $(CC) -print-sysroot 2>/dev/null)
DDK ?= $(firstword $(wildcard \
    $(SYSROOT)/x86_64-w64-mingw32/include/ddk \
    $(SYSROOT)/include/ddk \
    /usr/x86_64-w64-mingw32/include/ddk))

CFLAGS  = -O1 -Wall -Wextra -Wno-unused-parameter -fno-stack-protector -mno-stack-arg-probe \
          -fno-asynchronous-unwind-tables -fno-builtin -ffreestanding \
          -D_WIN32_WINNT=0x0A00 -I$(DDK)
LDFLAGS = -shared -nostdlib -Wl,--subsystem,native -Wl,--entry,DriverEntry \
          -Wl,--exclude-all-symbols -Wl,--disable-runtime-pseudo-reloc -Wl,--disable-auto-import -Wl,--image-base,0x140000000 \
          -Wl,--file-alignment,0x200 -Wl,--section-alignment,0x1000

all: tlm.sys

tlm.sys: tlm.c
	$(CC) $(CFLAGS) $(LDFLAGS) -o $@ $< -lntoskrnl -lhal

install: all
	mkdir -p $(TESTDIR)
	cp tlm.sys $(TESTDIR)/windows_uaf_driver.sys

clean:
	rm -f tlm.sys $(TESTDIR)/windows_uaf_driver.sys

.PHONY: all install clean
