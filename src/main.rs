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

    print_field(&arr);
    println!(
        "Where to make your next move? ({})",
        get_legal_moves_string(&arr)
    );
    let mut action = String::new();

    stdin()
        .read_line(&mut action)
        .expect("no correct number entered");

    let position: usize = action.trim().parse().unwrap();

    arr[position] = Cell::X;

    print_field(&arr);
}

fn get_legal_moves_string(field: &[Cell; 9]) -> String {
    let moves = get_legal_moves(&field)
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
