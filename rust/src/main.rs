
use std::convert::TryInto;

extern crate rand;
use rand::Rng;

extern crate clap;
use clap::{Arg, App};

#[derive(Clone, Copy)]
pub struct Cell {
    north: bool,
    south: bool,
    east: bool,
    west: bool
}

pub struct Maze {
    rows: usize,
    columns: usize,
    cells: Vec<Cell>
}

pub fn new_maze(rows: usize, columns: usize) -> Maze {
    return Maze {
        rows,
        columns,
        cells: vec![Cell{north: false, south: false, east: false, west: false};
                    (rows*columns).try_into().unwrap()]
    }
}

impl Maze {

    // print: Print the maze to the ASCII terminal.
    //   1. Print the top line.
    //   2. For each row:
    //      a. Print the left wall.
    //      b. For each Column:
    //         1. Print a space.
    //         2. If the cell is closed to the right, print a wall.
    //         3. Else, print a space.
    //      c. End of line.
    //      d. For each Column:
    //         1. If the cell is closed to the bottom, print a wall.
    //         2. Else, print a space.
    pub fn print(&self) {
        let mut line = String::new();
        line.push_str("     ");
        for x in 0..self.columns {
            line.push_str(&format!("{:>3} ", x+1));
        }
        println!("{}", line);
        line.clear();
        self.draw_horizontal();
        for y in 0..self.rows {
            line.push_str(&format!("{:>3} ", y+1));
            line.push('|');
            for x in 0..self.columns {
                line.push_str("   ");
                if self.cell_at(x,y).east {
                    line.push(' ');
                } else {
                    line.push('|');
                }
            }
            println!("{}", line);
            line.clear();
            if y < self.rows - 1 {
                line.push_str("    +");
                for x in 0..self.columns {
                    if self.cell_at(x,y).south {
                        line.push_str("   ");
                    } else {
                        line.push_str("---");
                    }
                    line.push('+');
                }
                println!("{}", line);
                line.clear();
            }
        }
        self.draw_horizontal();
    }
    
    fn draw_horizontal(&self) {
        let mut line = String::new();
        line.push_str("    +-");
        for _x in 1..self.columns {
            line.push_str("--+-");
        }
        line.push_str("--+");
        println!("{}", line);
    }

    pub fn cell_at(&self, x: usize, y: usize) -> Cell {
        return self.cells[y*self.columns+x];
    }

    fn open_cell_east(&mut self, x: usize, y: usize) {
        if x < self.columns - 1 {
            self.cells[y*self.columns+x].east = true;
            self.cells[y*self.columns+x+1].west = true;
        }
    }

    fn open_cell_north(&mut self, x: usize, y: usize) {
        if y > 0 {
            self.cells[y*self.columns+x].north = true;
            self.cells[(y-1)*self.columns+x].south = true;
        }
    }

    // fn open_cell_west(&mut self, x: usize, y: usize) {
    //     self.cells[y*self.columns+x].west = true;
    //     if x > 0 {
    //         self.cells[y*self.columns+x-1].east = true;
    //     }
    // }

    // fn open_cell_south(&mut self, x: usize, y:usize) {
    //     self.cells[y*self.columns+x].south = true;
    //     if y < self.rows - 1 {
    //         self.cells[(y+1)*self.columns+x].north = true;
    //     }
    // }

    pub fn populate_binary_tree(&mut self) {
        let mut rng = rand::thread_rng();
        let mut flip_coin = || {return rng.gen_range(0..2) == 1};
        for y in (0..self.rows).rev() {
            for x in 0..self.columns {
                if y == 0 {
                    self.open_cell_east(x, y);
                } else {
                    if x == self.columns - 1 {
                        self.open_cell_north(x, y);
                    } else {
                        if flip_coin() {
                            self.open_cell_north(x, y);
                        } else {
                            self.open_cell_east(x, y);
                        }
                    }
                }
            }
        }
    }

    fn select_sidewinder_run(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut result = vec![(x,y)];
        let mut x = x;
        let y = y;
        while self.cell_at(x,y).west {
            println!("adding cell to the west");
            x = x - 1;
            result.push((x, y));
        }
        return result;
    }

    fn open_sidewinder_north(&mut self, x: usize, y: usize) {
        let run = self.select_sidewinder_run(x, y);
        let mut rng = rand::thread_rng();
        let cell = run[rng.gen_range(0..run.len())];
        self.open_cell_north(cell.0, cell.1);
    }

    pub fn populate_sidewinder(&mut self) {
        let mut rng = rand::thread_rng();
        let mut flip_coin = || {return rng.gen_range(0..2) == 1};
        for y in (0..self.rows).rev() {
            for x in 0..self.columns {
                if y == 0 {
                    self.open_cell_east(x, y);
                } else {
                    if x == self.columns - 1 {
                        self.open_sidewinder_north(x, y);
                    } else {
                        if flip_coin() {
                            self.open_sidewinder_north(x, y);
                        } else {
                            self.open_cell_east(x, y);
                        }
                    }
                }
            }
        }
    }
}

fn parse_number(name: &str, value: Option<&str>, default: usize) -> usize {
    match value {
        None => default,
        Some(s) => {
            match s.parse::<usize>() {
                Ok(n) => n,
                Err(_) => {
                    println!("{}={}. Not a positive integer.", name, s);
                    default
                }
            }
        }
    }
}

fn main() {
    let matches = App::new("mazes")
        .version("1.0")
        .author("Brian P King")
        .about("Generates random mazes.")
        .arg(Arg::with_name("columns")
             .short("c")
             .long("columns")
             .value_name("COLUMNS")
             .help("Specify the number of columns in the maze (default 15).")
             .takes_value(true))
        .arg(Arg::with_name("rows")
             .short("r")
             .long("rows")
             .value_name("ROWS")
             .help("Specify the number of rows in the maze (default 15).")
             .takes_value(true))
        .arg(Arg::with_name("sidewinder")
             .long("sidewinder")
             .help("Use the Sidewinder algorithm instead of the default Binary Tree.")
             .takes_value(false))
        .get_matches();
    let columns = parse_number("columns", matches.value_of("columns"), 15);
    let rows = parse_number("rows", matches.value_of("rows"), 15);
    let algorithm = {
        if matches.is_present("sidewinder") {
            println!("selected the sidewinder algorithm.");
            "sidewinder"
        } else {
            println!("selected the binary algorithm.");
            "binary"
        }
    };
    let mut maze = new_maze(rows, columns);
    if algorithm == "binary" {
        maze.populate_binary_tree();
    } else {
        maze.populate_sidewinder();
    }
    maze.print();
}
