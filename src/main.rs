#[macro_use]
extern crate scroll;

#[derive(Pread)]
struct A {
    a: u32,
}

#[derive(Pread)]
struct B {
    b: [A; 4],
}

fn main() {
}
