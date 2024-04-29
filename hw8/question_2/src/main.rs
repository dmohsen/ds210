type Board = Vec<Vec<bool>>;

fn create_board(size: usize, live_cells: &[(usize, usize)]) -> Board {
    let mut board = vec![vec![false; size]; size];
    for &(x, y) in live_cells {
        board[x][y] = true;
    }
    board
}

fn count_neighbors(board: &Board, x: usize, y: usize) -> usize {
    let mut count = 0;
    let size = board.len();
    for i in 0..3 {
        for j in 0..3 {
            if i == 1 && j == 1 {
                continue; 
            }
            let ni = (x + i + size - 1) % size; 
            let nj = (y + j + size - 1) % size;
            count += board[ni][nj] as usize;
        }
    }
    count
}

fn evolve(board: &Board) -> Board {
    let size = board.len();
    let mut new_board = board.clone();
    for x in 0..size {
        for y in 0..size {
            let neighbors = count_neighbors(board, x, y);
            new_board[x][y] = matches!((board[x][y], neighbors), (true, 2) | (_, 3));
        }
    }
    new_board
}

#[test]
fn test_count_neighbors() {
    let initial_cells = vec![(0, 1), (1, 2), (2, 0), (2, 1), (2, 2)];
    let board = create_board(16, &initial_cells);
    assert_eq!(count_neighbors(&board, 0, 0), 1); 
    assert_eq!(count_neighbors(&board, 2, 1), 3);
}

fn main() {
    let initial_cells = vec![(0, 1), (1, 2), (2, 0), (2, 1), (2, 2)];
    let mut board = create_board(16, &initial_cells);

    println!("Initial board:");
    for row in &board {
        for &cell in row {
            print!("{}", if cell { '◼' } else { '◻' });
        }
        println!();
    }

    for _ in 0..10 {
        board = evolve(&board);
    }

    println!("Board after 10 generations:");
    for row in &board {
        for &cell in row {
            print!("{}", if cell { '◼' } else { '◻' });
        }
        println!();
    }
}

