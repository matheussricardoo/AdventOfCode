fn unescape(input: &str) -> usize {
    let content = &input[1..input.len() - 1];
    let mut count = 0;
    let mut chars = content.chars();

    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('x') => {
                    chars.next();
                    chars.next();
                    count += 1;
                }
                Some(_) => {
                    count += 1;
                }
                None => {}
            }
        } else {
            count += 1;
        }
    }
    count
}

fn main() {
    let raw_text = include_str!("input.txt");
    let mut code_count = 0;
    let mut mem_string_count = 0;
    for line in raw_text.lines() {
        code_count += line.len();
        mem_string_count += unescape(line);
    }
    println!("Total difference: {}", code_count - mem_string_count);
}
