
use std::convert::TryInto;

extern crate rand;
use rand::Rng;

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
    cells: Vec<Cell>,
    origin: (usize, usize),
    current: (usize, usize)
}

pub fn new_maze(rows: usize, columns: usize) -> Maze {
    return Maze {
        rows,
        columns,
        cells: vec![Cell{north: false, south: false, east: false, west: false};
                    (rows*columns).try_into().unwrap()],
        origin: (0,0),
        current: (0,0)
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
        self.draw_horizontal();
        for y in 0..self.rows {
            line.push('|');
            for x in 0..self.columns {
                line.push(' ');
                if self.cell_at(x,y).east {
                    line.push(' ');
                } else {
                    line.push('|');
                }
            }
            println!("{}", line);
            line.clear();
            line.push('+');
            for x in 0..self.columns {
                if self.cell_at(x,y).south {
                    line.push(' ');
                } else {
                    line.push('-');
                }
                line.push('+');
            }
            println!("{}", line);
            line.clear();
        }
        self.draw_horizontal();
    }
    
    fn draw_horizontal(&self) {
        let mut line = String::new();
        line.push_str("+-");
        for _x in 1..self.columns {
            line.push_str("--");
        }
        line.push('+');
        println!("{}", line);
    }

    pub fn cell_at(&self, x: usize, y: usize) -> Cell {
        return self.cells[y*self.columns+x];
    }

    fn open_cell_east(&mut self, x: usize, y: usize) {
        self.cells[y*self.columns+x].east = true;
        self.cells[y*self.columns+x+1].west = true;
    }

    fn open_cell_north(&mut self, x: usize, y: usize) {
        self.cells[y*self.columns+x].north = true;
        self.cells[(y-1)*self.columns+x].south = true;
    }

    pub fn populate_binary_tree(&mut self) {
        let mut rng = rand::thread_rng();
        let mut flip_coin = || {
            return rng.gen_range(0..2) == 1
        };
        for y in (0..self.rows).rev() {
            for x in (0..self.columns).rev() {
                if flip_coin() {
                    if y > 0 {
                        self.open_cell_north(x, y);
                    }
                } else {
                    if x < self.columns - 1 {
                        self.open_cell_east(x, y);
                    }
                }
            }
        }
    }

}

fn main() {
    let mut maze = new_maze(15, 15);
    maze.populate_binary_tree();
    maze.print();
}
