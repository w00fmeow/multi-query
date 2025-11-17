## multi-query

Multi-database query executor with unified JSON output

Archives of precompiled binaries for multi-query are available for macOS and Linux on [every release](https://github.com/w00fmeow/multi-query/releases).

[![Tests](https://github.com/w00fmeow/multi-query/workflows/tests/badge.svg)](https://github.com/w00fmeow/multi-query)

Dual-licensed under MIT or the [UNLICENSE](https://unlicense.org/).

## Installation

Archives are available on [every release](https://github.com/w00fmeow/multi-query/releases) as well as `.deb` files for Linux.

Autocomplete for arguments and man pages are included.

### (brew tap) Apple & Linux

Tap the repository by running this command:

```bash
brew tap w00fmeow/multi-query https://github.com/w00fmeow/multi-query
```

and install the package:

```bash
brew install multi-query
```

### .deb file for Linux

Download `.deb` file from [latest release](https://github.com/w00fmeow/multi-query/releases) and install it using one of the commands:

```bash
sudo apt install ./path/to/multi-query.deb
```

or

```bash
sudo dpkg -i ./path/to/multi-query.deb
```

## Developing

- `cargo run` - for development
- `cargo test` - to run tests
- `cargo build -r` - to build in release mode
