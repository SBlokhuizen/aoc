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

fn solve(grid: &Grid<u32>, is_p2: bool) {
    let trailheads = find_trailheads(grid);
    let mut peaks: Vec<(usize, usize)> = Default::default();
    let mut peaks_uniq: HashSet<(usize, usize)> = Default::default();
    let mut total: usize = 0;

    for trailhead in trailheads {
        follow_trail(grid, trailhead, &mut peaks);
        if !is_p2 {
            for peak in &peaks {
                peaks_uniq.insert(*peak);
            }
            total += peaks_uniq.len();
            peaks_uniq.clear();
        } else {
            total += peaks.len();
        }
        peaks.clear();
    }
    println!("{}", total);
}
fn follow_trail(grid: &Grid<u32>, trailhead: (usize, usize), peaks: &mut Vec<(usize, usize)>) {
    let (row, col) = trailhead;
    let height = grid[trailhead];

    if height == 9 {
        peaks.push(trailhead);
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
    solve(&grid, false);
    solve(&grid, true);
}
