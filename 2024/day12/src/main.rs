use grid::*;
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

fn flood_fill(
    start: (usize, usize),
    grid: &Grid<char>,
    visited: &mut Grid<bool>,
    area: &mut i32,
    perimeter: &mut i32,
    crop: char,
) {
    if visited[start] || grid[start] != crop {
        return;
    }
    *area += 1;

    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let grid_size = grid.size().0;
    visited[start] = true;

    for (dx, dy) in directions {
        let new_pos = (start.0 as isize + dx, start.1 as isize + dy);
        if new_pos.0 < 0
            || new_pos.1 < 0
            || new_pos.0 >= grid_size as isize
            || new_pos.1 >= grid_size as isize
        {
            *perimeter += 1;
            continue;
        }
        let new_pos = (new_pos.0 as usize, new_pos.1 as usize);
        if grid[new_pos] != crop {
            *perimeter += 1;
        }
        flood_fill(
            (
                (start.0 as isize + dx).try_into().unwrap(),
                (start.1 as isize + dy).try_into().unwrap(),
            ),
            grid,
            visited,
            area,
            perimeter,
            crop,
        );
    }
}

fn part1(grid: &mut Grid<char>) {
    let mut visited = Grid::init(grid.size().0, grid.size().1, false);
    let grid_size = grid.size().0;
    let mut total_price = 0;
    for i in 0..grid_size {
        for j in 0..grid_size {
            if !visited[(i, j)] {
                let crop = grid[(i, j)];
                let mut area = 0;
                let mut perimeter = 0;
                flood_fill((i, j), grid, &mut visited, &mut area, &mut perimeter, crop);
                total_price += area * perimeter;
            }
        }
    }
    println!("{}", total_price);
}

fn main() {
    let file_name = "../input/day12.txt";
    let data = fs::read_to_string(file_name).expect("Unable to read file");
    let mut grid = fill_grid(data);
    part1(&mut grid);
}
