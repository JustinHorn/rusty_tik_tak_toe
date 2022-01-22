use std::fmt;

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
    let arr = [(); 9].map(|_| Cell::EMPTY);

    print_field(arr);
}

fn print_field(field: [Cell; 9]) {
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
