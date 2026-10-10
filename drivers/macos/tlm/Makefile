# Build the mwemu macOS kext test target.
#
#   make           -> tlm.kext (arm64e) against the SDK's Kernel.framework
#   make install   -> copy it into ../../../test/ where the Rust tests look
#
# The .kext is loaded directly by mwemu, so it is never kextloaded into a real
# kernel (and must not be — the driver is deliberately vulnerable). No code
# signing, SIP, or Secure Boot is involved.

ARCH     ?= arm64e
SDK      := $(shell xcrun --show-sdk-path)
INC      := -I$(SDK)/System/Library/Frameworks/Kernel.framework/Headers
CFLAGS   := -arch $(ARCH) -mkernel -nostdlib -fno-builtin $(INC)
LDFLAGS  := -Xlinker -kext -lkmod -lcc_kext
TESTDIR  ?= $(abspath $(CURDIR)/../../../test)

all: tlm.kext

tlm.kext: tlm.c
	clang $(CFLAGS) $(LDFLAGS) -o $@ $<

install: all
	mkdir -p $(TESTDIR)
	cp tlm.kext $(TESTDIR)/macos_uaf_driver.kext

clean:
	rm -f tlm.kext $(TESTDIR)/macos_uaf_driver.kext

.PHONY: all install clean
