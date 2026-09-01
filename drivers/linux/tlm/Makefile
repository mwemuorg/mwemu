# Build the mwemu kernel-mode test target.
#
#   make            -> tlm.ko against the running kernel's headers
#   make install    -> copy it into ../../../test/ where the Rust tests look
#   make KDIR=/path -> build against another kernel tree
#
# The .ko is a plain ET_REL object: mwemu loads it directly, so nothing here
# ever needs to be inserted into a real kernel (and it should not be — the
# driver is deliberately vulnerable).

obj-m += tlm.o

KDIR ?= /lib/modules/$(shell uname -r)/build
PWD  := $(shell pwd)
TESTDIR ?= $(PWD)/../../../test

all:
	$(MAKE) -C $(KDIR) M=$(PWD) modules

install: all
	mkdir -p $(TESTDIR)
	cp tlm.ko $(TESTDIR)/linux_uaf_driver.ko

clean:
	$(MAKE) -C $(KDIR) M=$(PWD) clean
	rm -f $(TESTDIR)/linux_uaf_driver.ko

.PHONY: all install clean
