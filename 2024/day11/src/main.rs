use std::fs;
fn blink(stones: &mut Vec<String>) {
    let mut new_elements = Vec::new();

    for (i, stone) in stones.iter_mut().enumerate() {
        if *stone == "0" {
            *stone = "1".to_string();
        } else if stone.len() % 2 == 0 {
            let stone_size = stone.len() / 2;
            let first_stone = stone[..stone_size]
                .parse::<u64>()
                .map(|n| n.to_string())
                .unwrap_or_default();
            let second_stone = stone[stone_size..]
                .parse::<u64>()
                .map(|n| n.to_string())
                .unwrap_or_default();

            *stone = first_stone;
            new_elements.push((i + 1, second_stone));
        } else {
            if let Ok(parsed_number) = stone.parse::<u64>() {
                let result = parsed_number * 2024;
                *stone = result.to_string();
            }
        }
    }

    for (index, element) in new_elements {
        stones.insert(index, element);
    }
    //println!("{:?}", stones);
}
fn part1(data: &String) {
    let mut stones: Vec<String> = data.split_whitespace().map(|s| s.to_string()).collect();
    //println!("{:?}", stones);
    for i in 0..25 {
        println!("{i}");
        blink(&mut stones);
    }
    println!("{}", stones.len());
}
fn main() {
    let file_name = "../input/day11.txt";
    let data = fs::read_to_string(file_name).expect("Unable to read file");
    part1(&data);
}
