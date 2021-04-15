const COUNTERS: [&str;12] = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "nineth", "tenth", "eleventh", "twelfth"];
const LYRICS: [&str;12] = ["A partridge in a pear tree", "Two turtle doves", "Three French hens", "Four calling birds", "Five gold rings", "Six geese a laying", "Seven swans a swimming", "Eight maids a milking", "Nine ladies dancing", "Ten lords a leaping", "Eleven pipers piping", "Twelve drummers drumming"];

fn main() {
    for counter in 0..COUNTERS.len() {
        println!("On the {} day of Christmas my true love gave to me", COUNTERS[counter]);
        for lyric_counter in (0..counter).rev() {
            println!("{}", LYRICS[lyric_counter]);
        }
        println!("\n");
    }
}
