use std::{fs, path::Path};

fn main() {
    println!("Hello, world!");
    
    let file_path = Path::new("src/input.txt");
    
    let contests = fs::read_to_string(file_path);
    println!("With text:\n{:?}",contests);
}
