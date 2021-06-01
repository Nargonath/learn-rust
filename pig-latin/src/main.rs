fn is_in_array(haystack: &[char], needle: Option<char>) -> bool {
    match needle {
        None => false,
        Some(ndl) => {
            for c in haystack {
                if *c == ndl {
                    return true;
                }
            }
            false
        }
    }
}

fn convert_to_pig_latin(string: &String) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u', 'y'];
    let first_char = string.chars().nth(0);
    let start_with_vowel = is_in_array(&vowels, first_char);

    if start_with_vowel {
        format!("{}-hay", string)
    } else {
        match first_char {
            None => String::from(""),
            Some(c) => format!("{}-{}ay", &string[1..], &c),
        }
    }
}

fn idiomatic_pig_latin(string: &str) -> String {
    if string.is_empty() {
        return String::new();
    }

    let first_char = string.chars().nth(0).unwrap();
    if "aeiouy".contains(first_char) {
        format!("{}-hay", string)
    } else {
        format!("{}-{}ay", &string[first_char.len_utf8()..], &first_char)
    }
}

fn main() {
    let test = String::from("car");

    println!("initial string = {}", &test);
    println!("translated string = {}", &convert_to_pig_latin(&test));
    println!(
        "idiomatic translated string = {}",
        &idiomatic_pig_latin(&test)
    );
}
