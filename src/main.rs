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

enum Comp2R0 {
    A(R0),
    B(R0),
}

enum Comp2R1 {
    A(R1),
    B(R1),
}

enum Comp2R0R1 {
    A(R0),
    B(R1),
}

macro_rules! print_sizes {
    { $heading:literal, $( $ty_name:ty ),* $(,)? } => {
        println!(concat!("\n", $heading));
        $(
            println!("size of {}: {} bytes", stringify!($ty_name), size_of::<$ty_name>());
        )*
    };
}

pub fn main() {
    print_sizes!("Simple enums (no nested types)", R0, R1, R2, R3);

    print_sizes!(
        "Composite new-type enums (single variant)",
        Comp1R0,
        Comp1R1,
        Comp1R2,
        Comp1R3,
    );

    print_sizes!(
        "Composite enums (multiple variants)",
        Comp2R0,
        Comp2R1,
        Comp2R0R1,
    );
}
