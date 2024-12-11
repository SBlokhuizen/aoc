use std::collections::HashMap;
use std::fs;

fn blink(stones: &mut HashMap<String, u64>) {
    let mut new_stones: HashMap<String, u64> = HashMap::new();
    for (key, value) in stones.iter() {
        if key == "0" {
            let current_new_value = new_stones.get("1").unwrap_or(&0);

            new_stones.insert("1".to_string(), current_new_value + value);
        } else if key.len() % 2 == 0 {
            let stone_size = key.len() / 2;
            let first_stone = key[..stone_size]
                .parse::<u64>()
                .map(|n| n.to_string())
                .unwrap_or_default();
            let second_stone = key[stone_size..]
                .parse::<u64>()
                .map(|n| n.to_string())
                .unwrap_or_default();

            if let Some(curr_first_stone) = new_stones.get(&first_stone) {
                new_stones.insert(first_stone.to_string(), *curr_first_stone + value);
            } else {
                new_stones.insert(first_stone.to_string(), *value);
            }

            if let Some(curr_second_stone) = new_stones.get(&second_stone) {
                new_stones.insert(second_stone.to_string(), *curr_second_stone + value);
            } else {
                new_stones.insert(second_stone.to_string(), *value);
            }
        } else {
            let num: u64 = key.parse().expect("Failed to parse string to u64");
            let new_key = (num * 2024).to_string();
            let current_new_value = new_stones.get(&new_key).unwrap_or(&0);
            new_stones.insert(new_key, current_new_value + value);
        }
    }
    *stones = new_stones;
}

fn solve(data: &String) {
    let mut stones: HashMap<String, u64> = HashMap::new();
    let init_stones: Vec<String> = data.split_whitespace().map(String::from).collect();

    for stone in &init_stones {
        stones.insert(stone.to_string(), 1);
    }

    for _ in 0..25 {
        blink(&mut stones);
    }
    println!("{}", stones.values().sum::<u64>());
    for _ in 25..75 {
        blink(&mut stones);
    }
    println!("{}", stones.values().sum::<u64>());
}
fn main() {
    let file_name = "../input/day11.txt";
    let data = fs::read_to_string(file_name).expect("Unable to read file");
    solve(&data);
}
