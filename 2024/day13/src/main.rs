use std::fs;

#[derive(Debug)]
struct Direction {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct ClawMachine {
    a: Direction,
    b: Direction,
    prize: Direction,
}

fn fill_claw_machines(data: &String) -> Vec<ClawMachine> {
    let mut claw_machines: Vec<ClawMachine> = Vec::new();
    let lines: Vec<&str> = data
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect();
    for chunk in lines.chunks(3) {
        let button_a_parts: Vec<&str> = chunk[0].split(['+', ',']).map(|s| s.trim()).collect();
        let a_x: i32 = button_a_parts[1].parse().unwrap();
        let a_y: i32 = button_a_parts[3].parse().unwrap();

        let button_b_parts: Vec<&str> = chunk[1].split(['+', ',']).map(|s| s.trim()).collect();
        let b_x: i32 = button_b_parts[1].parse().unwrap();
        let b_y: i32 = button_b_parts[3].parse().unwrap();

        let prize_parts: Vec<&str> = chunk[2].split(['=', ',']).map(|s| s.trim()).collect();
        let prize_x: i32 = prize_parts[1].parse().unwrap();
        let prize_y: i32 = prize_parts[3].parse().unwrap();

        claw_machines.push(ClawMachine {
            a: Direction { x: a_x, y: a_y },
            b: Direction { x: b_x, y: b_y },
            prize: Direction {
                x: prize_x,
                y: prize_y,
            },
        });
    }
    claw_machines
}

fn calc_cost(claw_machine: &ClawMachine) -> i32 {
    let mut min_cost = i32::MAX;

    for num_a in 0..=100 {
        for num_b in 0..=100 {
            if claw_machine.a.x * num_a + claw_machine.b.x * num_b == claw_machine.prize.x
                && claw_machine.a.y * num_a + claw_machine.b.y * num_b == claw_machine.prize.y
            {
                let cost = num_a * 3 + num_b * 1;
                min_cost = min_cost.min(cost);
            }
        }
    }

    if min_cost == i32::MAX {
        return 0;
    }

    min_cost
}
fn part1(claw_machines: &Vec<ClawMachine>) {
    let mut total_cost: i32 = 0;
    for claw_machine in claw_machines {
        total_cost += calc_cost(claw_machine);
    }
    println!("{total_cost}");
}

fn main() {
    let file_name = "../input/day13.txt";
    let data = fs::read_to_string(file_name).expect("Unable to read file");
    let claw_machines = fill_claw_machines(&data);
    part1(&claw_machines);
}
