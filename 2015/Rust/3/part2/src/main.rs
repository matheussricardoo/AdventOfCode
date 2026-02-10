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

    let mut _sx = point.x;
    let mut _sy = point.y;

    let mut _rx = point.x;
    let mut _ry = point.y;

    houses.insert(Point { x: 0, y: 0 }, 2);

    for (i, c) in raw_text.chars().enumerate() {
        let (x_current, y_current) = if i % 2 == 0 {
            (&mut _sx, &mut _sy)
        } else {
            (&mut _rx, &mut _ry)
        };
        match c {
            '^' => *y_current += 1,
            'v' => *y_current -= 1,
            '>' => *x_current += 1,
            '<' => *x_current -= 1,
            _ => println!("This character is not acept."),
        }
        houses.insert(
            Point {
                x: *x_current,
                y: *y_current,
            },
            1,
        );
    }
    println!(
        "How many houses receive at least one present? {}",
        houses.len()
    );
}
