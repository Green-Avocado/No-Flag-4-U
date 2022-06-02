# Somewhat Safe Glibc Wrappers

## Features

- Memory is never freed after `__libc_start_main()` to prevent UAF
- Safely wraps non-constant format strings for `printf()`

## Build

Compile with:

```
cargo build --release
```

Optionally, disable the use of the `%n` conversion specifier in the `printf()` family with:

```
cargo rustc --release -- --cfg disallow_dangerous_printf
```

## Usage

```
ld.so --preload libsuper_safe_glibc_wrappers.so [COMMAND]
```
