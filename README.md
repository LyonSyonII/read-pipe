# read_pipe
Simple utility to read pipes from stdin.

## Usage
```rust
if let Some(pipe) = read_pipe::read_pipe() {
    println!("User has piped \"{pipe}\" to the program.")
}
```

## Examples
---
Input:
```bash
echo "Really interesting pipe" | cargo run
```
Output:
```
Some("Really interesting pipe")
```
---
Input:
```bash
cargo run
```
Output:
```
None
```
---
