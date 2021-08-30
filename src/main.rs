// const A_CONST: i32 = 42;

fn avg(a: u32, b: u32) -> u32 {
    (a & b) + ((a ^ b) >> 1)
}


fn main() {
    assert_eq!(avg(4294967295, 4294967295), 4294967295);
    assert_eq!(avg(0, 0), 0);
    assert_eq!(avg(10, 20), 15);
    assert_eq!(avg(4294967295, 1), 2147483648);
    println!("passed")
}

