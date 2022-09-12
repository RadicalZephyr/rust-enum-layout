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

#[rustfmt::skip]
enum R257 {
   V1, V2, V3, V4, V5, V6, V7, V8, V9, V10, V11, V12, V13, V14, V15, V16, V17, V18, V19, V20, V21, V22, V23, V24, V25, V26, V27, V28, V29, V30, V31, V32, V33, V34, V35, V36, V37, V38, V39, V40, V41, V42, V43, V44, V45, V46, V47, V48, V49, V50, V51, V52, V53, V54, V55, V56, V57, V58, V59, V60, V61, V62, V63, V64, V65, V66, V67, V68, V69, V70, V71, V72, V73, V74, V75, V76, V77, V78, V79, V80, V81, V82, V83, V84, V85, V86, V87, V88, V89, V90, V91, V92, V93, V94, V95, V96, V97, V98, V99, V100, V101, V102, V103, V104, V105, V106, V107, V108, V109, V110, V111, V112, V113, V114, V115, V116, V117, V118, V119, V120, V121, V122, V123, V124, V125, V126, V127, V128, V129, V130, V131, V132, V133, V134, V135, V136, V137, V138, V139, V140, V141, V142, V143, V144, V145, V146, V147, V148, V149, V150, V151, V152, V153, V154, V155, V156, V157, V158, V159, V160, V161, V162, V163, V164, V165, V166, V167, V168, V169, V170, V171, V172, V173, V174, V175, V176, V177, V178, V179, V180, V181, V182, V183, V184, V185, V186, V187, V188, V189, V190, V191, V192, V193, V194, V195, V196, V197, V198, V199, V200, V201, V202, V203, V204, V205, V206, V207, V208, V209, V210, V211, V212, V213, V214, V215, V216, V217, V218, V219, V220, V221, V222, V223, V224, V225, V226, V227, V228, V229, V230, V231, V232, V233, V234, V235, V236, V237, V238, V239, V240, V241, V242, V243, V244, V245, V246, V247, V248, V249, V250, V251, V252, V253, V254, V255, V256,V257,
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

enum Comp1R257 {
    A(R257),
}

enum Comp2R1 {
    A(R1),
    B(R1),
}

enum Comp2R2 {
    A(R2),
    B(R2),
}

enum Comp2R3 {
    A(R3),
    B(R3),
}

enum Comp2R257 {
    A(R257),
    B(R257),
}

enum Comp2R0R1 {
    A(R0),
    B(R1),
}

enum Comp2R1R2 {
    A(R1),
    B(R2),
}

enum Comp2R2R3 {
    A(R2),
    B(R3),
}

enum Comp2R0R257 {
    A(R0),
    B(R257),
}

enum Comp2R1R257 {
    A(R1),
    B(R257),
}

enum Comp2R2R257 {
    A(R2),
    B(R257),
}

enum Comp2R3R257 {
    A(R3),
    B(R257),
}

enum Comp3R0 {
    A(R0),
    B(R0),
    C(R0),
}

enum Comp3R1 {
    A(R1),
    B(R1),
    C(R1),
}

enum Comp3R2 {
    A(R2),
    B(R2),
    C(R2),
}

enum Comp3R3 {
    A(R3),
    B(R3),
    C(R3),
}

enum Comp3R257 {
    A(R257),
    B(R257),
    C(R257),
}

enum Comp3R0R257 {
    A(R0),
    B(R0),
    C(R257),
}

enum Comp3R1R257 {
    A(R1),
    B(R1),
    C(R257),
}

enum Comp3R2R257 {
    A(R2),
    B(R2),
    C(R257),
}

enum Comp3R3R257 {
    A(R3),
    B(R3),
    C(R257),
}

macro_rules! print_sizes {
    { $heading:literal, $( $ty_name:ty ),* $(,)? } => {
        println!(concat!("\n", $heading));
        $(
            println!("size of {}:\t{} bytes", stringify!($ty_name), size_of::<$ty_name>());
        )*
    };
}

pub fn main() {
    print_sizes!("Simple enums (no nested types)", R0, R1, R2, R3, R257);

    print_sizes!(
        "Composite new-type enums (single variant)",
        Comp1R0,
        Comp1R1,
        Comp1R2,
        Comp1R3,
        Comp1R257,
    );

    print_sizes!(
        "Composite enums with two variants",
        Comp2R1,
        Comp2R2,
        Comp2R3,
        Comp2R257,
    );

    print_sizes!(
        "Composite enums two different variants",
        Comp2R0R1,
        Comp2R1R2,
        Option<R2>,
        Comp2R2R3,
        Comp2R0R257,
        Comp2R1R257,
        Comp2R2R257,
        Comp2R3R257,
    );

    print_sizes!(
        "Composite enums with 3 variants",
        Comp3R0,
        Comp3R1,
        Comp3R2,
        Comp3R3,
        Comp3R257
    );

    print_sizes!(
        "Composite enums with variants, two different inner types",
        Comp3R0R257,
        Comp3R1R257,
        Comp3R2R257,
        Comp3R3R257,
    );
}
