pub fn annotate(board: &[&str]) -> Vec<String> {
    let rows = board.len();
    if rows == 0 {
        return vec![];
    }
    let cols = board[0].len();

    let mut grid: Vec<Vec<char>> = board
        .iter()
        .map(|row| row.chars().collect())
        .collect();

    let directions = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),           (0, 1),
        (1, -1),  (1, 0),  (1, 1),
    ];

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == '*' {
                continue;
            }
            let mut count = 0;
            for (dr, dc) in &directions {
                let nr = r as isize + dr;
                let nc = c as isize + dc;
                if nr >= 0 && nr < rows as isize && nc >= 0 && nc < cols as isize {
                    if grid[nr as usize][nc as usize] == '*' {
                        count += 1;
                    }
                }
            }
            if count > 0 {
                grid[r][c] = std::char::from_digit(count, 10).unwrap();
            }
        }
    }

    grid.into_iter()
        .map(|row| row.into_iter().collect())
        .collect()
}
