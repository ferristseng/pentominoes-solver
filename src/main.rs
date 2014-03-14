#[allow(dead_code)];

use solve::bruteForce;
use parse::{parseFile, discoverBoard};

mod solve;
mod parse;
mod pentomino;

fn main() {
  //let path = Path::new("test/pentominoes6x10.txt");
  //let path = Path::new("test/trivial.txt");
  let path = Path::new("test/pentominoes3x20.txt");
  let mut pentominoes = parseFile(&path);
  let board = discoverBoard(&mut pentominoes);

  println(bruteForce(&board, &mut pentominoes).to_str());
}
