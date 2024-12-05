use std::collections::HashMap;
use std::fs;

fn fill_hashmap(rules: String) -> HashMap<i32, Vec<i32>> {
    // hash map contains number and number before it
    let mut lookup_table: HashMap<i32, Vec<i32>> = HashMap::new();
    for line in rules.lines() {
        let parts: Vec<i32> = line.split('|').map(|s| s.parse().unwrap()).collect();
        add_relationship(&mut lookup_table, parts[0], parts[1]);
    }
    lookup_table
}

fn add_relationship(lookup_table: &mut HashMap<i32, Vec<i32>>, before: i32, after: i32) {
    lookup_table.entry(after).or_default().push(before);
}

fn sort_invalid(numbers: &mut Vec<i32>, lookup_table: &HashMap<i32, Vec<i32>>) -> Vec<i32> {
    for i in 0..numbers.len() {
        for j in i + 1..numbers.len() {
            if let Some(before) = lookup_table.get(&numbers[i]) {
                if before.contains(&numbers[j]) {
                    numbers.swap(i, j)
                }
            }
        }
    }
    numbers.to_vec()
}

fn parse(rules: String, updates: String) -> (i32, i32) {
    let lookup_table = fill_hashmap(rules);
    let mut total1 = 0;
    let mut total2 = 0;
    for line in updates.lines() {
        let mut numbers: Vec<i32> = line.split(',').map(|s| s.parse().unwrap()).collect();
        let mut is_valid = true;
        for i in 0..numbers.len() {
            for j in i + 1..numbers.len() {
                if let Some(before) = lookup_table.get(&numbers[i]) {
                    if before.contains(&numbers[j]) {
                        is_valid = false;
                    }
                }
            }
        }
        if is_valid {
            total1 += numbers[numbers.len() / 2];
        } else {
            let numbers_sorted = sort_invalid(&mut numbers, &lookup_table);
            total2 += numbers_sorted[numbers_sorted.len() / 2];
        }
    }
    (total1, total2)
}

fn main() {
    let file_name = "../input/day5.txt";
    let data = fs::read_to_string(file_name).expect("Unable to read file");
    let mut parts = data.splitn(2, "\n\n");
    let rules = parts.next().unwrap_or("").trim();
    let updates = parts.next().unwrap_or("").trim();

    let (total1, total2) = parse(rules.to_string(), updates.to_string());
    println!("{total1}\n{total2}");
}
