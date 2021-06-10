fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_clone<T: PartialOrd + Clone>(list: &[T]) -> T {
    let mut largest = list[0].clone();

    for item in list {
        if *item > largest {
            largest = item.clone();
        }
    }

    largest
}

fn main() {
    let sequence = [
        String::from("abc"),
        String::from("abcd"),
        String::from("ab"),
    ];

    let largest_string = largest_clone(&sequence);
    println!("largest = {}", largest_string);
}
