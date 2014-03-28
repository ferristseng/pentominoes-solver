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

/// This is a backtracking algorithm 
/// by Donald Knuth solving a generalized version 
/// of the Pentominoes puzzle, the exact cover problem.
pub fn solveSolutionMatrix(board: &Pentomino,
                           matrix: &~[~[int]],
                           markedRows: &mut ~[bool],
                           markedCols: &mut ~[bool], 
                           offset: uint,
                           target: uint,
                           solutionVec: &mut ~[uint],
                           numSolu: &mut uint) {
  // Check if we have a solution!
  let mut empty = 0;
  let mut filled = 0;

  for b in markedCols.iter() { 
    if (!b) { empty += 1 } else { filled += 1 } }
  
  if (empty == target) {
    *numSolu += 1;
    // Debug
    println("--New Solution--");
    for i in range(0, solutionVec.len()) {
      println(format!("{:?}", matrix[solutionVec[i]]));
    }
    return;
  } else if (filled == markedCols.len() - target) {
    return;
  }

  // Choose a column in the matrix that is unmarked 
  for i in board.range() {
    let i = i + offset; 

    if (markedCols[i]) { continue }

    markedCols[i] = true;

    // For each row in the column that contains a 1 
    for (j, row) in matrix.iter().enumerate() {
      if (row[i] == 1 && !markedRows[j]) {
        let mut newCols = ~[];
        let mut newRows = ~[];

        for (k, val) in row.iter().enumerate() {

          // Erase all cols where the row contains a 1
          if (*val == 1) { 
            if (!markedCols[k]) {
              newCols.push(k);
              markedCols[k] = true; 
            }
            // Erase all rows where the col contains a 1
            for (l, row0) in matrix.iter().enumerate() {
              if (row0[k] == 1 && !markedRows[l]) { 
                markedRows[l] = true;
                newRows.push(l);
              }
            }
          }
        }

        // Solve the subproblem
        solutionVec.push(j);
        
        solveSolutionMatrix(board, matrix, markedRows, markedCols, 
                            offset, target, solutionVec, numSolu);
        
        // Restore to explore a different solution
        solutionVec.pop();

        for c in newCols.move_iter() {
          markedCols[c] = false;
        }

        for r in newRows.move_iter() {
          markedRows[r] = false;
        }
      }
    }

    markedCols[i] = false;
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
  let mut solutionVec = vec::with_capacity(pentominoes.len());
  let mut solutions = 0;
    
  println("Rows in matrixVec: " + matrixVec.len().to_str());
  println("Cols in matrixVec: " + matrixVec[0].len().to_str());
  println("Expected empty squares in solution: " + 
          (board.area() - board.size()).to_str());
  
  solveSolutionMatrix(&board, &matrixVec, &mut markedRows, &mut markedCols, 
                      pentominoes.len(), board.area() - board.size(), 
                      &mut solutionVec, &mut solutions);

  println("Solutions: " + solutions.to_str());
}
