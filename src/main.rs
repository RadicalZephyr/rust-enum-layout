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

macro_rules! print_sizes {
    { $( $ty_name:ty ),* } => {
        $(
            println!("size of {}: {} bytes", stringify!($ty_name), size_of::<$ty_name>());
        )*
    };
}

pub fn main() {
    print_sizes!(R0, R1, R2, R3);
}
