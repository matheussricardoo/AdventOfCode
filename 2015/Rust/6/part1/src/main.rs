use regex::Regex;

fn main() {
    let raw_text = include_str!("input.txt");
    let mut total_lights = 0;
    let mut grid: [[usize; 1000]; 1000] = [[0; 1000]; 1000];
    let re = Regex::new(r"(turn on|turn off|toggle) (\d+),(\d+) through (\d+),(\d+)").unwrap();
    for line in raw_text.lines() {
        if let Some(caps) = re.captures(line) {
            let command = &caps[1];
            let x1: usize = caps[2].parse().unwrap();
            let y1: usize = caps[3].parse().unwrap();
            let x2: usize = caps[4].parse().unwrap();
            let y2: usize = caps[5].parse().unwrap();

            let state = if command == "turn on" {
                "ON"
            } else if command == "turn off" {
                "OFF"
            } else {
                "REVERT"
            };

            for i in x1..=x2 {
                for j in y1..=y2 {
                    if state == "ON" {
                        grid[i][j] = 1;
                    } else if state == "OFF" {
                        grid[i][j] = 0
                    } else {
                        if grid[i][j] == 1 {
                            grid[i][j] = 0
                        } else {
                            grid[i][j] = 1
                        }
                    }
                }
            }
        }
    }
    for i in 0..(1000) {
        for j in 0..(1000) {
            if grid[i][j] == 1 {
                total_lights += 1
            }
        }
    }
    println!("Total lights: {}", total_lights)
}
