#[allow(dead_code)];

extern mod extra;

use std::vec;
use std::hashmap::HashSet;
use extra::bitv::Bitv;
use parse::parseFile;
use pentomino::Pentomino;

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

pub fn generateSolutionMatrix(board: &Pentomino, 
                              pentominoes: &~[Pentomino]) -> ~[Bitv] {
  // HashSet is used to keep track of uniques
  let mut matrixSet: HashSet<~[u8]> = HashSet::new();
  let mut matrixVec: ~[Bitv] = ~[];
  let offset = pentominoes.len();
  let cols = offset + board.area();

  for (i, piece) in pentominoes.iter().enumerate() {
    let mut count: uint = 0;

    for (x, y, _) in board.coordinates() {
      for rotation in piece.rotations() {
        for permutation in rotation.reflections() {
          if (board.canPlace(&permutation, x, y)) {
            let mut row = Bitv::new(cols, false); 

            row.set(i, true);

            for (x0, y0, _) in permutation.filled() {
              row.set(board.getIndex(x0 + x, y0 + y) + offset, true);
            }

            if (matrixSet.insert(row.to_bytes())) { 
              matrixVec.push(row);
              count += 1;
            }
          }
        }
      }
    }

    println(piece.to_str());
    println(format!("{:u} placements", count));
  }

  matrixVec
}

fn main() {
  //let path = Path::new("test/pentominoes6x10.txt");
  //let path = Path::new("test/trivial.txt");
  //let path = Path::new("test/pentominoes3x20.txt");
  let path = Path::new("test/pentominoes8x8_middle_missing.txt");
  let mut pentominoes = parseFile(&path);
  let board = discoverBoard(&mut pentominoes);

  // Begin Solving
  let matrixVec = generateSolutionMatrix(&board, &pentominoes);
  let cols = pentominoes.len() + board.area();
  let mut markedRows = Bitv::new(matrixVec.len(), false);
  let mut markedCols = Bitv::new(cols, false);
  let mut solutionVec: ~[uint] = vec::with_capacity(pentominoes.len());
  let mut solutions: uint = 0;

  println(format!("{:u}x{:u} Board", board.dimX, board.dimY));
  println("Rows in matrixVec: " + matrixVec.len().to_str());
  println("Cols in matrixVec: " + cols.to_str());
  println("Expected empty squares in solution: " + 
          (board.area() - board.size()).to_str());
  
  /*
  solveSolutionMatrix(&board, &matrixVec, &mut markedRows, &mut markedCols, 
                      pentominoes.len(), board.area() - board.size(), 
                      &mut solutionVec, &mut solutions, 0);
  */

  println("Solutions: " + solutions.to_str());
}
