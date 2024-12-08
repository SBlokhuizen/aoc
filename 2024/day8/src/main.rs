use grid::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn fill_grid(data: String) -> Grid<char> {
    let num_cols = data.lines().count();
    let num_rows = data.lines().next().map(|line| line.len()).unwrap_or(0);
    let mut grid: Grid<char> = Grid::new(num_rows, num_cols);
    for (i, line) in data.lines().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            grid[(i, j)] = ch;
        }
    }
    grid
}

fn find_antennas(grid: &Grid<char>) -> HashMap<char, Vec<(usize, usize)>> {
    let grid_size = grid.size().0;
    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for i in 0..grid_size {
        for j in 0..grid_size {
            if grid[(i, j)] != '.' {
                antennas
                    .entry(grid[(i, j)])
                    .or_insert_with(Vec::new)
                    .push((i, j));
            }
        }
    }
    antennas
}

fn is_valid_antinode(antinode: (i32, i32), grid: &Grid<char>) -> bool {
    let grid_size = grid.size().0 as i32;
    antinode.0 >= 0 && antinode.1 >= 0 && antinode.0 < grid_size && antinode.1 < grid_size
    //&& grid[(antinode.0 as usize, antinode.1 as usize)] == '.'
}

fn part1(grid: &Grid<char>) {
    let antennas = find_antennas(grid);
    let mut antinodes: HashSet<(i32, i32)> = Default::default();
    for antenna in antennas.iter() {
        let antenna_type = antenna.0;
        let pos = antenna.1;
        println!("{antenna_type}");
        for i in 0..pos.len() {
            for j in i + 1..pos.len() {
                if i != j {
                    let diff: (i32, i32) = (
                        pos[i].0 as i32 - pos[j].0 as i32,
                        pos[i].1 as i32 - pos[j].1 as i32,
                    );
                    let antinode0: (i32, i32) =
                        (pos[i].0 as i32 + diff.0, pos[i].1 as i32 + diff.1);
                    let antinode1: (i32, i32) =
                        (pos[j].0 as i32 - diff.0, pos[j].1 as i32 - diff.1);

                    if is_valid_antinode(antinode0, grid) {
                        antinodes.insert(antinode0);
                    }
                    if is_valid_antinode(antinode1, grid) {
                        antinodes.insert(antinode1);
                    }

                    println!(
                        "{:?} {:?} -> {:?} {:?}",
                        pos[i], pos[j], antinode0, antinode1
                    );
                }
            }
        }
    }

    println!("{:?} len={}", antinodes, antinodes.len());
}

fn main() {
    let file_name = "../input/day8.txt";
    let data = fs::read_to_string(file_name).expect("Unable to read file");
    let grid = fill_grid(data);
    part1(&grid);
}
