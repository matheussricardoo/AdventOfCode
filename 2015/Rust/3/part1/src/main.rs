use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let raw_text = include_str!("input.txt");

    let mut houses: HashMap<Point, i32> = HashMap::new();

    let point = Point { x: 0, y: 0 };

    let mut _x = point.x;
    let mut _y = point.y;

    houses.insert(Point { x: 0, y: 0 }, 1);

    for (_i, c) in raw_text.chars().enumerate() {
        match c {
            '^' => _y += 1,
            'v' => _y -= 1,
            '>' => _x += 1,
            '<' => _x -= 1,
            _ => println!("This character is not acept."),
        }
        let present = Point { x: _x, y: _y };
        *houses.entry(present).or_insert(0) += 1;
    }
    println!(
        "How many houses receive at least one present? {}",
        houses.len()
    );
}
