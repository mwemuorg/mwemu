
# BENCHMARK

Let's emulate 10 million of instructions and measure it 5 times.

The time include x86 emulation and Windows simulation.

libmwemu version: 0.21.4

```
processor	: 15
vendor_id	: AuthenticAMD
cpu family	: 25
model		: 117
model name	: AMD Ryzen 9 8945HS w/ Radeon 780M Graphics
stepping	: 2
cpu MHz		: 400.000
cache size	: 1024 KB

~/s/mwemu ❯❯❯ free                  
               total       usado       libre  compartido   búf/caché  disponible
Mem:        63555984    22467896     8691988      497368    32396100    35248148
Inter:      16777212       39424    16737788
```


## 32bits with verbose (-vv)

```bash
time echo q | cargo run --release -- -f  test/sc32win_donut.bin -vv -c 10000001
________________________________________________________
Executed in   17,33 secs    fish           external
   usr time   18,75 secs   16,00 micros   18,75 secs
   sys time    9,92 secs  979,00 micros    9,92 secs
________________________________________________________
Executed in   17,21 secs    fish           external
   usr time   18,58 secs  332,00 micros   18,58 secs
   sys time    9,80 secs  217,00 micros    9,80 secs
________________________________________________________
Executed in   16,92 secs    fish           external
   usr time   18,02 secs  511,00 micros   18,02 secs
   sys time    9,91 secs  328,00 micros    9,91 secs
________________________________________________________
Executed in   17,48 secs    fish           external
   usr time   18,84 secs    0,00 millis   18,84 secs
   sys time   10,01 secs    2,40 millis   10,01 secs
________________________________________________________
Executed in   17,41 secs    fish           external
   usr time   18,74 secs    1,01 millis   18,74 secs
   sys time    9,88 secs    0,02 millis    9,88 secs
```

## 32bits with no verbose

```bash
time echo q | cargo run --release -- -f  test/sc32win_donut.bin -c 10000001
________________________________________________________
Executed in  508,03 millis    fish           external
   usr time  411,96 millis    0,00 millis  411,96 millis
   sys time   92,59 millis    1,82 millis   90,77 millis
________________________________________________________
Executed in  500,24 millis    fish           external
   usr time  408,82 millis  401,00 micros  408,42 millis
   sys time   87,36 millis  271,00 micros   87,09 millis
________________________________________________________
Executed in  512,79 millis    fish           external
   usr time  423,06 millis    0,48 millis  422,59 millis
   sys time   86,23 millis    1,31 millis   84,92 millis
________________________________________________________
Executed in  511,26 millis    fish           external
   usr time  417,94 millis    0,00 millis  417,94 millis
   sys time   89,04 millis    1,05 millis   87,99 millis
________________________________________________________
Executed in  527,62 millis    fish           external
   usr time  445,11 millis  301,00 micros  444,81 millis
   sys time   78,18 millis  210,00 micros   77,97 millis
```

## 64bits with verbose  (some winapi overhead)

```bash
time echo q | cargo run --release -- -f test/sc64win_metasploit.bin -6 -vv -c 1000001
________________________________________________________
Executed in    1,50 secs    fish           external
   usr time    1,14 secs    0,14 millis    1,14 secs
   sys time    1,12 secs    1,08 millis    1,12 secs
________________________________________________________
Executed in    1,50 secs    fish           external
   usr time    1,12 secs    0,00 micros    1,12 secs
   sys time    1,14 secs  842,00 micros    1,14 secs
________________________________________________________
Executed in    1,46 secs    fish           external
   usr time    1,06 secs    0,07 millis    1,06 secs
   sys time    1,11 secs    1,04 millis    1,11 secs
________________________________________________________
Executed in    1,48 secs    fish           external
   usr time    1,08 secs    0,00 millis    1,08 secs
   sys time    1,15 secs    1,65 millis    1,14 secs
________________________________________________________
Executed in    1,42 secs    fish           external
   usr time    1,07 secs    0,00 micros    1,07 secs
   sys time    1,08 secs  888,00 micros    1,08 secs
```

## 64bits with no verbose  (some winapi overhead)

```bash
time echo q | cargo run --release -- -f test/sc64win_metasploit.bin -6  -c 10000001
________________________________________________________
Executed in  462,43 millis    fish           external
   usr time  372,54 millis    0,41 millis  372,14 millis
   sys time   87,08 millis    1,28 millis   85,80 millis
________________________________________________________
Executed in  476,20 millis    fish           external
   usr time  386,08 millis  742,00 micros  385,34 millis
   sys time   87,85 millis    0,00 micros   87,85 millis
________________________________________________________
Executed in  471,80 millis    fish           external
   usr time  378,54 millis  339,00 micros  378,20 millis
   sys time   91,81 millis  244,00 micros   91,56 millis
________________________________________________________
Executed in  473,17 millis    fish           external
   usr time  380,33 millis    0,00 millis  380,33 millis
   sys time   90,44 millis    1,59 millis   88,84 millis
________________________________________________________
Executed in  482,94 millis    fish           external
   usr time  365,06 millis  260,00 micros  364,80 millis
   sys time   98,44 millis  190,00 micros   98,25 millis
```

## Max-Speed 18M instructions

```bash
time echo q | cargo run --release -- -f  test/sc32win_donut.bin -c 18000001
________________________________________________________
Executed in  778,99 millis    fish           external
   usr time  679,30 millis  316,00 micros  678,99 millis
   sys time   96,05 millis  196,00 micros   95,86 millis
________________________________________________________
Executed in  796,61 millis    fish           external
   usr time  700,19 millis  453,00 micros  699,74 millis
   sys time   92,51 millis  283,00 micros   92,23 millis
________________________________________________________
Executed in  776,77 millis    fish           external
   usr time  690,63 millis  394,00 micros  690,24 millis
   sys time   81,80 millis  247,00 micros   81,56 millis
________________________________________________________
Executed in  801,97 millis    fish           external
   usr time  711,06 millis    0,00 micros  711,06 millis
   sys time   86,84 millis  592,00 micros   86,25 millis
________________________________________________________
Executed in  783,98 millis    fish           external
   usr time  711,61 millis  292,00 micros  711,32 millis
   sys time   68,41 millis  184,00 micros   68,22 millis
```

## 18M with verbose:
```bash
time echo q | cargo run --release -- -f  test/sc32win_donut.bin -c 18000001 -vv  
________________________________________________________
Executed in   32,53 secs    fish           external
   usr time   36,35 secs  407,00 micros   36,35 secs
   sys time   17,91 secs  244,00 micros   17,91 secs
________________________________________________________
Executed in   33,00 secs    fish           external
   usr time   37,19 secs    0,00 millis   37,19 secs
   sys time   17,92 secs    1,74 millis   17,92 secs
________________________________________________________
Executed in   27,21 secs    fish           external
   usr time   31,60 secs  266,00 micros   31,60 secs
   sys time   16,92 secs  164,00 micros   16,92 secs
________________________________________________________
Executed in   32,80 secs    fish           external
   usr time   36,94 secs    0,00 micros   36,94 secs
   sys time   17,93 secs  662,00 micros   17,93 secs
________________________________________________________
Executed in   32,82 secs    fish           external
   usr time   36,86 secs  307,00 micros   36,86 secs
   sys time   17,98 secs  194,00 micros   17,98 secs

``` 

