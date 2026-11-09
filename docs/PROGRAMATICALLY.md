# Using MWEMU programatically

just modify the main and implement your custom emulation.

# normal usage

The main instatiates emu module, set the config, loads the binary and run.


```rust
    let mut emu = libmwemu::emu32(); // or emu64() / emu_aarch64()
    
    emu.set_config(cfg);
    emu.init();
    emu.load_code(&filename.to_string());

    emu.run(0);
```

`emu32()`, `emu64()` and `emu_aarch64()` also plug in the Windows API handlers.
If you build the emulator another way (`Emu::new(arch)`, `Serialization::load`),
call `libmwemu::register_api_handlers()` once first, or the first API call
stops the emulation with an error.

in run method we can specify a stop address, when emulator reach this address will end the run function.
So we can do multiple runs.

but run(0) means forever.

# calling a specific function

```rust

    let mut emu = libmwemu::emu32();
    
    emu.set_config(cfg);
    emu.init();
    emu.load_code(&filename.to_string());
    
    // alloc a buffer needed to generate the key
    let buff = emu.alloc("key_buffer", 1024);

    // set eip
    emu.regs.set_eip(my_crypto_function_address);  // or emu.regs.rip = my_function_address   in 64bits

    // params pushed in reverse order
    emu.stack_push32(length);
    emu.stack_push32(buff);
    emu.stack_push32(seed);
    emu.stack_push32(0); // return address 

    emu.run(ret_address); // stop on latest ret, dont emulate it that will jump to zero.

    // show the buffer in the screen
    emu.maps.dump(buff);

    // you can spawn the console if needed
    emu.spawn_console();
```

you can build a program that decrypt stuff by emulating malwares's decryption functions.

