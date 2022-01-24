use std::{fmt, io::stdin};

#[derive(PartialEq)]
enum Cell {
    EMPTY,
    X,
    O,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Cell::EMPTY => write!(f, " "),
            Cell::O => write!(f, "O"),
            Cell::X => write!(f, "X"),
        }
    }
}

fn main() {
    let mut arr = [(); 9].map(|_| Cell::EMPTY);

    let mut legal_moves = get_legal_moves(&arr);

    let mut game_state = &Cell::EMPTY;

    while legal_moves.len() > 0 && game_state == &Cell::EMPTY {
        print_field(&arr);

        make_move(&mut arr, legal_moves);
        legal_moves = get_legal_moves(&arr);
        game_state = get_game_state(&arr);
    }
    print_field(&arr);
    if game_state == &Cell::EMPTY {
        println!("Its a draw!");
    } else if game_state == &Cell::X {
        println!("Player X won!");
    } else {
        println!("Player O won!");
    }
}

fn get_game_state(field: &[Cell; 9]) -> &Cell {
    for i in 0..3 {
        if field[i] != Cell::EMPTY && field[i] == field[i + 3] && field[i + 3] == field[i + 6] {
            return &field[i];
        }
        if field[3 * i] != Cell::EMPTY
            && field[3 * i] == field[1 + 3 * i]
            && field[i + 3] == field[2 + 3 * i]
        {
            return &field[3 * i];
        }
    }

    if field[0] != Cell::EMPTY && field[0] == field[4] && field[4] == field[8] {
        return &field[4];
    }
    if field[2] != Cell::EMPTY && field[2] == field[4] && field[4] == field[6] {
        return &field[4];
    }

    return &Cell::EMPTY;
}

fn make_move(field: &mut [Cell; 9], legal_moves: Vec<usize>) {
    let mut action = String::new();
    println!(
        "Where to make your next move? ({})",
        legal_moves_to_string(&legal_moves)
    );
    stdin()
        .read_line(&mut action)
        .expect("no correct number entered");

    let position: usize = match action.trim().parse() {
        Ok(v) => v,
        Err(_) => 11,
    };

    if field.len() > position && field[position] == Cell::EMPTY {
        field[position] = if legal_moves.len() % 2 == 1 {
            Cell::X
        } else {
            Cell::O
        };
    } else {
        make_move(field, legal_moves);
    }
}

fn legal_moves_to_string(legal_moves: &Vec<usize>) -> String {
    let moves = legal_moves
        .into_iter()
        .map(|i| i.to_string() + ",")
        .collect::<String>();
    let len = moves.len();
    if len > 0 {
        moves[..len - 1].to_string()
    } else {
        moves.to_string()
    }
}

fn get_legal_moves(field: &[Cell; 9]) -> Vec<usize> {
    let mut index_list: Vec<usize> = Vec::new();
    for i in 0..9 {
        if field[i] == Cell::EMPTY {
            index_list.append(&mut vec![i]);
        }
    }
    return index_list;
}

fn print_field(field: &[Cell; 9]) {
    for i in 0..9 {
        if i % 3 == 0 {
            println!("");
            println!("-------");
            print!("|")
        }
        print!("{}|", field[i]);
    }
    println!("");
    println!("-------");
}
