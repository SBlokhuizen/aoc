use std::fs;

static WIDTH: i32 = 101;
static HEIGHT: i32 = 103;

#[derive(Debug)]
struct Robot {
    position: (i32, i32),
    velocity: (i32, i32),
}

fn fill_robots(data: &String) -> Vec<Robot> {
    let mut robots: Vec<Robot> = Vec::new();
    for line in data.lines() {
        let parts: Vec<i32> = line
            .split(|c: char| c == 'p' || c == '=' || c == ',' || c == 'v' || c == ' ')
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();
        let robot = Robot {
            position: (parts[0], parts[1]),
            velocity: (parts[2], parts[3]),
        };
        robots.push(robot);
    }
    robots
}

fn wait_a_sec(robots: &mut Vec<Robot>) {
    for robot in robots {
        robot.position.0 += robot.velocity.0;
        robot.position.1 += robot.velocity.1;

        if robot.position.0 < 0 {
            robot.position.0 += WIDTH
        }
        if robot.position.0 >= WIDTH {
            robot.position.0 -= WIDTH
        }
        if robot.position.1 < 0 {
            robot.position.1 += HEIGHT
        }
        if robot.position.1 >= HEIGHT {
            robot.position.1 -= HEIGHT
        }
    }
}

fn count_robots(robots: &Vec<Robot>) {
    let mut quandrants = [0, 0, 0, 0];
    for robot in robots {
        if robot.position.0 < WIDTH / 2 && robot.position.1 < HEIGHT / 2 {
            quandrants[0] += 1;
        } else if robot.position.0 > WIDTH / 2 && robot.position.1 < HEIGHT / 2 {
            quandrants[1] += 1;
        } else if robot.position.0 > WIDTH / 2 && robot.position.1 > HEIGHT / 2 {
            quandrants[2] += 1;
        } else if robot.position.0 < WIDTH / 2 && robot.position.1 > HEIGHT / 2 {
            quandrants[3] += 1;
        }
    }
    let mut safety_factor = 1;
    for quandrant in quandrants {
        safety_factor *= quandrant;
    }
    println!("{safety_factor}");
}
fn part1(robots: &mut Vec<Robot>) {
    for _ in 0..100 {
        wait_a_sec(robots);
    }
    count_robots(robots);
}

fn main() {
    let file_name = "../input/day14.txt";
    let data = fs::read_to_string(file_name).expect("Unable to read file");
    let mut robots = fill_robots(&data);
    part1(&mut robots);
}
