fn main() {
    use std::collections::HashMap;

    let mut integers = vec![10, 5, 29, 1, 5];
    let mut mean = 0;
    let mut most_often = HashMap::new();

    integers.sort();

    for int in &integers {
        mean += int;
        let count = most_often.entry(int).or_insert(0);
        *count += 1;
    }
    mean /= integers.len();
    let half_vec_length = integers.len() >> 1;
    let median = integers[half_vec_length - 1];

    let mut mode: (usize, usize) = (0, 0);
    for (key, count) in most_often.iter() {
        if mode.1 < *count {
            mode = (**key, *count);
        }
    }

    println!("mean = {}", mean);
    println!("median = {}", median);
    println!("mode = {:?}", mode);
}
