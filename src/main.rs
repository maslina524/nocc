#![no_std]

mod io;
use io::Io;

fn main() {
    let io = Io::new();
    io.print("Hello World!");
}
