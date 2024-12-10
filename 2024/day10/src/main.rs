use grid::*;
use std::collections::HashSet;
use std::fs;

fn fill_grid(data: String) -> Grid<u32> {
    let grid_size = data.lines().count();
    let mut grid: Grid<u32> = Grid::new(grid_size, grid_size);
    for (i, line) in data.lines().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            grid[(i, j)] = ch as u32 - '0' as u32;
        }
    }
    grid
}

fn part1(grid: &Grid<u32>) {
    let trailheads = find_trailheads(grid);
    let mut peaks: HashSet<(usize, usize)> = Default::default();
    let mut total: usize = 0;

    for trailhead in trailheads {
        peaks.clear();
        follow_trail(grid, trailhead, &mut peaks);
        total += peaks.len();
    }
    println!("{}", total);
}
fn follow_trail(grid: &Grid<u32>, trailhead: (usize, usize), peaks: &mut HashSet<(usize, usize)>) {
    let (row, col) = trailhead;
    let height = grid[trailhead];

    if height == 9 {
        peaks.insert(trailhead);
        return;
    }

    let next_height = height + 1;
    let grid_size = grid.size().0;

    if row > 0 && grid[(row - 1, col)] == next_height {
        follow_trail(grid, (row - 1, col), peaks);
    }

    if row + 1 < grid_size && grid[(row + 1, col)] == next_height {
        follow_trail(grid, (row + 1, col), peaks);
    }

    if col > 0 && grid[(row, col - 1)] == next_height {
        follow_trail(grid, (row, col - 1), peaks);
    }

    if col + 1 < grid_size && grid[(row, col + 1)] == next_height {
        follow_trail(grid, (row, col + 1), peaks);
    }
}

fn find_trailheads(grid: &Grid<u32>) -> Vec<(usize, usize)> {
    let mut trailheads: Vec<(usize, usize)> = vec![];
    let grid_size = grid.size().0;
    for row in 0..grid_size {
        for col in 0..grid_size {
            if grid[(row, col)] == 0 {
                trailheads.push((row, col));
            }
        }
    }
    trailheads
}

fn main() {
    let file_name = "../input/day10.txt";
    let data = fs::read_to_string(file_name).expect("Unable to read file");
    let grid = fill_grid(data);
    part1(&grid);
}
