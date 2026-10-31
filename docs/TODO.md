# TODO


1. [ ] ldr update on LoadLibrary
2. [x] implement pe64
3. [ ] optimize GetProcAddress storing in the handler the lib name
4. [x] support vmprotect
5. [x] set all flags
6. [ ] list breakpoints
7. [ ] clear breakpoint bug
8. [ ] md accept registers
9. [ ] md memory check the string filter
10. [ ] mr mw options can crash the console
11. [ ] fix instruction breakpoint
12. [ ] more 64bits apis
13. [ ] in self.execption() put a message self.exception(msg)
14. [ ] improve seh command
15. [ ] better api implementations
16. [ ] winhttp
17. [ ] implement a basic decompiler in rust.
18. [ ] remove expect() on implemented instructions, just break;
19. [ ] stack\_push and stack\_pop assumes the stack is in the memory map stack
20. [ ] step over
21. [ ] more fpu and xmm
22. [ ] on WriteProcessMemory/recv save the payload written to disk
23. [ ] remove non printable bytes from strings
24. [ ] randomize initial register for avoid targeted anti-amulation
25. [ ] support guloader
26. [ ] scripting
27. [ ] intead of panic spawn console
28. [ ] set the code base addr
29. [ ] on every set\_eip of a non branch dump stack to log file
30. [x] other rep instruction preffix
31. [x] check pf flag bug
32. [x] save state to disk and continue
33. [ ] command to exit the bucle or to see  next instruction
34. [x] optimize loop counter


- the string change non printables for spaces instead of points:
```
=>r rax
        rax: 0x3c0037 3932215 'LoadLibraryA    ws2_32.dl' (code)
=>s
0x22dfdc: 0xc () ''
0x22dfe4: 0x3c0037 (code) 'LoadLibraryA....ws2_32.dll'
```

