use std::collections::HashMap;

fn main() {
    let raw_text = include_str!("input.txt");
    let mut dic: HashMap<String, String> = HashMap::new();
    let mut memo: HashMap<String, u16> = HashMap::new();
    for line in raw_text.lines() {
        let parts: Vec<&str> = line.split(" -> ").collect();
        let instruction = parts[0];
        let wire = parts[1];
        dic.insert(wire.to_string(), instruction.to_string());
    }

    let result = get_value("a", &mut memo, &dic);

    println!("Signal on wire a: {}", result);

    let mut memo: HashMap<String, u16> = HashMap::new();
    dic.insert("b".to_string(), result.to_string());
    let result_part2 = get_value("a", &mut memo, &dic);
    println!("Signal on wire new a: {}", result_part2);
}

fn get_value(wire: &str, memo: &mut HashMap<String, u16>, dic: &HashMap<String, String>) -> u16 {
    if let Ok(num) = wire.parse::<u16>() {
        return num;
    }

    if let Some(&val) = memo.get(wire) {
        return val;
    }
    let instruction = dic.get(wire).expect("String not found in dictionary");
    let parts: Vec<&str> = instruction.split_whitespace().collect();
    let result: u16;

    if parts.len() == 1 {
        result = get_value(parts[0], memo, dic);
    } else if parts.len() == 2 {
        let val = get_value(parts[1], memo, dic);
        result = !val;
    } else {
        let left = get_value(parts[0], memo, dic);
        let right = get_value(parts[2], memo, dic);

        result = match parts[1] {
            "AND" => left & right,
            "OR" => left | right,
            "LSHIFT" => left << right,
            "RSHIFT" => left >> right,
            _ => panic!("Invalid operation: {}", parts[1]),
        };
    }
    memo.insert(wire.to_string(), result);
    result
}
