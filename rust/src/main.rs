use clap::{App, Arg};
//use cursive::views::{BoxedView, NamedView, TextView};

mod maze {

    use rand::Rng;
    use std::convert::TryInto;

    #[derive(Clone, Copy)]
    pub struct Cell {
        north: bool,
        south: bool,
        east: bool,
        west: bool,
    }

    impl Cell {
        pub fn new() -> Self {
            Cell {
                north: false,
                south: false,
                east: false,
                west: false,
            }
        }
    }

    pub struct Maze {
        rows: usize,
        columns: usize,
        cells: Vec<Cell>,
    }

    impl Maze {
        pub fn new(rows: usize, columns: usize) -> Self {
            return Maze {
                rows,
                columns,
                cells: vec![Cell::new(); (rows * columns).try_into().unwrap()],
            };
        }

        // print: Print the maze to a String.
        pub fn to_string(&self) -> String {
            let mut maze_content = String::new();
            let mut line = String::new();
            line.push_str("     ");
            for x in 0..self.columns {
                line.push_str(&format!("{:>3} ", x + 1));
            }
            maze_content.push_str(&format!("{}\n", line));
            line.clear();
            maze_content.push_str(&format!("{}\n", self.draw_horizontal()));
            for y in 0..self.rows {
                line.push_str(&format!("{:>3} ", y + 1));
                line.push('|');
                for x in 0..self.columns {
                    line.push_str("   ");
                    if self.cell_at(x, y).east {
                        line.push(' ');
                    } else {
                        line.push('|');
                    }
                }
                maze_content.push_str(&format!("{}\n", line));
                line.clear();
                if y < self.rows - 1 {
                    line.push_str("    +");
                    for x in 0..self.columns {
                        if self.cell_at(x, y).south {
                            line.push_str("   ");
                        } else {
                            line.push_str("---");
                        }
                        line.push('+');
                    }
                    maze_content.push_str(&format!("{}\n", line));
                    line.clear();
                }
            }
            maze_content.push_str(&format!("{}\n", self.draw_horizontal()));
            maze_content
        }

        fn draw_horizontal(&self) -> String {
            let mut line = String::new();
            line.push_str("    +-");
            for _x in 1..self.columns {
                line.push_str("--+-");
            }
            line.push_str("--+");
            line
        }

        pub fn cell_at(&self, x: usize, y: usize) -> Cell {
            return self.cells[y * self.columns + x];
        }

        fn open_cell_east(&mut self, x: usize, y: usize) {
            if x < self.columns - 1 {
                self.cells[y * self.columns + x].east = true;
                self.cells[y * self.columns + x + 1].west = true;
            }
        }

        fn open_cell_north(&mut self, x: usize, y: usize) {
            if y > 0 {
                self.cells[y * self.columns + x].north = true;
                self.cells[(y - 1) * self.columns + x].south = true;
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

        // MAZE ALGORITHM: Binary Tree
        pub fn populate_binary_tree(&mut self) {
            let mut rng = rand::thread_rng();
            let mut flip_coin = || return rng.gen_range(0..2) == 1;
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

        // MAZE ALGORITHM: Sidewinder
        fn select_sidewinder_run(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
            let mut result = vec![(x, y)];
            let mut x = x;
            let y = y;
            while self.cell_at(x, y).west {
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
            let mut flip_coin = || return rng.gen_range(0..2) == 1;
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
}

// pub struct MazeView {
//     buffer: String
// }

// impl MazeView {
//     pub fn new() -> Self {
//         MazeView { String::new() }
//     }
// }

// impl View for MazeView {
//     pub fn layout(&mut self, size XY<usize>) {
//     }
//     pub fn draw(&self, printer: &Printer) {
//         self.clear();
//         printer.print(0, self.buffer);
//     }
// }

// *********************************************************************************************
// MAIN
// *********************************************************************************************

fn parse_number(name: &str, value: Option<&str>, default: usize) -> usize {
    match value {
        None => default,
        Some(s) => match s.parse::<usize>() {
            Ok(n) => n,
            Err(_) => {
                println!("{}={}. Not a positive integer.", name, s);
                default
            }
        },
    }
}

// TODO: Turn Maze.print into a Display trait.
// TODO: Allow clearing a maze.

fn main() {
    let matches = App::new("mazes")
        .version("1.0")
        .author("Brian P King")
        .about("Generates random mazes.")
        .arg(
            Arg::with_name("columns")
                .short("c")
                .long("columns")
                .value_name("COLUMNS")
                .help("Specify the number of columns in the maze (default 15).")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("rows")
                .short("r")
                .long("rows")
                .value_name("ROWS")
                .help("Specify the number of rows in the maze (default 15).")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("draw")
                .long("draw")
                .help("Draw the maze to the screen (do not use the UI).")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("binary")
                .long("binary")
                .help("Use the Binary Tree algorithm (default)")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("sidewinder")
                .long("sidewinder")
                .help("Use the Sidewinder algorithm instead of the default Binary Tree.")
                .takes_value(false),
        )
        .get_matches();
    if matches.is_present("draw") {
        let columns = parse_number("columns", matches.value_of("columns"), 15);
        let rows = parse_number("rows", matches.value_of("rows"), 15);
        let algorithm = {
            if matches.is_present("sidewinder") {
                "sidewinder"
            } else {
                "binary"
            }
        };
        let mut maze = maze::Maze::new(rows, columns);
        if algorithm == "binary" {
            maze.populate_binary_tree();
        } else {
            maze.populate_sidewinder();
        }
        print!("{}", maze.to_string());
    } else {
        println!("{}", String::from("Not yet implemented."));
        // let mut siv = cursive::default();
        // siv.add_global_callback('q', |s| s.quit());
        // siv.add_fullscreen_layer(NamedView::new("box",
        //                                         BoxView::with_full_screen(NamedView::new("maze",
        //                                                                                  TextView::empty()))));
        // let mut maze = maze::Maze::new(width, height);
        // if algorithm == "binary" {
        //     maze.populate_binary_tree();
        // } else {
        //     maze.populate_sidewinder();
        // }
        // siv.call_on_name("maze", |view: &mut TextView| {
        //     view.set_content(maze.to_string());
        // });
        // siv.run();
    }
}

/* WITH CURSIVE!
 * For this to work, the TUI will need to drive the maze, not the surrounding program.  MazeView
 * will need to contain its own maze. The size of the maze will fit the available space within
 * the screen. The algorithm selection will be driven by TUI dialogs.  Should we have a maze on
 * startup, or have a print option to drive redrawing the maze?  Redrawing would indicate
 * repopulating. We also want to be able to draw other characters at specific locations. How is
 * that managed? I need to wrap this up with various elements, such as a player and enemy.
 *
 * The maze needs to be made more homogenous, so we do not need to know what type of maze we are
 * populating.
 */
