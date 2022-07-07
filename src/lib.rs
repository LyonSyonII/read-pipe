use std::io::Read;

/// Reads a pipe from stdin and returns it as a `String`.
/// # Errors
/// Will return `None` if there is no pipe handled to the program or some error occurs while reading stdin (see [`std::io::Read::read_to_string`](https://doc.rust-lang.org/std/io/trait.Read.html#method.read_to_string) for more information).
fn read_pipe() -> Option<String> {
    if atty::isnt(atty::Stream::Stdin) {
        let mut buf = String::new();
        std::io::stdin().read_to_string(&mut buf).ok()?;
        Some(buf)
    } else {
        None
    }
}

/// Reads pipe from stdin, splits it at whitespace and returns it as a Vec<String>.
/// 
/// See [`read_pipe`]() for more information.
fn read_pipe_split_whitespace() -> Option<Vec<String>> {
    let read = read_pipe()?;
    Some(read.split_whitespace().map(|s| s.into()).collect())
}