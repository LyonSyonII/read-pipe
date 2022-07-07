mod lib;

fn main() {
    let res = lib::read_pipe();
    println!("{:?}", res);
}