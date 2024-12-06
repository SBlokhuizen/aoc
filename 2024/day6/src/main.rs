use grid::*;
use std::collections::HashSet;
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

fn advance(grid: &mut Grid<char>, cursor: (usize, usize)) -> (bool, (usize, usize)) {
    let mut new_cursor = cursor;
    let size = grid.size().0;
    if grid[cursor] == '^' {
        if cursor.0 == 0 {
            return (false, cursor);
        }
        new_cursor = (cursor.0 - 1, cursor.1);
        if grid[new_cursor] == '#' {
            grid[cursor] = '>';
            return (true, cursor);
        } else {
            grid[cursor] = '.';
            grid[new_cursor] = '^';
        }
    } else if grid[cursor] == '>' {
        if cursor.1 == size - 1 {
            return (false, cursor);
        }
        new_cursor = (cursor.0, cursor.1 + 1);
        if grid[new_cursor] == '#' {
            grid[cursor] = 'v';
            return (true, cursor);
        } else {
            grid[cursor] = '.';
            grid[new_cursor] = '>';
        }
    } else if grid[cursor] == 'v' {
        if cursor.0 == size - 1 {
            return (false, cursor);
        }
        new_cursor = (cursor.0 + 1, cursor.1);
        if grid[new_cursor] == '#' {
            grid[cursor] = '<';
            return (true, cursor);
        } else {
            grid[cursor] = '.';
            grid[new_cursor] = 'v';
        }
    } else if grid[cursor] == '<' {
        if cursor.1 == 0 {
            return (false, cursor);
        }
        new_cursor = (cursor.0, cursor.1 - 1);
        if grid[new_cursor] == '#' {
            grid[cursor] = '^';
            return (true, cursor);
        } else {
            grid[cursor] = '.';
            grid[new_cursor] = '<';
        }
    } else {
        println!("ERROR");
    }
    (true, new_cursor)
}

fn has_loop(grid: &mut Grid<char>) -> bool {
    let mut visited: HashSet<((usize, usize), char)> = HashSet::new();
    let mut cont = true;
    let mut has_loop = false;
    let mut cursor = find_cursor(grid);
    while cont == true && has_loop == false {
        if visited.contains(&(cursor, grid[cursor])) {
            has_loop = true;
        }
        visited.insert((cursor, grid[cursor]));
        (cont, cursor) = advance(grid, cursor);
    }
    has_loop
}

fn check_all_obstructions(grid: &mut Grid<char>, visited: HashSet<(usize, usize)>) -> i32 {
    let mut total_loops = 0;
    let start_pos = find_cursor(&grid);

    for (row, col) in visited.iter() {
        //println!("{row},{col}");
        if grid[(*row, *col)] == '.' {
            grid[(*row, *col)] = '#'
        } else {
            continue;
        }

        let has_loop = has_loop(grid);
        let end_pos = find_cursor(&grid);
        grid[start_pos] = '^';
        grid[end_pos] = '.';
        grid[(*row, *col)] = '.';
        if has_loop {
            //println!("loop");
            total_loops += 1;
        }
    }
    total_loops
}

fn count_visited(mut grid: Grid<char>) -> HashSet<(usize, usize)> {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut cont = true;
    let mut cursor = find_cursor(&grid);
    while cont == true {
        visited.insert(cursor);
        (cont, cursor) = advance(&mut grid, cursor);
    }
    println!("{}", visited.len());
    visited
}
fn main() {
    let file_name = "../input/day6.txt";
    let data = fs::read_to_string(file_name).expect("Unable to read file");
    let mut grid = fill_grid(data);

    let visited = count_visited(grid.clone());
    let total_loops = check_all_obstructions(&mut grid, visited);
    println!("{total_loops}");
}
