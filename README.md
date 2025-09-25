# dp2

Print files in hex and ascii.

## Examples

Dump file:

```
$ dp Cargo.toml
----------------------------------------------------------------------------- Cargo.toml
00000000:  5b 70 61 63 6b 61 67 65  5d 0a 6e 61 6d 65 20 3d  [package].name =
00000010:  20 22 64 70 22 0a 76 65  72 73 69 6f 6e 20 3d 20   "dp".version =
00000020:  22 30 2e 30 2e 32 22 0a  64 65 73 63 72 69 70 74  "0.0.2".descript
00000030:  69 6f 6e 20 3d 20 22 50  72 69 6e 74 20 66 69 6c  ion = "Print fil
00000040:  65 73 20 69 6e 20 68 65  78 20 61 6e 64 20 61 73  es in hex and as
00000050:  63 69 69 22 0a 6c 69 63  65 6e 73 65 20 3d 20 22  cii".license = "
00000060:  4d 49 54 22 0a 65 64 69  74 69 6f 6e 20 3d 20 22  MIT".edition = "
00000070:  32 30 32 34 22 0a 0a 5b  64 65 70 65 6e 64 65 6e  2024"..[dependen
00000080:  63 69 65 73 5d 0a 0a 5b  70 72 6f 66 69 6c 65 2e  cies]..[profile.
00000090:  72 65 6c 65 61 73 65 5d  0a 6f 70 74 2d 6c 65 76  release].opt-lev
000000a0:  65 6c 20 3d 20 33 0a 64  65 62 75 67 20 3d 20 66  el = 3.debug = f
000000b0:  61 6c 73 65 0a 73 74 72  69 70 20 3d 20 74 72 75  alse.strip = tru
000000c0:  65 0a 6c 74 6f 20 3d 20  74 72 75 65 0a 63 6f 64  e.lto = true.cod
000000d0:  65 67 65 6e 2d 75 6e 69  74 73 20 3d 20 31 0a     egen-units = 1.

Cargo.toml: 223 bytes, 0.2178 KiB 0.0002 MiB
```

Dump stdin:

```
$ dp
hi mom
00000000:  68 69 20 6d 6f 6d 0a                              hi mom.
dp
00000007:  64 70 0a                                          dp.
xxd?
0000000a:  78 78 64 3f 0a                                    xxd?.
```

Dump files:

```
$ dp Cargo.toml Cargo.lock
----------------------------------------------------------------------------- Cargo.toml
00000000:  5b 70 61 63 6b 61 67 65  5d 0a 6e 61 6d 65 20 3d  [package].name =
00000010:  20 22 64 70 22 0a 76 65  72 73 69 6f 6e 20 3d 20   "dp".version =
00000020:  22 30 2e 30 2e 32 22 0a  64 65 73 63 72 69 70 74  "0.0.2".descript
00000030:  69 6f 6e 20 3d 20 22 50  72 69 6e 74 20 66 69 6c  ion = "Print fil
00000040:  65 73 20 69 6e 20 68 65  78 20 61 6e 64 20 61 73  es in hex and as
00000050:  63 69 69 22 0a 6c 69 63  65 6e 73 65 20 3d 20 22  cii".license = "
00000060:  4d 49 54 22 0a 65 64 69  74 69 6f 6e 20 3d 20 22  MIT".edition = "
00000070:  32 30 32 34 22 0a 0a 5b  64 65 70 65 6e 64 65 6e  2024"..[dependen
00000080:  63 69 65 73 5d 0a 0a 5b  70 72 6f 66 69 6c 65 2e  cies]..[profile.
00000090:  72 65 6c 65 61 73 65 5d  0a 6f 70 74 2d 6c 65 76  release].opt-lev
000000a0:  65 6c 20 3d 20 33 0a 64  65 62 75 67 20 3d 20 66  el = 3.debug = f
000000b0:  61 6c 73 65 0a 73 74 72  69 70 20 3d 20 74 72 75  alse.strip = tru
000000c0:  65 0a 6c 74 6f 20 3d 20  74 72 75 65 0a 63 6f 64  e.lto = true.cod
000000d0:  65 67 65 6e 2d 75 6e 69  74 73 20 3d 20 31 0a     egen-units = 1.

Cargo.toml: 223 bytes, 0.2178 KiB 0.0002 MiB
----------------------------------------------------------------------------- Cargo.lock
00000000:  23 20 54 68 69 73 20 66  69 6c 65 20 69 73 20 61  # This file is a
00000010:  75 74 6f 6d 61 74 69 63  61 6c 6c 79 20 40 67 65  utomatically @ge
00000020:  6e 65 72 61 74 65 64 20  62 79 20 43 61 72 67 6f  nerated by Cargo
00000030:  2e 0a 23 20 49 74 20 69  73 20 6e 6f 74 20 69 6e  ..# It is not in
00000040:  74 65 6e 64 65 64 20 66  6f 72 20 6d 61 6e 75 61  tended for manua
00000050:  6c 20 65 64 69 74 69 6e  67 2e 0a 76 65 72 73 69  l editing..versi
00000060:  6f 6e 20 3d 20 34 0a 0a  5b 5b 70 61 63 6b 61 67  on = 4..[[packag
00000070:  65 5d 5d 0a 6e 61 6d 65  20 3d 20 22 64 70 22 0a  e]].name = "dp".
00000080:  76 65 72 73 69 6f 6e 20  3d 20 22 30 2e 30 2e 32  version = "0.0.2
00000090:  22 0a                                             ".

Cargo.lock: 146 bytes, 0.1426 KiB 0.0001 MiB
```

Pipe output from other command to dumper:

```
$ printf '\x00\x01\x02\x03\x04\x05\x06\x07\x08\xae\xff\xea\xfb\xcc' | dp
00000000:  00 01 02 03 04 05 06 07  08 ae ff ea fb cc        .............
```
