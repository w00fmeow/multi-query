## multi-query

Multi-database query executor with unified JSON output

Archives of precompiled binaries for multi-query are available for macOS and Linux on [every release](https://github.com/w00fmeow/multi-query/releases).

[![Tests](https://github.com/w00fmeow/multi-query/workflows/tests/badge.svg)](https://github.com/w00fmeow/multi-query)

Dual-licensed under MIT or the [UNLICENSE](https://unlicense.org/).

## Usage

```console
Multi-database query executor with unified JSON output

Usage: multi-query [OPTIONS] --query <FILE>

Options:
  -q, --query <FILE>                  Path to SQL query file to execute across all databases
  -c, --connection-string <NAME,URI>  Database connection string in format: <name>,<uri>

                                      Examples:
                                        prod-db,postgresql://user:pass@localhost/dbname

                                      Can be specified multiple times to query multiple databases
      --config <FILE>                 Path to config file [default: ~/.multi-query/config.json]
      --generate-config               Generate a default config file at the config path
  -h, --help                          Print help
  -V, --version                       Print version
```

### Basic Examples

**Execute a query on a single database:**

```bash
multi-query --query my-query.sql --connection-string prod-db,postgresql://user:pass@localhost/dbname
```

**Execute a query on multiple databases:**

```bash
multi-query \
    --query my-query.sql \
    --connection-string prod-region-1,postgresql://user:pass@localhost/dbname \
    --connection-string prod-region-2,postgresql://user:pass@localhost:5433/dbname
```

**Example output:**

```json
{"id":1,"name":"Alice","status":"active","db_name":"prod-region-1"}
{"id":2,"name":"Bob","status":"active","db_name":"prod-region-1"}
{"id":1,"name":"Charlie","status":"active","db_name":"prod-region-2"}
```

### Using a Config File

Instead of passing connection strings every time, you can load them from a config file.

**Generate a default config file:**

```bash
multi-query --generate-config
```

This creates a default configuration file at `~/.multi-query/config.json`.

**Edit the config file with your connection strings:**

```json
{
  "connection_strings": [
    {
      "name": "prod-region-1",
      "uri": "postgresql://user:pass@localhost/dbname"
    },
    {
      "name": "prod-region-2",
      "uri": "postgresql://user:pass@localhost:5433/dbname"
    }
  ]
}
```

**Now you can run queries without specifying connection strings:**

```bash
multi-query --query my-query.sql
```

This queries both `prod-region-1` and `prod-region-2` from the config file.

**Use a custom config file location:**

```bash
multi-query --query my-query.sql --config ./my-databases.json
```

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
