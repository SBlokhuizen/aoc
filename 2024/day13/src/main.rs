use std::fs;

#[derive(Debug)]
struct Direction {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct ClawMachine {
    a: Direction,
    b: Direction,
    prize: Direction,
}

fn fill_claw_machines(data: &String, is_p2: bool) -> Vec<ClawMachine> {
    let mut claw_machines: Vec<ClawMachine> = Vec::new();
    let lines: Vec<&str> = data
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect();
    for chunk in lines.chunks(3) {
        let button_a_parts: Vec<&str> = chunk[0].split(['+', ',']).map(|s| s.trim()).collect();
        let a_x: i64 = button_a_parts[1].parse().unwrap();
        let a_y: i64 = button_a_parts[3].parse().unwrap();

        let button_b_parts: Vec<&str> = chunk[1].split(['+', ',']).map(|s| s.trim()).collect();
        let b_x: i64 = button_b_parts[1].parse().unwrap();
        let b_y: i64 = button_b_parts[3].parse().unwrap();

        let prize_parts: Vec<&str> = chunk[2].split(['=', ',']).map(|s| s.trim()).collect();

        let mut prize_x: i64 = prize_parts[1].parse().unwrap();
        let mut prize_y: i64 = prize_parts[3].parse().unwrap();
        if is_p2 {
            prize_x += 10000000000000;
            prize_y += 10000000000000;
        }

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

fn calc_cost(claw_machine: &ClawMachine) -> i64 {
    let mut min_cost = i64::MAX;

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

    if min_cost == i64::MAX {
        return 0;
    }

    min_cost
}

fn part1(claw_machines: &Vec<ClawMachine>) {
    let mut total_cost: i64 = 0;
    for claw_machine in claw_machines {
        total_cost += calc_cost(claw_machine);
    }
    println!("{total_cost}");
}

fn calc_cost_p2(claw_machine: &ClawMachine) -> i64 {
    let ax = claw_machine.a.x;
    let bx = claw_machine.b.x;
    let px = claw_machine.prize.x;
    let ay = claw_machine.a.y;
    let by = claw_machine.b.y;
    let py = claw_machine.prize.y;

    // Solve: N_a*ax+N_b*bx=px, N_a*ay+N_b*by = py
    let n_a = (by * px - bx * py) / (ax * by - ay * bx);
    let n_b = (ax * py - ay * px) / (ax * by - ay * bx);

    // check if solution is correct
    if n_a * ax + n_b * bx == px && n_a * ay + n_b * by == py {
        return 3 * n_a + n_b;
    } else {
        return 0;
    }
}

fn part2(claw_machines: &Vec<ClawMachine>) {
    let mut total_cost: i64 = 0;
    for claw_machine in claw_machines {
        total_cost += calc_cost_p2(claw_machine);
    }
    println!("{total_cost}");
}

fn main() {
    let file_name = "../input/day13.txt";
    let data = fs::read_to_string(file_name).expect("Unable to read file");
    let claw_machines = fill_claw_machines(&data, false);
    part1(&claw_machines);
    let claw_machines = fill_claw_machines(&data, true);
    part2(&claw_machines);
}
