#![allow(dead_code)]

use std::mem::size_of;

enum R0 {}

enum R1 {
    A,
}

enum R2 {
    A,
    B,
}

enum R3 {
    A,
    B,
    C,
}

enum Comp1R0 {
    A(R0),
}

enum Comp1R1 {
    A(R1),
}

enum Comp1R2 {
    A(R2),
}

enum Comp1R3 {
    A(R3),
}

macro_rules! print_sizes {
    { $( $ty_name:ty ),* } => {
        $(
            println!("size of {}: {} bytes", stringify!($ty_name), size_of::<$ty_name>());
        )*
    };
}

pub fn main() {
    print_sizes!(R0, R1, R2, R3);
    print_sizes!(Comp1R0, Comp1R1, Comp1R2, Comp1R3);
}
