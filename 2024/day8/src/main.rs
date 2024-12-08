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

fn find_antennas(grid: &Grid<char>) -> HashMap<char, Vec<(i32, i32)>> {
    let grid_size = grid.size().0;
    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    for i in 0..grid_size {
        for j in 0..grid_size {
            if grid[(i, j)] != '.' {
                antennas
                    .entry(grid[(i, j)])
                    .or_insert_with(Vec::new)
                    .push((i.try_into().unwrap(), j.try_into().unwrap()))
            }
        }
    }
    antennas
}

fn is_valid_antinode(antinode: (i32, i32), grid_size: usize) -> bool {
    let grid_size = grid_size as i32;
    antinode.0 >= 0 && antinode.1 >= 0 && antinode.0 < grid_size && antinode.1 < grid_size
}

fn create_antinode(pos: (i32, i32), diff: (i32, i32), is_pos: bool) -> (i32, i32) {
    let antinode: (i32, i32);
    if is_pos {
        antinode = (pos.0 + diff.0, pos.1 + diff.1);
    } else {
        antinode = (pos.0 - diff.0, pos.1 - diff.1);
    }
    antinode
}

fn solve(grid: &Grid<char>, is_p2: bool) {
    let antennas = find_antennas(grid);
    let mut antinodes: HashSet<(i32, i32)> = Default::default();
    let grid_size = grid.size().0;
    for antenna in antennas.iter() {
        let pos_antenna = antenna.1;
        for i in 0..pos_antenna.len() {
            for j in i + 1..pos_antenna.len() {
                if i != j {
                    let diff = (
                        pos_antenna[i].0 - pos_antenna[j].0,
                        pos_antenna[i].1 - pos_antenna[j].1,
                    );

                    if is_p2 {
                        let mut pos_antinode = pos_antenna[i];
                        while is_valid_antinode(pos_antinode, grid_size) {
                            antinodes.insert(pos_antinode);
                            pos_antinode = create_antinode(pos_antinode, diff, true);
                        }

                        let mut neg_antinode = pos_antenna[j];
                        while is_valid_antinode(neg_antinode, grid_size) {
                            antinodes.insert(neg_antinode);
                            neg_antinode = create_antinode(neg_antinode, diff, false);
                        }
                    } else {
                        let pos_antinode = create_antinode(pos_antenna[i], diff, true);
                        if is_valid_antinode(pos_antinode, grid_size) {
                            antinodes.insert(pos_antinode);
                        }
                        let neg_antinode = create_antinode(pos_antenna[j], diff, false);
                        if is_valid_antinode(neg_antinode, grid_size) {
                            antinodes.insert(neg_antinode);
                        }
                    }
                }
            }
        }
    }
    println!("{}", antinodes.len());
}

fn main() {
    let file_name = "../input/day8.txt";
    let data = fs::read_to_string(file_name).expect("Unable to read file");
    let grid = fill_grid(data);
    solve(&grid, false);
    solve(&grid, true);
}
