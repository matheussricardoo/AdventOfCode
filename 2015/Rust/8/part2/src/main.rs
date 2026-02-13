fn encoded_size(input: &str) -> usize {
    let mut count = 2;

    for c in input.chars() {
        match c {
            '"' | '\\' => count += 2,
            _ => count += 1,
        }
    }
    count
}

fn main() {
    let raw_text = include_str!("input.txt");
    let mut code_count = 0;
    let mut total_encoded = 0;

    for line in raw_text.lines() {
        code_count += line.len();
        total_encoded += encoded_size(line);
    }
    println!("Total difference: {}", total_encoded - code_count);
}
