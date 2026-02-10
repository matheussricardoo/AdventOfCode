fn main() {
    let raw_text = include_str!("input.txt");

    let mut total_paper = 0;
    let mut total_ribbon = 0;
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
        let volume = list[0] * list[1] * list[2];
        let wrap = 2 * list[0] + 2 * list[1];

        total_ribbon += volume + wrap
    }
    println!("Result: {total_paper}");
    println!("Total Ribbon: {total_ribbon}");
}
