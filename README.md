# Less Dangerous Libc

Provides wrappers for some libc functions to mitigate security risks.

## Features

- Memory is never freed after `__libc_start_main()` to prevent UAF
- Safely wraps non-constant format strings for `printf()`

## Build

Compile with:

```
cargo build --release
```

Currently supports only `x86_64-unknown-linux-gnu`.

### Options

By default, this library aims to provide as much safety as possible without breaking most programs.
However, in some cases it may be desirable to disable or enable certain mitigations.

Enable or disable optional features by passing flags to rustc:

```
cargo rustc --release -- --cfg [OPTION]
```
| OPTION | Description |
|--------------------------------|-----------------------------------------------------------------|
| `enable_n_directive_filter`    | disallow `%n` conversion specifiers in format strings           |
| `enable_require_logger`        | panic if unable to connect to the logger                        |
| `disable_format_string_hooks`  | do not hook libc functions belonging to the `printf()` family   |
| `disable_heap_hooks`           | do not hook libc functions for managing dynamic memory          |

## Usage

Call `ld.so` with the `--preload` flag (only affects the original process):

```bash
ld.so --preload libsuper_safe_glibc_wrappers.so [COMMAND]
```

OR

Set the `LD_PRELOAD` environment variable (affects child processes):

```bash
export LD_PRELOAD=libsuper_safe_glibc_wrappers.so
[COMMAND]
```

### Logging

A TCP listener must be active to receive logs.
One is provided in the `log_server` binary.
