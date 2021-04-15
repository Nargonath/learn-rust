fn main() {
    for number in 1..11 {
        println!("Here is the {}th number of the fibonacci suite: {}", number, fibo(number));
    }
}

fn fibo(n: u32) -> u32 {
    if n < 3 {
        n - 1
    } else {
        fibo(n - 1) + fibo(n - 2)
    }
}

