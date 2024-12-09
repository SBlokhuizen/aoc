use std::fs;

fn expand(data: &String) -> Vec<char> {
    let mut expansion: Vec<char> = Default::default();
    let mut is_block = true;
    let mut num: u64;
    let mut id = 0;
    for ch in data.trim().chars() {
        num = (ch as u64) - ('0' as u64);
        if is_block {
            for _ in 0..num {
                let id_str = id.to_string();
                for ch in id_str.chars() {
                    expansion.push(ch);
                }
            }
            is_block = false;
            id += 1;
        } else {
            for _ in 0..num {
                expansion.push('.');
            }
            is_block = true;
        }
    }
    expansion
}

fn compress(expansion: &mut Vec<char>) {
    let mut front_ptr: usize = 0;
    let mut back_ptr: usize = expansion.len() - 1;
    while front_ptr < back_ptr {
        while expansion[front_ptr] != '.' {
            front_ptr += 1;
        }
        while expansion[back_ptr] == '.' {
            back_ptr -= 1;
        }
        expansion.swap(front_ptr, back_ptr);
    }
    expansion.swap(front_ptr, back_ptr);
    //println!("{:?}", expansion);
    //println!("{} {}", expansion[front_ptr], expansion[back_ptr]);
}

fn part1(data: &String) {
    let mut expansion = expand(data);
    println!("{:?}", expansion);
    compress(&mut expansion);
    println!("{:?}", expansion);
    let mut id = 0;
    let mut num: u64;
    let mut total = 0;
    for &ch in expansion.iter() {
        if ch != '.' {
            num = (ch as u64) - ('0' as u64);
            //println!("{} {}", num, id);
            total += num * id;
            id += 1;
        }
    }

    println!("{}", total);
}

fn main() {
    let file_name = "../test_input/day9.txt";
    let data = fs::read_to_string(file_name).expect("Unable to read file");
    part1(&data);
}
