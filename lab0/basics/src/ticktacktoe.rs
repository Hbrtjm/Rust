use std::io::{stdin,stdout,Write};

#[derive(PartialEq)]
enum GameState
{
    ONGOING,
    XWON,
    OWON,
}

struct Coordinate
{
    x: i8,
    y: i8,
}

fn coordinate_correct(x: i8) -> bool
{
    return x >= 0 && x <= 2;
}

impl Coordinate {
    fn from_input(input: &str) -> Option<Coordinate> 
    {
        // Extract the coordinates from the form (*x*,*y*) -> Array of form [*x*,*y*], where * mean
        // any amount of spaces
        let cleaned = input.trim().trim_matches(|c| c == '(' || c == ')');
        let coords: Vec<&str> = cleaned.split(',').map(|s| s.trim()).collect();

        if coords.len() == 2 
        {
            // So that ( 1 , 1 ) is equivalent to (1,1)
            if let (Ok(x), Ok(y)) = (coords[0].trim().parse::<i8>(), coords[1].trim().parse::<i8>()) { 
                if coordinate_correct(x) && coordinate_correct(y) 
                {
                    return Some(Coordinate { x, y });
                }
            }
        }
        None
    }
}

// This could probably be done better
fn check_win(board: [char; 9]) -> GameState 
{
    for i in 0..3
    {
        // Check board horizontally...
        if board[3*i] == board[3*i+1] && board[3*i+1] == board[3*i+2]
        {
            if board[3*i] == 'X'
            {
                return GameState::XWON;
            }
            else if board[3*i] == 'O'
            {
                return GameState::OWON;
            }
        }
        // Check vertically...
        else if board[i] == board[i + 3] && board[i + 3] == board[i + 6]
        {
            if board[i] == 'X'
            {
                return GameState::XWON;
            }
            else if board[i] == 'O'
            {
                return GameState::OWON;
            }
        }
    }
    // Check the top to bottom left to right diagonal...
    if board[0] == board[4] && board[4] == board[8] {
        if board[0] == 'X' {
            return GameState::XWON;
        } else if board[0] == 'O' {
            return GameState::OWON;
        }
    }
    // ... And the bottom to top left to right diagonal
    if board[2] == board[4] && board[4] == board[6] {
        if board[2] == 'X' {
            return GameState::XWON;
        } else if board[2] == 'O' {
            return GameState::OWON;
        }
    }

    // Nothing found - show must go on!
    return GameState::ONGOING
}


fn print_board(board: [char;9])
{
    println!("------------");
    for i in 0..3
    {
        println!("| {} | {} | {} |",board[3*i], board[3*i+1], board[3*i+2]); 
        println!("------------");
    }
}

fn update_board(value: char,coord: Coordinate, board: &mut [char; 9]) -> bool
{
    if board[3*(coord.x as usize)+(coord.y as usize)] == 'X' || board[3*(coord.x as usize)+(coord.y as usize)] == 'O'
    {
        return false;
    }
    board[3*(coord.x as usize)+(coord.y as usize)] = value;
    return true;
}

fn initialize_board(board: &mut [char; 9])
{
    *board = [' '; 9];
    for i in 0..9 {
        board[i] = char::from_digit((i + 1) as u32, 10).unwrap(); // Convert number to char
    }
}

fn main()
{
    let mut board: [char; 9] = ['x'; 9];
    initialize_board(&mut board); 
    print_board(board);
    loop 
    {
    let mut game_timer = 0;
    let mut current_player;
    while check_win(board) == GameState::ONGOING
    {
        current_player = if game_timer % 2 == 0 { 'X' } else { 'O' };

        loop
        {
        let mut s = String::new();
        print!("Player {}, enter coordinates (x,y): ", current_player);
        let _=stdout().flush();
        stdin().read_line(&mut s).expect("Did not enter a correct string");
        match Coordinate::from_input(&s) {
            Some(coord) => {
                if update_board(current_player, coord, &mut board)
                {
                    print_board(board);
                    game_timer += 1;
                    break;
                }
                else
                {
                    println!("That space is already occupied");
                }
            }
            None => println!("Invalid input! Please enter coordinates in the form (x,y)."),
            }
        }
    }
    let state = check_win(board);
    match state 
    {
        GameState::XWON => println!("X has won the game!"),
        GameState::OWON => println!("O has won the game!"),
    _ => {}
    }
    // Added '_' because the compiler was screaming it's being overriten 
    let mut _question = String::new();
    loop 
    {
        _question = "".to_string();
        print!("Do you want to play again? (1 - yes, 0 - no): ");
        let _ = stdout().flush();
        stdin().read_line(&mut _question).expect("Failed to read input");
        let answer = _question.trim();
        if answer == "0" || answer == "1" {
            _question = String::from(answer);
            break;
        }
    }
    if _question == "0"
    {
        break
    }
    initialize_board(&mut board); 
    }
}
