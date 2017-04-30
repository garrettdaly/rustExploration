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

fn main() {
    let lines = lines_from_file("maze_config");
    find_path(&lines);
    for line in lines {
        println!("{0}", line);
    }

}
