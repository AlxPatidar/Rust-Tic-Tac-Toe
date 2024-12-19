use std::io;

const PLAYER_X: char = 'X';
const PLAYER_O: char = 'O';
const BOARD_SIZE: usize = 3;
type Board = [[char; BOARD_SIZE]; BOARD_SIZE];

// initialize board to show
fn initialize_board() -> Board {
    return [[' '; BOARD_SIZE]; BOARD_SIZE];
}
fn print_board(board: &Board) {
    for row in 0..BOARD_SIZE {
        print!(" - - ");
    }
    println!("");
    for row in board {
        for column in row {
            print!("| {} ", column);
        }
        println!("|");
        for _ in 0..BOARD_SIZE {
            print!(" - - ");
        }
        println!("");
    }
}
fn check_winner(current_player: char, board: &Board) -> bool {
    // check all row are same
    for row in 0..BOARD_SIZE {
        if board[row][0] == current_player
            && board[row][1] == current_player
            && board[row][2] == current_player
        {
            return true;
        }
    }
    // check all column are same
    for col in 0..BOARD_SIZE {
        if board[0][col] == current_player
            && board[1][col] == current_player
            && board[2][col] == current_player
        {
            return true;
        }
    }

    if (board[0][0] == current_player
        && board[1][1] == current_player
        && board[2][2] == current_player)
        || (board[0][2] == current_player
            && board[1][1] == current_player
            && board[2][0] == current_player)
    {
        return true;
    }
    return false;
}
fn check_draw(board: &Board) -> bool {
    for row in board {
        for cell in row {
            if *cell == ' ' {
                return false;
            }
        }
    }
    return true;
}
fn get_player_move(current_player: char, board: &Board) -> (usize, usize) {
    loop {
        println!("Player {} (row, col)", current_player);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Please enter");
        let cordinates: Vec<usize> = input
            .split_whitespace()
            .flat_map(str::parse::<usize>)
            .collect();
        if cordinates.len() == 2 {
            let (row, col) = (cordinates[0], cordinates[1]);
            if row < BOARD_SIZE && col < BOARD_SIZE && board[row][col] == ' ' {
                return (row, col);
            }
        }
        println!("Invalid input;");
    }
}
fn play_game() {
    let mut board = initialize_board();
    let mut current_player = PLAYER_X;
    loop {
        println!("Current Board");
        print_board(&board);
        let (row, col) = get_player_move(current_player, &board);
        board[row][col] = current_player;
        if check_winner(current_player, &board) {
            println!("Player {} is the winner", current_player);
            break;
        }
        if check_draw(&board) {
            println!("This game is draw");
            break;
        }
        current_player = if current_player == PLAYER_X {
            PLAYER_O
        } else {
            PLAYER_X
        }
    }
}
fn main() {
    println!("Welcome to tick tock game:");
    play_game();
}
