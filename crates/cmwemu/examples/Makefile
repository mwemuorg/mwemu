# Build the C demo against the cmwemu library.
#
#   make            # build ./demo against the cdylib (needs LD_LIBRARY_PATH to run)
#   make run        # build and run it
#   make static     # link the static lib instead (self-contained binary)
#
# Run `cargo build` first so target/debug/ has the library + header.

TARGET_DIR ?= ../target/debug
CFLAGS     := -I ../include -Wall -Wextra

.PHONY: all run static clean

all: demo

demo: demo.c ../include/mwemu.h
	$(CC) $(CFLAGS) demo.c -L $(TARGET_DIR) -lcmwemu -o demo

run: demo
	LD_LIBRARY_PATH=$(TARGET_DIR) ./demo

# Static link: pull in the system libs the Rust staticlib depends on.
static: demo.c ../include/mwemu.h
	$(CC) $(CFLAGS) demo.c $(TARGET_DIR)/libcmwemu.a \
		-lbz2 -lz -lpthread -ldl -lm -o demo
	./demo

clean:
	rm -f demo
