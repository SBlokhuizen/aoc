use grid::*;
use std::collections::HashSet;
use std::thread;
use std::time::Duration;
use std::{char, fs};

fn find_cursor(grid: &Grid<char>) -> (usize, usize) {
    // assume square grid
    let grid_size = grid.size().0;
    for i in 0..grid_size {
        for j in 0..grid_size {
            let cursor = grid[(i, j)];
            if cursor == '^' || cursor == '>' || cursor == 'v' || cursor == '<' {
                return (i, j);
            }
        }
    }
    (0, 0)
}
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

fn step(grid: &mut Grid<char>, visited: &mut HashSet<(usize, usize)>) -> bool {
    let cursor = find_cursor(grid);
    let new_cursor: (usize, usize);
    let size = grid.size().0;
    visited.insert(cursor);
    if grid[cursor] == '^' {
        if cursor.0 == 0 {
            return false;
        }
        new_cursor = (cursor.0 - 1, cursor.1);
        if grid[new_cursor] == '#' {
            grid[cursor] = '>'
        } else {
            grid[cursor] = '.';
            grid[new_cursor] = '^';
        }
    } else if grid[cursor] == '>' {
        if cursor.1 == size - 1 {
            return false;
        }
        new_cursor = (cursor.0, cursor.1 + 1);
        if grid[new_cursor] == '#' {
            grid[cursor] = 'v'
        } else {
            grid[cursor] = '.';
            grid[new_cursor] = '>';
        }
    } else if grid[cursor] == 'v' {
        if cursor.0 == size - 1 {
            return false;
        }
        new_cursor = (cursor.0 + 1, cursor.1);
        if grid[new_cursor] == '#' {
            grid[cursor] = '<'
        } else {
            grid[cursor] = '.';
            grid[new_cursor] = 'v';
        }
    } else if grid[cursor] == '<' {
        if cursor.1 == 0 {
            return false;
        }
        new_cursor = (cursor.0, cursor.1 - 1);
        if grid[new_cursor] == '#' {
            grid[cursor] = '^'
        } else {
            grid[cursor] = '.';
            grid[new_cursor] = '<';
        }
    }
    //thread::sleep(Duration::from_millis(100)); // 100 milliseconds = 0.1 seconds
    //print_grid(&grid);
    true
}
fn print_grid(grid: &Grid<char>) {
    for row in grid.iter_rows() {
        for &ch in row {
            print!("{}", ch); // Print each character
        }
        println!(); // Newline after each row
    }
    println!(); // Newline after each row
}
fn main() {
    let file_name = "../input/day6.txt";
    let data = fs::read_to_string(file_name).expect("Unable to read file");
    let mut grid = fill_grid(data);
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut cont = true;
    while cont == true {
        cont = step(&mut grid, &mut visited);
    }

    println!("{}", visited.len());
}
