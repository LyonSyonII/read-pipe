#![doc = include_str!("../README.md")]

use std::io::{IsTerminal, Read};

/// Reads a pipe from stdin and returns it as a `String`.
/// # Errors
/// Will return `None` if there is no pipe handled to the program or some error occurs while reading stdin (see [`std::io::Read::read_to_string`](https://doc.rust-lang.org/std/io/trait.Read.html#method.read_to_string) for more information).
pub fn read_pipe() -> Option<String> {
    let mut buf = String::new();
    if !std::io::stdin().is_terminal() {
        std::io::stdin().read_to_string(&mut buf).ok()?;
    }
    
    (!buf.is_empty()).then_some(buf.trim().into())
}

/// Reads pipe from stdin, splits it by whitespace and returns it as a `Vec<String>`.
/// 
/// See [`read_pipe`] for more information.
pub fn read_pipe_split_whitespace() -> Option<Vec<String>> {
    let read = read_pipe()?;
    Some(read.split_whitespace().map(|s| s.into()).collect())
}