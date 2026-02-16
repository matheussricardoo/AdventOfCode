use std::collections::{HashMap, HashSet};

fn main() {
    let raw_input = include_str!("input.txt");
    let mut distances: HashMap<String, HashMap<String, i32>> = HashMap::new();
    for line in raw_input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let city_a = parts[0];
        let city_b = parts[2];
        let distance_number = parts[4].parse::<i32>().unwrap_or(0);

        distances
            .entry(city_a.to_string())
            .or_insert_with(HashMap::new)
            .insert(city_b.to_string(), distance_number);

        distances
            .entry(city_b.to_string())
            .or_insert_with(HashMap::new)
            .insert(city_a.to_string(), distance_number);
    }

    let mut min_dist = i32::MAX;
    let total_cities = distances.len();
    for start_city in distances.keys() {
        let mut visited = HashSet::new();
        visited.insert(start_city.clone());

        cities_visited(
            start_city,
            &mut visited,
            0,
            total_cities,
            &distances,
            &mut min_dist,
        );
    }
    println!("{}", min_dist)
}

fn cities_visited(
    city: &str,
    visited: &mut HashSet<String>,
    total_distance: i32,
    total_cities: usize,
    distances: &HashMap<String, HashMap<String, i32>>,
    min_dist: &mut i32,
) {
    if visited.len() == total_cities {
        if total_distance < *min_dist {
            *min_dist = total_distance
        }
        return;
    }

    if let Some(neighbors) = distances.get(city) {
        for (neighbor, &dist) in neighbors {
            if !visited.contains(neighbor) {
                visited.insert(neighbor.clone());

                cities_visited(
                    neighbor,
                    visited,
                    total_distance + dist,
                    total_cities,
                    distances,
                    min_dist,
                );

                visited.remove(neighbor);
            }
        }
    }
}
