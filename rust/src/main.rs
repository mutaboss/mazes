
use std::convert::TryInto;

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

pub fn NewMaze(rows: usize, columns: usize) -> Maze {
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

}

fn main() {
    let maze = NewMaze(15, 15);
    maze.print();
}
