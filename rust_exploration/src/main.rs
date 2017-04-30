use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;

fn lines_from_file<P>(filename: P) -> Vec<String>
    where P: AsRef<Path>,
{
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines().map(|l| l.expect("Could not parse line")).collect()
}

fn find_path(lines: &Vec<String>) {
  for line in lines {
      println!("{0}", line);
  }
}

fn find_start(lines: &Vec<String>) -> (usize, usize) {
    for (y,line) in lines.iter().enumerate() {
        for (x,c) in line.chars().enumerate() {
            if c == 'S' {
                return (x,y);
            }
        }
    }
    return (0,0);
}

fn main() {
    let lines = lines_from_file("maze_config");
    // find_path(&lines);
    let (x,y) = find_start(&lines);
}
