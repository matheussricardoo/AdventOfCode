use md5;

fn main() {
    let input = "iwrupvqb";
    let mut count = 0;
    loop {
        let sum_input = format!("{}{}", input, count);
        let sum_bytes = sum_input.as_bytes();

        let digest = md5::compute(sum_bytes);
        let convert_hash = format!("{:x}", digest);
        if convert_hash.starts_with("00000") {
            print!("{}\n", count);
            break;
        } else {
            count += 1
        }
    }
}
