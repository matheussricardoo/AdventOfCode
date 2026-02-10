fn main() {
    let raw_text = include_str!("input.txt");
    let mut good_strings = 0;

    for line in raw_text.lines() {
        let mut vowels = 0;
        let mut has_double = false;
        let mut has_forbidden = false;
        let mut last_char = '\0';
        for ch in line.chars() {
            if ch == 'a' || ch == 'e' || ch == 'i' || ch == 'o' || ch == 'u' {
                vowels += 1
            }
            if ch == last_char {
                has_double = true
            }
            if last_char == 'a' && ch == 'b'
                || last_char == 'c' && ch == 'd'
                || last_char == 'p' && ch == 'q'
                || last_char == 'x' && ch == 'y'
            {
                has_forbidden = true
            }
            last_char = ch;
        }
        if vowels >= 3 && has_double == true && has_forbidden == false {
            good_strings += 1
        }
    }
    println!("{}", good_strings);
}
