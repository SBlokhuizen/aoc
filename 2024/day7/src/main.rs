use std::{fs, u64};
fn split_line(line: &str) -> (u64, Vec<u64>) {
    let mut parts = line.split(':');
    let result: u64 = parts
        .next()
        .expect("No ':' found in input")
        .trim()
        .parse()
        .expect("First number is not a valid u64");

    let args: Vec<u64> = parts
        .next()
        .expect("No numbers found after ':'")
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<u64>().expect("Invalid number in input"))
        .collect();
    (result, args)
}

fn solve(data: &String, is_p2: bool) {
    let mut total = 0;
    for line in data.lines() {
        let (result, args) = split_line(line);
        let num_op = args.len() - 1;
        let mut num_pos_op: i32 = 2;
        if is_p2 {
            num_pos_op = 3;
        }
        let num_options = num_pos_op.pow(num_op as u32);

        for i in 0..num_options {
            let mut tmp_result = args[0];
            let mut current = i;

            for j in 0..num_op {
                let operator = current % num_pos_op;
                current /= num_pos_op;

                match operator {
                    0 => tmp_result = tmp_result + args[j + 1],
                    1 => tmp_result = tmp_result * args[j + 1],
                    2 => {
                        tmp_result = {
                            let combined = format!("{}{}", tmp_result, args[j + 1]);
                            combined.parse::<u64>().unwrap()
                        }
                    }
                    _ => unreachable!(),
                }
            }
            if tmp_result == result {
                total += result;
                break;
            }
        }
    }
    println!("{total}");
}

fn main() {
    let file_name = "../input/day7.txt";
    let data = fs::read_to_string(file_name).expect("Unable to read file");
    solve(&data, false);
    solve(&data, true);
}
