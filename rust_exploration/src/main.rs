use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::thread;
use std::path::Path;

fn lines_from_file<P>(filename: P) -> Vec<String>
    where P: AsRef<Path>,
{
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines().map(|l| l.expect("Could not parse line")).collect()
}

struct Maze<'a> {
    grid: &'a Vec<Vec<char>>,
    sx: &'a usize,
    sy: &'a usize,
    ex: &'a usize,
    ey: &'a usize,
}

fn find_path(lines: &Vec<String>) {
    // Ensure that user has valid start and end
    let (sx,sy) = find_start(&lines).expect("You didn't specify a start point!");
    let (ex,ey) = find_end(&lines).expect("You didn't specify an end point!");

    let mut m = lines.iter().map(|l| l.chars().collect::<Vec<char>>()).collect();
    // Debug maze:
    // println!("{:?}", m);
    // println!("sx={0}, sy={1}", sx, sy);
    // println!("ex={0}, ey={1}", ex, ey);

    // let path = recurse(Maze {grid: &m, sx: &sx, sy: &sy, ex: &ex, ey: &ey});
    recurse(Maze {grid: &m, sx: &sx, sy: &sy, ex: &ex, ey: &ey});
}

fn recurse(maze: Maze) {
    let adj = find_adj(&maze);
    //This works
    if adj.contains(&(*maze.ex, *maze.ey)) {
        println!("Found ending, returning maze");
    }

    //make the new maze for next recursion
    let mut new_grid = &maze.grid;
    let possible_num = new_grid[*maze.sx][*maze.sy];


    //loop over the adjacencies and add to the current value
    for adj_place in adj {
        let (new_sx, new_sy) = adj_place;
        let next_maze = Maze {grid: new_grid, sx: &new_sx, sy: &new_sy, ex: &maze.ex, ey: &maze.ey};
        let child_threads = thread::spawn(move || {
            //thread that calls another recurse
            println!("Spawning recurse thread");
            //recurse(next_maze);

        });
    }

    // Debug adj:
    // println!("{:?}", adj);

}

fn find_adj(maze: &Maze) -> Vec<(usize, usize)> {
    let adj_inds: Vec<(usize, usize)> = vec![(*maze.sx, *maze.sy - 1),
                                            (*maze.sx + 1, *maze.sy),
                                            (*maze.sx, *maze.sy + 1),
                                            (*maze.sx - 1, *maze.sy)];

    let mut vec = Vec::new();

    for (x,y) in adj_inds {
        let c = maze.grid[y][x];
        if c == 'o' || c == 'E' {
            vec.push((x,y));
        }
    }

    println!("{:?}", vec);
    return vec;
}

fn find_start(lines: &Vec<String>) -> Option<(usize, usize)> {
    for (y,line) in lines.iter().enumerate() {
        for (x,c) in line.chars().enumerate() {
            if c == 'S' {
                // Like Haskell's Just
                return Some((x,y));
            }
        }
    }
    // Like Haskell's Nothing
    return None;
}

fn find_end(lines: &Vec<String>) -> Option<(usize, usize)> {
    for (y,line) in lines.iter().enumerate() {
        for (x,c) in line.chars().enumerate() {
            if c == 'E' {
                return Some((x,y));
            }
        }
    }
    return None;
}

fn main() {
    let lines = lines_from_file("maze_config");
    find_path(&lines);

}
