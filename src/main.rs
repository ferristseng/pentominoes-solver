#[allow(dead_code)];

use std::vec;
use std::hashmap::HashSet;
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
                              pentominoes: &~[Pentomino]) -> ~[~[int]] {
  let mut matrixSet: HashSet<~[int]> = HashSet::new();
  let offset = pentominoes.len();
  let cols = offset + board.area();

  for (i, piece) in pentominoes.iter().enumerate() {
    for (x, y, _) in board.coordinates() {
      for rotation in piece.rotations() {
        for permutation in rotation.reflections() {
          if (board.canPlace(&permutation, x, y)) {
            let mut row = vec::from_elem(cols, 0);

            row[i] = 1;

            for (x0, y0, _) in permutation.coordinates() {
              row[board.getIndex(x0 + x, y0 + y) + offset] = 1;
            }

            matrixSet.insert(row);
          }
        }
      }
    }
  }

  matrixSet.move_iter().to_owned_vec()
}

pub fn solveSolutionMatrix(board: &Pentomino,
                           pentominoes: &~[Pentomino],
                           matrix: ~[~[int]],
                           markedRows: &mut ~[bool],
                           markedCols: &mut ~[bool]) {
  let offset = pentominoes.len();

  for i in board.range() {

  }
}

fn main() {
  //let path = Path::new("test/pentominoes6x10.txt");
  //let path = Path::new("test/trivial.txt");
  //let path = Path::new("test/pentominoes3x20.txt");
  let path = Path::new("test/pentominoes8x8_middle_missing.txt");
  let mut pentominoes = parseFile(&path);
  let board = discoverBoard(&mut pentominoes);
  let matrixVec = generateSolutionMatrix(&board, &pentominoes);
  let mut markedRows = vec::from_elem(matrixVec.len(), false);
  let mut markedCols = vec::from_elem(matrixVec[0].len(), false);
    
  println("Rows in matrixVec: " + matrixVec.len().to_str());
  println("Cols in matrixVec: " + matrixVec[0].len().to_str());
  println("Expected empty squares in solution: " + 
          (board.area() - board.size()).to_str());

}
