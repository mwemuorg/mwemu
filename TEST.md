Do this tests and indicate if passes all the tests, if one test fail dont continue and notify it.
If a step is commented '#' dont do that step

1. cargo test    
2. cargo test --release 
3. make test_linux   --> you have to see the result of ls and good termination
4. make test_windows --> (take time) you have to see that arrives at lest to ** 102080270:140457a15 kernel32!GetProcessHeap =3 
5. make test_incepion --> arives to exit: ** 975682 syscall exit()  1
6. make test_syscall --> (take a lot of time) you have to see yellow syscalls  and you have to see that LdrInitializeThunk is completed: ntdll!LdrInitializeThunk emulated completely.
