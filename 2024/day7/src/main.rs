use std::fs;
fn split_line(line: &str) -> (i32, Vec<i32>) {
    let mut parts = line.split(':');
    let result: i32 = parts
        .next()
        .expect("No ':' found in input")
        .trim()
        .parse()
        .expect("First number is not a valid i32");

    let args: Vec<i32> = parts
        .next()
        .expect("No numbers found after ':'")
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<i32>().expect("Invalid number in input"))
        .collect();
    (result, args)
}
fn part1(data: String) {
    let mut total = 0;
    for line in data.lines() {
        let (result, args) = split_line(line);
        let num_op = args.len() - 1;
        //println!("{} {:?}", result, args);
        let num_options = 1 << num_op;

        for i in 0..num_options {
            let mut tmp_result = args[0];

            for j in 0..num_op {
                if (i >> j) & 1 == 0 {
                    tmp_result = tmp_result + args[j + 1];
                } else {
                    tmp_result = tmp_result * args[j + 1];
                }
            }
            if tmp_result == result {
                total += result;
                break;
            }

            //println!("{}", tmp_result);
        }
    }
    println!("{total}");
}

fn main() {
    let file_name = "../test_input/day7.txt";
    let data = fs::read_to_string(file_name).expect("Unable to read file");
    part1(data);
}
