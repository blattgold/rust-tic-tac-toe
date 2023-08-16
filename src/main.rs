fn main() {
    println!("Welcome to Tic-Tac-Toe");
    game();
}

#[derive(PartialEq, Copy, Clone, Debug)]
enum Field {
    Empty,
    X,
    O
}

fn game() {
    let mut board: Vec<Field> = init_board();
    let mut xturn = true; // true = X's turne, false = O's turn

    'gloop: loop {
        draw_board(&board);
        get_input(&mut board, xturn);

        if let Some(winner) = check_winner(&board) {
            match winner {
                Field::X | Field::O => {
                    println!("{:?} won!", winner); 
                    draw_board(&board); 
                    break 'gloop;
                },
                _ => {
                    println!("game over! the game ended in a stalemate."); 
                    draw_board(&board);
                    break 'gloop;
                },
            }
        }

        xturn = !xturn;
    }
}

fn init_board() -> Vec<Field> {
    Vec::from([Field::Empty, Field::Empty, Field::Empty,
               Field::Empty, Field::Empty, Field::Empty,
               Field::Empty, Field::Empty, Field::Empty])
}

fn draw_board(board: &Vec<Field>) {
    for (i, f) in board.iter().enumerate() {
        match f {
            Field::X => print!("X"),
            Field::O => print!("O"),
            _        => print!("{}", (i + 1).to_string())
        }
        if i == 2 || i == 5 || i == 8 {
            println!();
        }
    }     
    println!();
}

fn get_input(board: &mut Vec<Field>, xturn: bool) {
    println!("{}'s turn!", if xturn {"X"} else {"O"});
    println!("Which place would you like to place on to?(1 - 9):");
    let mut inp = String::new();

    loop {
        if let Err(_) = std::io::stdin().read_line(&mut inp) {
            inp.clear();
            continue;
        }
        inp.pop(); // remove trailing newline

        match inp.parse::<usize>() {
            Ok(x @ 1..=9) if board[x -1] == Field::Empty => {
                if xturn {board[x -1] = Field::X;}
                else {board[x -1] = Field::O;} 
                break;
            },
            _ => {
                println!("invalid input, must be a number (1 - 9) of a space that isn't occupied!"); 
                inp.clear();
            },
        }
    }

    println!();
}
/// checks whether someone has won or a stalemate occured.
/// takes a reference to the board, returns a Field Option.
/// None means game is not over yet.
/// Some(Field::Empty) means stalemate
/// Some(Field::X) and Some(Field::O) means that either X or O won
fn check_winner(board: &Vec<Field>) -> Option<Field> {
    let winstates: [[usize; 3]; 8] = [[0, 3, 6], [1, 4, 7], [2, 5, 8],
                                     [0, 1, 2], [3, 4, 5], [6, 7, 8],
                                     [0, 4, 8], [2, 4, 6]];

    // check winner
    for combo in winstates {
        if board[combo[0]] == Field::X && board[combo[1]] == Field::X && board[combo[2]] == Field::X ||
           board[combo[0]] == Field::O && board[combo[1]] == Field::O && board[combo[2]] == Field::O {
               return Some(board[combo[0]]);
           }
    }

    // check if full, aka stalemate
    if board.iter().all(|&x| x != Field::Empty) {
        return Some(Field::Empty);
    }

    // if you got here, it means the game isn't over
    None
}
