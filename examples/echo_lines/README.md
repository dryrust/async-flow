# Echo Lines Example

This is a trivial two-block example program that reads bytes from standard
input (stdin) and writes them to standard output (stdout).

Since stdin and stdout are line buffered in the terminal, effectively this
program ends up echoing lines of text without needing to decode/encode lines.

## Usage

```bash
cargo run --example echo_lines
```
