# super-safe-glibc-wrappers

## Features

- Memory is never freed after `__libc_start_main()` to prevent UAF
- Disabled non-constant format strings for `printf()`

## Usage

```
ld.so --preload libsuper_safe_glibc_wrappers.so [COMMAND]
```
