use std::{fs, path::Path};

fn main() {
    let file_path = Path::new("src/input.txt");

    let raw_text = fs::read_to_string(file_path).unwrap();

    let mut total_paper = 0;
    for line in raw_text.lines() {
        let dimensions: Vec<i32> = line.split('x').map(|s| s.parse().unwrap()).collect();

        let l = dimensions[0];
        let w = dimensions[1];
        let h = dimensions[2];

        let mut list = vec![l, w, h];
        list.sort();

        let area = 2 * l * w + 2 * w * h + 2 * h * l;
        let slack = list[0] * list[1];

        total_paper += area + slack;
    }
    println!("Result: {total_paper}");
}
