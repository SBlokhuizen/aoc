use std::fs;

fn create_matrix(data: &String) -> Vec<Vec<char>> {
    let matrix: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();
    matrix
}

fn get_horizontal(n: usize, matrix: &Vec<Vec<char>>) -> Vec<char> {
    let row = &matrix[n];
    row.to_vec()
}

fn get_vertical(n: usize, matrix: &Vec<Vec<char>>) -> Vec<char> {
    let col: Vec<char> = matrix.iter().map(|row| row[n]).collect();
    col
}

fn get_diag_left(n: usize, matrix: &Vec<Vec<char>>) -> Vec<char> {
    let mut diagonal = Vec::new();
    let grid_size = matrix.len();
    let mut row: usize;
    let mut col: usize;

    if n < grid_size {
        row = n;
        col = 0;
    } else {
        row = grid_size - 1;
        col = n - grid_size + 1;
    }
    loop {
        diagonal.push(matrix[row][col]);

        if row == 0 || col == grid_size - 1 {
            break;
        }
        row -= 1;
        col += 1;
    }

    diagonal
}
fn get_diag_right(n: usize, matrix: &Vec<Vec<char>>) -> Vec<char> {
    let mut diagonal = Vec::new();
    let grid_size = matrix.len();
    let mut row: usize;
    let mut col: usize;

    if n < grid_size {
        row = n;
        col = grid_size - 1;
    } else {
        row = grid_size - 1;
        col = 2 * grid_size - n - 2;
    }

    loop {
        diagonal.push(matrix[row][col]);

        if row == 0 || col == 0 {
            break;
        }
        row -= 1;
        col -= 1;
    }

    diagonal
}

fn part1(matrix: &Vec<Vec<char>>) -> i32 {
    let grid_size = matrix.len();
    let mut total: i32 = 0;
    for i in 0..grid_size {
        let line = get_horizontal(i, &matrix);
        let s: String = line.iter().collect();
        let num_xmas = s.matches("XMAS").count();
        let num_samx = s.matches("SAMX").count();
        total += num_xmas as i32;
        total += num_samx as i32;
    }
    for i in 0..grid_size {
        let line = get_vertical(i, &matrix);
        let s: String = line.iter().collect();
        let num_xmas = s.matches("XMAS").count();
        let num_samx = s.matches("SAMX").count();
        total += num_xmas as i32;
        total += num_samx as i32;
    }
    for i in 0..2 * grid_size - 1 {
        let line = get_diag_left(i, &matrix);
        let s: String = line.iter().collect();
        let num_xmas = s.matches("XMAS").count();
        let num_samx = s.matches("SAMX").count();
        total += num_xmas as i32;
        total += num_samx as i32;
    }
    for i in 0..2 * grid_size - 1 {
        let line = get_diag_right(i, &matrix);
        let s: String = line.iter().collect();
        let num_xmas = s.matches("XMAS").count();
        let num_samx = s.matches("SAMX").count();
        total += num_xmas as i32;
        total += num_samx as i32;
    }
    total
}

fn check_mas(matrix: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    if (matrix[i - 1][j - 1] == 'M'
        && matrix[i + 1][j - 1] == 'M'
        && matrix[i - 1][j + 1] == 'S'
        && matrix[i + 1][j + 1] == 'S')
        || (matrix[i - 1][j - 1] == 'M'
            && matrix[i - 1][j + 1] == 'M'
            && matrix[i + 1][j - 1] == 'S'
            && matrix[i + 1][j + 1] == 'S')
        || (matrix[i + 1][j + 1] == 'M'
            && matrix[i + 1][j - 1] == 'M'
            && matrix[i - 1][j + 1] == 'S'
            && matrix[i - 1][j - 1] == 'S')
        || (matrix[i + 1][j + 1] == 'M'
            && matrix[i - 1][j + 1] == 'M'
            && matrix[i + 1][j - 1] == 'S'
            && matrix[i - 1][j - 1] == 'S')
    {
        return true;
    }
    false
}
fn part2(matrix: &Vec<Vec<char>>) -> i32 {
    let grid_size = matrix.len();
    let mut total = 0;
    for i in 0..grid_size {
        for j in 0..grid_size {
            if matrix[i][j] == 'A' && i > 0 && i < grid_size - 1 && j > 0 && j < grid_size - 1 {
                if check_mas(matrix, i, j) {
                    total += 1
                }
            }
        }
    }
    total
}

fn main() {
    let file_name = "../input/day4.txt";
    let data = fs::read_to_string(file_name).expect("Unable to read file");
    let matrix = create_matrix(&data);
    let count = part1(&matrix);
    println!("{count}");
    let count = part2(&matrix);
    println!("{count}");
}
