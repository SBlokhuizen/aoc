use std::fs;

fn part1(first_col: &mut Vec<i32>, second_col: &mut Vec<i32>) {
    first_col.sort();
    second_col.sort();

    let result: Vec<i32> = first_col
        .iter()
        .zip(second_col.iter())
        .map(|(a, b)| (a - b).abs())
        .collect();
    println!("{:?}", result.iter().sum::<i32>());
}

fn part2(first_col: Vec<i32>, second_col: Vec<i32>) {
    let mut similarty_vec: Vec<i32> = Vec::with_capacity(first_col.len());
    for num1 in first_col.iter() {
        let mut curr_similarity = 0;
        for num2 in second_col.iter() {
            if num1 == num2 {
                curr_similarity += num1
            }
        }
        similarty_vec.push(curr_similarity);
    }

    println!("{:?}", similarty_vec.iter().sum::<i32>());
}

fn main() {
    let file_name = "list.txt";
    let data = fs::read_to_string(file_name).expect("Unable to read file");
    let num_lines = data.lines().count();
    let mut first_col: Vec<i32> = Vec::with_capacity(num_lines);
    let mut second_col: Vec<i32> = Vec::with_capacity(num_lines);

    for line in data.lines() {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .filter_map(|word| word.parse::<i32>().ok())
            .collect();
        let [num1, num2] = numbers.as_slice() else {
            todo! {}
        };
        first_col.push(*num1);
        second_col.push(*num2);
    }
    part1(&mut first_col, &mut second_col);
    part2(first_col, second_col);
}
