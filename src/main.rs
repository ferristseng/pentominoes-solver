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

pub fn solve(solutionMatrix: &mut DancingMatrix, depth: uint, solutions: &mut uint) {
  if (solutionMatrix.root().right() == 0) {
    println("Solution Found!");
    *solutions += 1;
    return;
  }

  // Find the column with the minimum number of 1's 
  let mut minCol = 0;
  
  for (col, n) in solutionMatrix.iterHeader() {
    if (minCol == 0) { minCol = col }
    if (n.len() == 0) { return }
    if (n.len() < solutionMatrix.get(0, minCol).len() && col != 0) { minCol = col }
  }

  let mut currentCol;
  let mut currentRow = solutionMatrix.get(0, minCol).down();

  debug!("minCol=({:u}, {:u})", minCol, solutionMatrix.get(0, minCol).len());

  solutionMatrix.coverCol(minCol);

  while (currentRow != 0) {
    currentCol = solutionMatrix.get(currentRow, minCol).right(); 

    while (currentCol != minCol) {
      solutionMatrix.coverCol(currentCol);
      
      currentCol = solutionMatrix.get(currentRow, currentCol).right();
    }

    solve(solutionMatrix, depth + 1, solutions);

    // Undelete 
    currentCol = solutionMatrix.get(currentRow, minCol).left();

    while (currentCol != minCol) {
      solutionMatrix.uncoverCol(currentCol);

      currentCol = solutionMatrix.get(currentRow, currentCol).left();
    }

    currentRow = solutionMatrix.get(currentRow, minCol).down();
  }

  solutionMatrix.uncoverCol(minCol);
}

fn main() {
  let path = Path::new("test/pentominoes6x10.txt");
  //let path = Path::new("test/trivial.txt");
  //let path = Path::new("test/pentominoes3x20.txt");
  //let path = Path::new("test/pentominoes8x8_middle_missing.txt");
  let mut pentominoes = parseFile(&path);
  let board = discoverBoard(&mut pentominoes);

  // Begin Solving
  let mut solutions = 0;
  let mut solutionMatrix = generateSolutionMatrix(&board, &pentominoes);
  let cols = pentominoes.len() + board.area();

  println(format!("{:u}x{:u} Board", board.dimX, board.dimY));
  println("Rows in solutionMatrix: " + solutionMatrix.len().to_str());
  println("Cols in solutionMatrix: " + cols.to_str());

  solve(&mut solutionMatrix, 0, &mut solutions);

  println(format!("Solutions Found: {:u}", solutions));
}
