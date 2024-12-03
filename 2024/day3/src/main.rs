use std::fs;

fn solve(line: &str, is_p2: bool) -> i32 {
    let mut window = String::from("");
    let mut dontwindow = String::from("");
    let mut completed_dontwindow = String::from("do()");
    let mut mult = 0;
    for char in line.chars() {
        //Match mul window
        match char {
            'm' if window.is_empty() => window.push(char),
            'u' if window == "m" => window.push(char),
            'l' if window == "mu" => window.push(char),
            '(' if window == "mul" => window.push(char),
            ',' if window.starts_with("mul(") => window.push(char),
            ')' if window.starts_with("mul(") && window.contains(",") => {
                if let Some(comma_pos) = window.rfind(',') {
                    let first_num: i32 = window[4..comma_pos].parse().expect("Not a valid number");

                    let second_num: i32 =
                        window[comma_pos + 1..].parse().expect("Not a valid number");

                    if completed_dontwindow == "do()" {
                        //println!("do: {first_num} {second_num}");
                        mult += first_num * second_num;
                    } else {
                        //println!("dont: {first_num} {second_num}");
                    }
                }
                window.clear();
            }
            _ => {
                if char.is_numeric() {
                    if window.starts_with("mul(") || window.ends_with(",") {
                        window.push(char);
                    } else {
                        window.clear();
                    }
                } else {
                    window.clear();
                }
            }
        }
        if is_p2 {
            // Match don't window
            match char {
                'd' if dontwindow.is_empty() => dontwindow.push(char),
                'o' if dontwindow == "d" => dontwindow.push(char),
                'n' if dontwindow == "do" => dontwindow.push(char),
                '\'' if dontwindow == "don" => dontwindow.push(char),
                't' if dontwindow == "don'" => dontwindow.push(char),
                '(' if dontwindow == "do" || dontwindow == "don't" => dontwindow.push(char),
                ')' if dontwindow == "do(" || dontwindow == "don't(" => {
                    dontwindow.push(char);
                    completed_dontwindow = dontwindow.clone();
                }
                _ => dontwindow.clear(),
            }
        }
    }
    mult
}

fn main() {
    // No Regex allowed 😡
    let file_name = "../input/day3.txt";
    let data = fs::read_to_string(file_name).expect("Unable to read file");
    let mult = solve(&data, false);
    println!("{mult}");
    let mult = solve(&data, true);
    println!("{mult}");
}
