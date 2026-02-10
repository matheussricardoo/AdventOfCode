use std::{fs, path::Path};

fn main() {
    let file_path = Path::new("src/input.txt");
    let raw_text = fs::read_to_string(file_path).unwrap_or_default();

    let mut good_strings = 0;

    for line in raw_text.lines() {
        let chars: Vec<char> = line.chars().collect();
        let len = chars.len();

        if len < 3 {
            continue;
        }

        let mut has_sandwich = false;
        let mut letter_repeat = false;
        for i in 0..(len - 2) {
            if chars[i] == chars[i + 2] {
                has_sandwich = true;
            }
            for j in (i + 2)..(len - 1) {
                if chars[i] == chars[j] && chars[i + 1] == chars[j + 1] {
                    letter_repeat = true;
                }
            }
        }
        if letter_repeat && has_sandwich {
            good_strings += 1;
        }
    }
    println!("{}", good_strings);
}
