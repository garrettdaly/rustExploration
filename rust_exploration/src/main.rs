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
  let (sx,sy) = find_start(&lines).expect("You didn't specify a start point!");
  let (ex,ey) = find_end(&lines).expect("You didn't specify an end point!");

  let m = lines.iter().map(|l| l.chars().collect::<Vec<char>>()).collect();

  recurse(Maze {grid: &m, sx: &sx, sy: &sy, ex: &ex, ey: &ey});
}

fn recurse(maze: Maze) {
    let adj = find_adj(&maze);
}

fn find_adj(maze: &Maze) -> Vec<(usize, usize)> {
    let adj_inds: Vec<(usize, usize)> = vec![(*maze.sx, *maze.sy - 1),
                                            (*maze.sx + 1, *maze.sy),
                                            (*maze.sx, *maze.sy + 1),
                                            (*maze.sx - 1, *maze.sy)];

    let mut vec = Vec::new();

    for (x,y) in adj_inds {
        if maze.grid[y][x] == 'o' {
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
                return Some((x,y));
            }
        }
    }
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
