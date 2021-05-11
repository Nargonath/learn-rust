fn main() {
    println!("1 - 2 = {}", 1i32 - 2);

    println!("0011 and 0101 is {:04b}", 0b0011 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011 ^ 0b0101);
    println!("1 << 5 is {:08b}", 1u32 << 5);
    // divides by 2
    println!("(10 + 4) >> 1 is {}", (10 + 4) >> 1);
    println!("(11 + 4) >> 1 is {}", (11 + 4) >> 1);
}
