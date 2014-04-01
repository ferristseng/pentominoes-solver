#[allow(dead_code)];
#[allow(unused_mut)];

extern mod extra;

use std::vec;
use parse::parseFile;
use pentomino::Pentomino;
use dlink::DancingMatrix;
use solve::generateSolutionMatrix;

mod dlink;
mod solve;
mod parse;
mod pentomino;

/// Finds the Board in a vector of pentominoes, and removes it
/// and returns it.
#[inline]
pub fn discoverBoard(pentominoes: &mut ~[Pentomino]) -> Pentomino {
  let mut index = 0;
  let mut max = 0;
  
  for (i, pentomino) in pentominoes.iter().enumerate() {
    if (pentomino.area() > max) { 
      max = pentomino.area();
      index = i; 
    }
  }

  pentominoes.remove(index)
}

pub fn solve(solutionMatrix: &mut DancingMatrix) {
  if (solutionMatrix.root().right() == 0) {
    println("Solution Found!");
  }

  let mut minCol = 0;
  
  for (col, n) in solutionMatrix.iterHeader() {
    if (minCol == 0) { minCol = col }
    if (n.len() < solutionMatrix.get(0, minCol).len() && col != 0) { minCol = col }
  }

  //println(solutionMatrix.to_str());
  
  solutionMatrix.deleteCol(minCol);

  //println(solutionMatrix.to_str());

  //solve(solutionMatrix);

  solutionMatrix.undeleteCol(minCol);
}

fn main() {
  //let path = Path::new("test/pentominoes6x10.txt");
  let path = Path::new("test/trivial.txt");
  //let path = Path::new("test/pentominoes3x20.txt");
  //let path = Path::new("test/pentominoes8x8_middle_missing.txt");
  let mut pentominoes = parseFile(&path);
  let board = discoverBoard(&mut pentominoes);

  // Begin Solving
  let mut solutionMatrix = generateSolutionMatrix(&board, &pentominoes);
  let cols = pentominoes.len() + board.area();

  println(format!("{:u}x{:u} Board", board.dimX, board.dimY));
  println("Rows in solutionMatrix: " + solutionMatrix.len().to_str());
  println("Cols in solutionMatrix: " + cols.to_str());

  solve(&mut solutionMatrix);
}
