# Somewhat Safe Glibc Wrappers

## Features

- Memory is never freed after `__libc_start_main()` to prevent UAF
- Safely wraps non-constant format strings for `printf()`

## Usage

```
ld.so --preload libsuper_safe_glibc_wrappers.so [COMMAND]
```
