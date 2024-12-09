use std::fs;

fn expand(data: &String) -> Vec<String> {
    let mut expansion: Vec<String> = Default::default();
    let mut is_block = true;
    let mut num: u64;
    let mut id = 0;
    for ch in data.trim().chars() {
        num = (ch as u64) - ('0' as u64);
        if is_block {
            for _ in 0..num {
                let id_str = id.to_string();
                expansion.push(id_str);
            }
            is_block = false;
            id += 1;
        } else {
            for _ in 0..num {
                expansion.push(".".to_string());
            }
            is_block = true;
        }
    }
    expansion
}

fn compress(expansion: &mut Vec<String>) {
    let mut front_ptr: usize = 0;
    let mut back_ptr: usize = expansion.len() - 1;
    while front_ptr < back_ptr {
        while expansion[front_ptr] != "." {
            front_ptr += 1;
        }
        while expansion[back_ptr] == "." {
            back_ptr -= 1;
        }
        expansion.swap(front_ptr, back_ptr);
    }
    expansion.swap(front_ptr, back_ptr);
}

fn calc_checksum(expansion: &mut Vec<String>) -> u64 {
    let mut id = 0;
    let mut num: u64;
    let mut checksum = 0;
    for string in expansion.iter() {
        if string != "." {
            num = string
                .parse::<u64>()
                .expect("Failed to parse string as u64");
            checksum += num * id;
            id += 1;
        }
    }
    checksum
}

fn part1(data: &String) {
    let mut expansion = expand(data);
    compress(&mut expansion);
    let checksum = calc_checksum(&mut expansion);
    println!("{}", checksum);
}

fn main() {
    let file_name = "../test_input/day9.txt";
    let data = fs::read_to_string(file_name).expect("Unable to read file");
    part1(&data);
}
