use std::fs;

fn is_increasing(numbers: &Vec<i32>) -> bool {
    let mut prev_num = numbers[0] - 1;
    for number in numbers.iter() {
        if *number <= prev_num {
            return false;
        }
        prev_num = *number;
    }
    true
}

fn is_decreasing(numbers: &Vec<i32>) -> bool {
    let mut prev_num = numbers[0] + 1;
    for number in numbers.iter() {
        if *number >= prev_num {
            return false;
        }
        prev_num = *number;
    }
    true
}

fn is_diff_max(numbers: &Vec<i32>, max_diff: i32) -> bool {
    let mut prev_num = numbers[0];
    for number in numbers.iter() {
        let abs_diff = (*number - prev_num).abs();
        if abs_diff > max_diff {
            return false;
        }
        prev_num = *number;
    }
    true
}

fn is_safe(numbers: &Vec<i32>) -> bool {
    is_diff_max(numbers, 3) && (is_decreasing(numbers) || is_increasing(numbers))
}

fn is_safe_p2(numbers: &Vec<i32>) -> bool {
    if is_safe(numbers) {
        return true;
    } else {
        for i in 0..numbers.len() {
            let mut numbers_drop = numbers.clone();
            numbers_drop.remove(i);
            if is_safe(&numbers_drop) {
                return true;
            }
        }
    }
    false
}

fn main() {
    let file_name = "../input/day2.txt";
    let data = fs::read_to_string(file_name).expect("Unable to read file");
    let mut total_safe_p2 = 0;
    let mut total_safe_p1 = 0;
    for line in data.lines() {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .filter_map(|word| word.parse::<i32>().ok())
            .collect();

        if is_safe(&numbers) {
            total_safe_p1 += 1;
        }

        if is_safe_p2(&numbers) {
            total_safe_p2 += 1;
        }
    }
    println!("{total_safe_p1}");
    println!("{total_safe_p2}");
}
